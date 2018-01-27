// Copyright 2017-2018 Maskerad Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use std::collections::HashMap;
use std::path::{PathBuf, Path};
use gltf::{Gltf, Glb};

use maskerad_filesystem::filesystem::FileSystem;
use maskerad_filesystem::game_directories::RootDir;
use maskerad_filesystem::file_extension::FileExtension;

use maskerad_memory_allocators::{StackAllocator, StackAllocatorCopy};

use maskerad_data_parser::level_description::LevelDescription;
use maskerad_data_parser::gameobject_description::{GameObjectDescription};

use properties_registry::PropertiesMap;

use resource_manager_errors::{ResourceManagerError, ResourceManagerResult};
//TODO: See if we can remove the RefCell.
use std::cell::RefCell;

pub struct ResourceManager {
    //A resources registry
    //A properties registry
    //An allocators registry
    //A refcount registry

    gltf_registry: RefCell<HashMap<PathBuf, Gltf>>,
    resource_ref_count: RefCell<HashMap<PathBuf, u8>>,
    memory_allocator: StackAllocator,
    copy_memory_allocator: StackAllocatorCopy,
    properties_registry: PropertiesMap,
}

impl ResourceManager {
    fn new(stack_size: usize, copy_stack_size: usize) -> Self {
        ResourceManager {
            gltf_registry: RefCell::new(HashMap::new()),
            resource_ref_count: RefCell::new(HashMap::new()),
            memory_allocator: StackAllocator::with_capacity(stack_size),
            copy_memory_allocator: StackAllocatorCopy::with_capacity(copy_stack_size),
            properties_registry: PropertiesMap::new(),
        }
    }


    //Doesn't actually "create" the level, we just load all the needed resources for all the objects inside the level.
    pub fn load_level_resources(&self, level_description: &LevelDescription, file_system: &FileSystem) -> ResourceManagerResult<()> {
        /*
        When loading level :
        1 - read all needed resources and increment their ref count by one.
        2 - then decrement the ref count of each unneeded resources.
        3 - if a ref count drop to 0, unload the resource.
        4 - load all the other resources and place them in the right memory allocator.
        */

        //read the level file
        let mut content = String::new();

        for paths in level_description.gameobject_description_paths().iter() {
            content.clear();
            let mut reader = file_system.open(paths.as_ref())?;
            file_system.read_to_string(&mut reader, &mut content)?;

            let go_desc = GameObjectDescription::load_from_toml(content.as_str())?;

            //TODO: paths to all needed resources of the game object
            //1 - get the needed resources
            let mut new_paths = Vec::new();
            let mesh_path = PathBuf::from("test");
            new_paths.push(mesh_path.clone());






            //increment the ref count by one
            for new_resource_path in new_paths.iter() {
                if let Some(ref_count) = self.resource_ref_count.borrow_mut().get_mut(new_resource_path) {
                    *ref_count += 1;
                } else {
                    self.resource_ref_count.borrow_mut().insert(new_resource_path.clone(), 1);
                }
            }






            //2 - decrement unneeded resources
            let resources_to_decrease = self.resource_ref_count.borrow_mut().iter_mut().filter(|elem| {
                !new_paths.contains(elem.0)
            }).map(|elem| {
                elem.0
            }).cloned().collect::<Vec<PathBuf>>();

            for pathbufs in resources_to_decrease.into_iter() {
                if let Some(ref_count) = self.resource_ref_count.borrow_mut().get_mut(&pathbufs) {
                    *ref_count -= 1;
                }
            }







            //3 - unload resources with a ref count of 0.
            let resources_to_unload = self.resource_ref_count.borrow().iter().filter(|elem| {
                (*elem.1) == 0
            }).map(|elem| {
                elem.0
            }).cloned().collect::<Vec<PathBuf>>();

            for pathbufs in resources_to_unload.into_iter() {
                self.unload_resource(pathbufs.as_path(), file_system)?;
            }








            //4 - load all the needed resources (only the new with a ref count of 1, other are already loaded).
            let resources_to_load = self.resource_ref_count.borrow().iter().filter(|elem| {
                (*elem.1) == 1
            }).map(|elem| {
                elem.0
            }).cloned().collect::<Vec<PathBuf>>();

            for pathbufs in resources_to_load.into_iter() {
                self.load_resource(pathbufs.as_path(), file_system).unwrap();
            }

        }

        Ok(())
    }

    pub fn load_resource(&self, path: &Path, file_system: &FileSystem) -> ResourceManagerResult<()> {
        let bufreader = file_system.open(path)?;
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
                //TODO: the registry take ptr/ref to data as values, he doesn't take ownership of it.

                if let None = self.gltf_registry.borrow().get(path) {
                    self.gltf_registry.borrow_mut().insert(path.to_path_buf(), gltf_data);
                }
            },
            FileExtension::TOML => {
                return Err(ResourceManagerError::ResourceError(format!("TOML files are not valid resources to load !")));
            }
        }

        Ok(())
    }

    pub fn unload_resource(&self, path: &Path, file_system: &FileSystem) -> ResourceManagerResult<()> {
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
                self.gltf_registry.borrow_mut().remove(path);
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