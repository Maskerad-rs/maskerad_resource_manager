// Copyright 2017-2018 Maskerad Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use std::collections::HashMap;
use std::path::{PathBuf, Path};
use gltf::{Gltf, Glb};
use std::io::{Read, Seek};
use std::io::BufReader;
use lewton::inside_ogg::OggStreamReader;

use imagefmt::Image;
use imagefmt::tga;
use imagefmt::ColFmt;

use maskerad_filesystem::filesystem as maskerad_filesystem;

use maskerad_data_parser::level_description::LevelDescription;
use maskerad_data_parser::gameobject_description::GameObjectDescription;

use properties::PropertiesRegistry;
use resources::ResourceRegistry;
use refcount_registry::RefCountRegistry;

use maskerad_gameobject_model::properties::transform::Transform;

use resource_manager_errors::{ResourceManagerError, ResourceManagerResult};

pub struct ResourceManager {
    //A resources registry
    //A properties registry
    //An allocators registry
    //A refcount registry
    resource_registry: ResourceRegistry,
    properties_registry: PropertiesRegistry,
    refcount_registry: RefCountRegistry,
}

impl ResourceManager {
    fn new() -> Self {
        ResourceManager {
            resource_registry: ResourceRegistry::new(),
            properties_registry: PropertiesRegistry::new(),
            refcount_registry: RefCountRegistry::new(),
        }
    }

    //First step.
    fn read_needed_resources(&self, gameobject_descriptions: &[GameObjectDescription]) -> Vec<PathBuf> {
        let mut vec = Vec::new();

        for gameobject_description in gameobject_descriptions.iter() {

            //MESH
            //a gameobject's mesh has an external resource -> gltf data.
            let mesh_option = gameobject_description.mesh();
            if let Some(ref mesh_desc) = *mesh_option {
                //Don't allow doublons.
                if !vec.contains(&mesh_desc.path().to_path_buf()) {
                    vec.push(mesh_desc.path().to_path_buf());
                }
            }

            //TODO: other resources
        }

        vec
    }

    //Second step.
    fn increment_refcount(&mut self, path_new_resource: &Path) -> ResourceManagerResult<()> {
        if !self.refcount_registry.has_refcount(path_new_resource) {
            //Add the refcount of this resource
            self.refcount_registry.add_refcount(path_new_resource);
        } else {
            //increment the refcount of this resource
            self.refcount_registry.increment_refcount_of(path_new_resource)?
        }
        Ok(())
    }

    //Third step.
    fn decrement_refcount(&mut self, resource_to_decrease: &Path) -> ResourceManagerResult<()> {
        self.refcount_registry.decrement_refcount_of(resource_to_decrease)?;
        Ok(())
    }

    fn unload_resource(&mut self, path: &Path) -> ResourceManagerResult<()> {
        match path.extension() {
            Some(osstr_ext) => {
                match osstr_ext.to_str() {
                    Some(str_ext) => {
                        match str_ext {
                            "ogg" => {
                                self.resource_registry.remove_ogg(path);
                            },
                            "tga" => {
                                self.resource_registry.remove_tga(path);
                            },
                            "gltf" => {
                                self.resource_registry.remove_gltf(path);
                            },
                            _ => {
                                return Err(ResourceManagerError::ResourceError(format!("The data at path {} cannot be unloaded by the engine !", path.display())));
                            }
                        }
                    },
                    None => {
                        return Err(ResourceManagerError::ResourceError(format!("The path {} is not valid unicode !", path.display())));
                    }
                }
            },
            None => {
                return Err(ResourceManagerError::ResourceError(format!("The path {} is not valid !", path.display())));
            }
        }
        Ok(())
    }

    fn load_resource(&mut self, path: &Path) -> ResourceManagerResult<()> {
        let mut reader = maskerad_filesystem::open(path)?;
        match path.extension() {
            Some(osstr_ext) => {
                match osstr_ext.to_str() {
                    Some(str_ext) => {
                        match str_ext {
                            "ogg" => {
                                let ogg_data = OggStreamReader::new(reader)?;
                                self.resource_registry.add_ogg(path, ogg_data);
                            },
                            "tga" => {
                                let tga_data = tga::read(&mut reader, ColFmt::Auto)?;
                                self.resource_registry.add_tga(path, tga_data);
                            },
                            "gltf" => {
                                let gltf_data = Gltf::from_reader(reader)?.validate_completely()?;
                                self.resource_registry.add_gltf(path, gltf_data);
                            },
                            _ => {
                                return Err(ResourceManagerError::ResourceError(format!("The data at path {} cannot be loaded by the engine !", path.display())));
                            }
                        }
                    },
                    None => {
                        return Err(ResourceManagerError::ResourceError(format!("The path {} is not valid unicode !", path.display())));
                    }
                }
            },
            None => {
                return Err(ResourceManagerError::ResourceError(format!("The path {} is not valid !", path.display())));
            }
        }
        Ok(())
    }

    //Doesn't actually "create" the level, we just load all the needed resources for all the objects inside the level.
    pub fn load_level_resources(&mut self, level_description: &LevelDescription) -> ResourceManagerResult<()> {
        /*
        When loading level :
        1 - read all needed resources.
        2 - Increment the ref count of all those resources, and add them if they don't exist.
        3 - then decrement the ref count of each unneeded resources.
        4 - if a ref count drop to 0, unload the resource.
        5 - load all the other resources and place them in the right memory allocator.
        */

        let go_desc = level_description.generate_gameobject_descriptions()?;

        let path_new_resources = self.read_needed_resources(&go_desc);
        for path in path_new_resources.iter() {
            self.increment_refcount(path.as_path())?;
        }

        let resource_to_decrease = self.refcount_registry.keys().filter(|elem| {
            !path_new_resources.contains(elem)
        }).cloned().collect::<Vec<PathBuf>>();
        for resource in resource_to_decrease.iter() {
            self.decrement_refcount(resource.as_path())?;
        }


        let resources_to_unload = self.refcount_registry.iter().filter(|elem| {
            (*elem.1) == 0
        }).map(|elem| {
            elem.0
        }).cloned().collect::<Vec<PathBuf>>();
        for resource in resources_to_unload.iter() {
            self.unload_resource(resource.as_path())?;
        }



        let resources_to_load = self.refcount_registry.iter().filter(|elem| {
            (*elem.1) == 1
        }).map(|elem| {
            elem.0
        }).cloned().collect::<Vec<PathBuf>>();
        for resource in resources_to_load.iter() {
            self.load_resource(resource.as_path())?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod resource_manager_test {
    use super::*;

    #[test]
    fn resource_manager_creation() {
        let resource_manager = ResourceManager::new();
        assert!(resource_manager.properties_registry.is_mesh_empty());
        assert!(resource_manager.properties_registry.is_transform_empty());
        assert!(resource_manager.resource_registry.is_gltf_empty());
    }

    #[test]
    fn resource_manager_load_unload_resource() {
    //Only one copy of the resource.
    }

    #[test]
    fn resource_manager_load_unload_asynchronously_resource() {

    }

    #[test]
    fn resource_manager_post_process_resource() {
        //fine_tuning of the resource after it has been loaded
    }

    #[test]
    fn resource_manager_memory_usage() {
        //should we allocate in our stack allocators ? Or maybe create an object pool ?

        //global resources -> beginning of stack allocator and put a marker.
        //level-tied lifetime -> After the global resources in the stack allocator
        //small lifetime -> ???
        //streamed resource -> ???
    }

    #[test]
    fn resource_manager_composite_resource_and_referential_integrity() {
        //Composite resource -> Model has mesh, anims, skeletons...
        //Referential integrity -> Model has a mesh, which has a skeletons and anims. Skeleton must be loaded before anims...
    }

    #[test]
    fn resource_manager_package_resources_in_one_big_file() {
        //Optional
    }

    #[test]
    fn resource_manager_get_gltf_resource() {

    }

    #[test]
    fn resource_manager_get_ogg_resource() {

    }

    #[test]
    fn resource_manager_get_tga_resource() {

    }

    #[test]
    fn resource_manager_get_flac_resource() {

    }

}