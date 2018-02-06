// Copyright 2017-2018 Maskerad Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use std::path::{PathBuf, Path};
use gltf::Gltf;
use lewton::inside_ogg::OggStreamReader;
use std::rc::Rc;
use imagefmt::tga;
use imagefmt::ColFmt;
use std::collections::hash_map::Iter;
use maskerad_filesystem::filesystem as maskerad_filesystem;
use maskerad_data_parser::level_description::LevelDescription;
use resources::resources_registry::ResourceRegistry;
use resources::refcount_registry::RefCountRegistry;
use resources::resource_manager_errors::{ResourceManagerError, ResourceManagerResult};
use resources::resource_manager_trait::IResourceManager;
use maskerad_memory_allocators::StackAllocator;

pub struct ResourceManager {
    //A resources registry
    //An allocators registry
    //A refcount registry
    stack_allocator: StackAllocator,
    resource_registry: ResourceRegistry,
    refcount_registry: RefCountRegistry,
}

impl IResourceManager for ResourceManager {
    fn load_resource<P>(&mut self, path: P) -> ResourceManagerResult<()> where
        P: AsRef<Path> + Into<PathBuf>
    {
        let mut reader = maskerad_filesystem::open(path.as_ref())?;
        match path.as_ref().extension() {
            Some(osstr_ext) => {
                match osstr_ext.to_str() {
                    Some(str_ext) => {
                        match str_ext {
                            "ogg" => {
                                let ogg_data = Rc::new(OggStreamReader::new(reader)?);

                                self.resource_registry.add_ogg(path.as_ref(), ogg_data);
                            },
                            "tga" => {
                                let tga_data = Rc::new(tga::read(&mut reader, ColFmt::Auto)?);

                                self.resource_registry.add_tga(path.as_ref(), tga_data);
                            },
                            "gltf" => {
                                let gltf_data = Rc::new(Gltf::from_reader(reader)?.validate_completely()?);

                                self.resource_registry.add_gltf(path.as_ref(), gltf_data);
                            },
                            _ => {
                                return Err(ResourceManagerError::ResourceError(format!("The data at path {} cannot be loaded by the engine !", path.as_ref().display())));
                            }
                        }
                    },
                    None => {
                        return Err(ResourceManagerError::ResourceError(format!("The path {} is not valid unicode !", path.as_ref().display())));
                    }
                }
            },
            None => {
                return Err(ResourceManagerError::ResourceError(format!("The path {} is not valid !", path.as_ref().display())));
            }
        }
        Ok(())
    }

    fn unload_resource<P>(&mut self, path: P) -> ResourceManagerResult<()> where
        P: AsRef<Path>
    {
        match path.as_ref().extension() {
            Some(osstr_ext) => {
                match osstr_ext.to_str() {
                    Some(str_ext) => {
                        match str_ext {
                            "ogg" => {
                                self.resource_registry.remove_ogg(path.as_ref());
                            },
                            "tga" => {
                                self.resource_registry.remove_tga(path.as_ref());
                            },
                            "gltf" => {
                                self.resource_registry.remove_gltf(path.as_ref());
                            },
                            _ => {
                                return Err(ResourceManagerError::ResourceError(format!("The data at path {} cannot be unloaded by the engine !", path.as_ref().display())));
                            }
                        }
                    },
                    None => {
                        return Err(ResourceManagerError::ResourceError(format!("The path {} is not valid unicode !", path.as_ref().display())));
                    }
                }
            },
            None => {
                return Err(ResourceManagerError::ResourceError(format!("The path {} is not valid !", path.as_ref().display())));
            }
        }
        Ok(())
    }

    fn increment_refcount_resource<P>(&mut self, path: P) -> ResourceManagerResult<Option<PathBuf>> where
        P: AsRef<Path> + Into<PathBuf>
    {
        let mut new_path = None;

        if !self.refcount_registry.has_refcount(path.as_ref()) {
            //Add the refcount of this resource
            self.refcount_registry.add_refcount(path.as_ref());
            new_path = Some(path.into());
        } else {
            //increment the refcount of this resource
            self.refcount_registry.increment_refcount_of(path.as_ref())?
        }
        Ok(new_path)
    }

    fn decrement_refcount_resource<P>(&mut self, path: P) -> ResourceManagerResult<()> where
        P: AsRef<Path>
    {
        self.refcount_registry.decrement_refcount_of(path.as_ref())?;
        Ok(())
    }

    fn refcounts(&self) -> Iter<PathBuf, u8> {
        self.refcount_registry.iter()
    }
}

impl ResourceManager {
    fn with_capacity(allocator_capacity: usize, allocator_capacity_copy: usize) -> Self {
        ResourceManager {
            stack_allocator: StackAllocator::with_capacity(allocator_capacity, allocator_capacity_copy),
            refcount_registry: RefCountRegistry::new(),
            resource_registry: ResourceRegistry::new(),
        }
    }

    //First step.
    fn read_needed_resources<I>(&self, level: I) -> Vec<String> where
        I: AsRef<LevelDescription>,
    {
        let mut vec: Vec<String> = Vec::new();

        //TODO: mesh
        for gameobject_builder in level.as_ref().slice() {
            if let Some(mesh_path) = gameobject_builder.get_mesh_resource() {
                if !vec.contains(&mesh_path) {
                    vec.push(mesh_path);
                }
            }

            //TODO: other resources
        }

        vec
    }

}

#[cfg(test)]
mod resource_manager_test {
    use super::*;

    #[test]
    fn resource_manager_creation() {
        let resource_manager = ResourceManager::with_capacity(100, 50);
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