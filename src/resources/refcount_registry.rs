// Copyright 2017-2018 Maskerad Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use std::path::{Path, PathBuf};
use std::collections::HashMap;
use resources::resource_manager_errors::{ResourceManagerError, ResourceManagerResult};
use std::collections::hash_map::{Iter, Keys};

#[derive(Debug)]
pub struct RefCountRegistry(HashMap<PathBuf, u8>); //TODO: Maybe Cell<u8>, if problems with mutable reference number of this struct.

impl Default for RefCountRegistry {
    fn default() -> Self {
        RefCountRegistry(HashMap::default())
    }
}

impl RefCountRegistry {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn get_refcount_of<P: AsRef<Path>>(&self, path: P) -> ResourceManagerResult<u8> {
        match self.0.get(path.as_ref()) {
            Some(refcount) => {
                Ok(*refcount)
            },
            None => {
                Err(ResourceManagerError::ResourceError(format!("Could not get the refcount of the resource at path {} in the refcount registry !", path.as_ref().display())))
            }
        }
    }

    pub fn increment_refcount_of<P: AsRef<Path>>(&mut self, path: P) -> ResourceManagerResult<()> {
        match self.0.get_mut(path.as_ref()) {
            Some(ref_count) => {
                *ref_count += 1;
                Ok(())
            },
            None => {
                Err(ResourceManagerError::ResourceError(format!("Could not get the refcount of the resource at path {} in the refcount registry and increment it !", path.as_ref().display())))
            }
        }
    }

    pub fn decrement_refcount_of<P: AsRef<Path>>(&mut self, path: P) -> ResourceManagerResult<()> {
        match self.0.get_mut(path.as_ref()) {
            Some(ref_count) => {
                *ref_count -= 1;
                Ok(())
            },
            None => {
                Err(ResourceManagerError::ResourceError(format!("Could not get the refcount of the resource at path {} in the refcount registry and decrement it !", path.as_ref().display())))
            }
        }
    }

    pub fn add_refcount<P: Into<PathBuf>>(&mut self, path: P) {
        self.0.insert(path.into(), 1);
    }

    pub fn has_refcount<P: AsRef<Path>>(&self, path: P) -> bool {
        self.0.get(path.as_ref()).is_some()
    }

    pub fn iter(&self) -> Iter<PathBuf, u8> {
        self.0.iter()
    }

    pub fn keys(&self) -> Keys<PathBuf, u8> {
        self.0.keys()
    }
}