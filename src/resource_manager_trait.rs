// Copyright 2017-2018 Maskerad Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use std::path::{Path, PathBuf};
use std::collections::hash_map::Iter;
use maskerad_data_parser::level_description::LevelDescription;
use resource_manager_errors::{ResourceManagerError, ResourceManagerResult};

//TODO: type Key and replace every PathBuf by Self::Key ?
pub trait IResourceManager {
    fn load_resource<P>(&mut self, path: P) -> ResourceManagerResult<()> where
        P: AsRef<Path> + Into<PathBuf>;
    fn unload_resource<P>(&mut self, path: P) -> ResourceManagerResult<()> where
        P: AsRef<Path>;
    fn increment_refcount_resource<P>(&mut self, path: P) -> ResourceManagerResult<Option<PathBuf>> where
        P: AsRef<Path> + Into<PathBuf>;
    fn decrement_refcount_resource<P>(&mut self, path: P) -> ResourceManagerResult<()> where
        P: AsRef<Path>;
    fn refcounts(&self) -> Iter<PathBuf, u8>;




    fn resources_to_decrease<P>(&self, new_resource_slice: P) -> Vec<PathBuf> where
        P: AsRef<[PathBuf]>,
    {
        self.refcounts()
            .filter(|elem| {
            !new_resource_slice.as_ref().contains(elem.0)
            })
            .map(|elem|{
                elem.0
            })
            .cloned()
            .collect::<Vec<PathBuf>>()
    }

    fn resources_to_unload(&self) -> Vec<PathBuf> {
        self.refcounts()
            .filter(|elem| {
                (*elem.1) == 0
            })
            .map(|elem| {
                elem.0
            })
            .cloned()
            .collect::<Vec<PathBuf>>()
    }

    fn resources_to_load(&self) -> Vec<PathBuf> {
        self.refcounts()
            .filter(|elem| {
                (*elem.1) == 1
            })
            .map(|elem| {
                elem.0
            })
            .cloned()
            .collect::<Vec<PathBuf>>()
    }

    fn increment_refcount_resources<P>(&mut self, resource_path_iter: P) -> ResourceManagerResult<Vec<PathBuf>> where
        P: IntoIterator,
        P::Item: AsRef<Path> + Into<PathBuf>,
    {
        let mut new_resources = Vec::new();
        for path in resource_path_iter {
            if let Some(new_resource_path) = self.increment_refcount_resource(path)? {
                new_resources.push(new_resource_path)
            }
        }

        Ok(new_resources)
    }

    fn decrement_refcount_resources<P>(&mut self, resource_path_iter: P) -> ResourceManagerResult<()> where
        P: IntoIterator,
        P::Item: AsRef<Path> + Into<PathBuf>,
    {
        for resource in resource_path_iter {
            self.decrement_refcount_resource(resource)?;
        }

        Ok(())
    }

    fn load_resources<P>(&mut self, resource_path_iter: P) -> ResourceManagerResult<()> where
        P: IntoIterator,
        P::Item: AsRef<Path> + Into<PathBuf>,
    {
        for resource in resource_path_iter {
            self.load_resource(resource)?;
        }

        Ok(())
    }

    fn unload_resources<P>(&mut self, resource_path_iter: P) -> ResourceManagerResult<()> where
        P: IntoIterator,
        P::Item: AsRef<Path>,
    {
        for resource in resource_path_iter {
            self.unload_resource(resource)?;
        }

        Ok(())
    }

    fn load_level_resources<P>(&mut self, resource_path_iter: P) -> ResourceManagerResult<()> where
        P: IntoIterator + AsRef<[PathBuf]>,
        P::Item : AsRef<Path> + Into<PathBuf>,
    {
                /*
                When loading level :
                1 - read all needed resources.
                2 - Increment the ref count of all those resources, and add them if they don't exist.
                3 - then decrement the ref count of each unneeded resources.
                4 - if a ref count drop to 0, unload the resource.
                5 - load all the other resources and place them in the right memory allocator.
                */
        let new_resources = self.increment_refcount_resources(resource_path_iter)?;

        let resources_to_decrease = self.resources_to_decrease(new_resources);
        self.decrement_refcount_resources(resources_to_decrease)?;

        let resources_to_unload = self.resources_to_unload();
        self.unload_resources(resources_to_unload)?;

        let resources_to_load = self.resources_to_load();
        self.load_resources(resources_to_load)?;

        Ok(())
    }
}