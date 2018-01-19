// Copyright 2017-2018 Maskerad Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use std::collections::HashMap;
use std::path::PathBuf;
use gltf::{Gltf, Glb};
use maskerad_filesystem::filesystem::FileSystem;
use maskerad_filesystem::game_directories::RootDir;
use maskerad_filesystem::file_extension::FileExtension;
use resource_manager_errors::{ResourceManagerError, ResourceManagerResult};

pub struct ResourceManager {
    gltf_registry: HashMap<PathBuf, Gltf>,
    resource_ref_count: HashMap<PathBuf, u8>,

}

impl ResourceManager {
    fn new() -> Self {
        ResourceManager {
            gltf_registry: HashMap::new(),
            resource_ref_count: HashMap::new(),
        }
    }

    pub fn load_resource(&mut self, root_dir: &RootDir, path: &str, file_system: &FileSystem) -> ResourceManagerResult<()> {
        let full_path = file_system.get_absolute_path(root_dir, path)?;
        let bufreader = file_system.open(root_dir, path)?;
        let file_extension = file_system.get_file_extension(path)?;

        match file_extension {
            FileExtension::FLAC => {
                unimplemented!()
            },
            FileExtension::OGG => {
                unimplemented!()
            },
            FileExtension::TGA => {
                unimplemented!()
            },
            FileExtension::GLTF => {
                let gltf_data = Gltf::from_reader(bufreader)?.validate_completely()?;

                if let None = self.gltf_registry.get(&full_path) {
                    self.gltf_registry.insert(full_path, gltf_data);
                }
            }
        }

        Ok(())
    }

    pub fn unload_resource(&mut self, root_dir: &RootDir, path: &str, file_system: &FileSystem) -> ResourceManagerResult<()> {
        let full_path = file_system.get_absolute_path(root_dir, path)?;
        let file_extension = file_system.get_file_extension(path)?;
        match file_extension {
            FileExtension::FLAC => {
                unimplemented!()
            },
            FileExtension::OGG => {
                unimplemented!()
            },
            FileExtension::TGA => {
              unimplemented!()
            },
            FileExtension::GLTF => {
                self.gltf_registry.remove(&full_path);
            },
        }

        Ok(())
    }

    pub fn get_gltf_resource(&self, root_dir: &RootDir, path: &str, file_system: &FileSystem) -> ResourceManagerResult<&Gltf> {
        match self.gltf_registry.get(&full_path) {
            Some(gltf_data) => {
                Ok(gltf_data)
            },
            None => {
                Err(ResourceManagerError::ResourceError(format!("The gltf data at path {} could not be found in the gltf registry ! Are you sure this data was loaded ?", path.display())))
            }
        }
    }

    //TODO: a get_..._resource for all types. Stop with the trait object madness.
}

#[cfg(test)]
mod resource_manager_test {
    use super::*;

    #[test]
    fn resource_manager_creation() {
        let resource_manager = ResourceManager::new();
        assert!(resource_manager.registry.is_empty());
        assert!(resource_manager.resource_ref_count.is_empty());
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
    fn resource_manager_get_ogg_flac_resource() {

    }

    #[test]
    fn resource_manager_get_tga_resource() {

    }

    #[test]
    fn resource_manager_get_flac_resource() {

    }

}