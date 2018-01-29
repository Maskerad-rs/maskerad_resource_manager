// Copyright 2017-2018 Maskerad Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use std::collections::HashMap;
use std::path::{PathBuf, Path};
use gltf::{Gltf, Glb};
use claxon::FlacReader;
use std::io::Read;
use std::io::BufReader;
use lewton::inside_ogg::OggStreamReader;

use imagefmt::Image;
use imagefmt::tga;
use imagefmt::ColFmt;

use maskerad_filesystem::filesystem::FileSystem;
use maskerad_filesystem::game_directories::RootDir;
use maskerad_filesystem::file_extension::FileExtension;

use maskerad_data_parser::level_description::LevelDescription;
use maskerad_data_parser::gameobject_description::{GameObjectDescription};

use properties::PropertiesRegistry;
use resources::ResourceRegistry;
use refcount_registry::RefCountRegistry;

use maskerad_gameobject_model::properties::transform::Transform;

use resource_manager_errors::{ResourceManagerError, ResourceManagerResult};
//TODO: See if we can remove the RefCell.
use std::cell::RefCell;

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
    fn increment_refcounts(&mut self, path_new_resources: &[PathBuf]) {

        for path in path_new_resources.iter() {
            if !self.refcount_registry.has_refcount(path.as_path()) {
                //Add the refcount of this resource
                self.refcount_registry.add_refcount(path.as_path());
            } else {
                //increment the refcount of this resource
                self.refcount_registry.increment_refcount_of(path.as_path());
            }
        }
    }

    //Third step.
    fn decrement_refcounts(&mut self, resources_to_decrease: &[PathBuf]) {
        for path in resources_to_decrease.iter() {
            self.refcount_registry.decrement_refcount_of(path.as_path());
        }
    }

    //Fourth step.
    fn unload_resources(&mut self, resources_to_unload: &[PathBuf], file_system: &FileSystem) -> ResourceManagerResult<()> {
        for path in resources_to_unload.iter() {
            self.unload_resource(path.as_path(), file_system)?;
        }
        Ok(())
    }

    //Fifth step.
    fn load_resources(&mut self, resources_to_load: &[PathBuf], file_system: &FileSystem) -> ResourceManagerResult<()> {
        for path in resources_to_load.iter() {
            self.load_resource(path.as_path(), file_system)?;
        }
        Ok(())
    }

    //Doesn't actually "create" the level, we just load all the needed resources for all the objects inside the level.
    pub fn load_level_resources(&mut self, level_description: &LevelDescription, file_system: &FileSystem) -> ResourceManagerResult<()> {
        /*
        When loading level :
        1 - read all needed resources.
        2 - Increment the ref count of all those resources, and add them if they don't exist.
        3 - then decrement the ref count of each unneeded resources.
        4 - if a ref count drop to 0, unload the resource.
        5 - load all the other resources and place them in the right memory allocator.
        */

        //Generate all the gameobject descriptions
        let gameobject_descriptions = level_description.generate_gameobject_descriptions(file_system)?;
        let path_new_resources = self.read_needed_resources(&gameobject_descriptions);
        self.increment_refcounts(&path_new_resources);



        let resources_to_decrease = self.refcount_registry.iter().filter(|elem| {
            !path_new_resources.contains(elem.0)
        }).map(|elem| {
            elem.0
        }).cloned().collect::<Vec<PathBuf>>();
        self.decrement_refcounts(&resources_to_decrease);



        let resources_to_unload = self.refcount_registry.iter().filter(|elem| {
            (*elem.1) == 0
        }).map(|elem| {
            elem.0
        }).cloned().collect::<Vec<PathBuf>>();
        self.unload_resources(&resources_to_unload, file_system)?;



        let resources_to_load = self.refcount_registry.iter().filter(|elem| {
            (*elem.1) == 1
        }).map(|elem| {
            elem.0
        }).cloned().collect::<Vec<PathBuf>>();
        self.load_resources(&resources_to_load, file_system)?;

        Ok(())
    }

    //TODO: The filesystem should stay outside of those functions, the fs should give the resource to those functions.
    pub fn load_resource(&mut self, path: &Path, file_system: &FileSystem) -> ResourceManagerResult<()> {
        let mut bufreader = file_system.open(path)?;
        let file_extension = file_system.get_file_extension(path)?;

        match file_extension {
            FileExtension::FLAC => {
                let flac_reader = FlacReader::new(bufreader)?;
                self.resource_registry.add_flac(path, flac_reader);
            },
            FileExtension::OGG => {
                let ogg_reader = OggStreamReader::new(bufreader)?;
                self.resource_registry.add_ogg(path, ogg_reader);
            },
            FileExtension::TGA => {
                let tga_image = tga::read(&mut bufreader, ColFmt::Auto)?;
                self.resource_registry.add_tga(path, tga_image);
            },
            FileExtension::GLTF => {
                let gltf_data = Gltf::from_reader(bufreader)?.validate_completely()?;
                self.resource_registry.add_gltf(path, gltf_data);
            },
            FileExtension::TOML => {
                return Err(ResourceManagerError::ResourceError(format!("TOML files are not valid resources to load !")));
            }
        }

        Ok(())
    }

    pub fn unload_resource(&mut self, path: &Path, file_system: &FileSystem) -> ResourceManagerResult<()> {
        let file_extension = file_system.get_file_extension(path)?;
        match file_extension {
            FileExtension::FLAC => {
                self.resource_registry.remove_flac(path);
            },
            FileExtension::OGG => {
                self.resource_registry.remove_ogg(path);
            },
            FileExtension::TGA => {
              self.resource_registry.remove_tga(path);
            },
            FileExtension::GLTF => {
                self.resource_registry.remove_gltf(path);
            },
            FileExtension::TOML => {
                return Err(ResourceManagerError::ResourceError(format!("TOML files are not valid resources to unload! ")));
            }
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