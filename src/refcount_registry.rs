// Copyright 2017-2018 Maskerad Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use std::path::{Path, PathBuf};
use std::collections::HashMap;
use std::ops::{Deref, DerefMut};
use resource_manager_errors::{ResourceManagerError, ResourceManagerResult};

pub struct RefCountRegistry(HashMap<PathBuf, u8>); //TODO: Maybe Cell<u8>, if problems with mutable reference number of this struct.

impl RefCountRegistry {
    pub fn new() -> Self {
        RefCountRegistry(HashMap::new())
    }

    pub fn get_refcount_of(&self, path: &Path) -> ResourceManagerResult<u8> {
        match self.get(path) {
            Some(refcount) => {
                Ok(*refcount)
            },
            None => {
                Err(ResourceManagerError::ResourceError(format!("Could not get the refcount of the resource at path {:?} in the refcount registry !", path)))
            }
        }
    }

    pub fn increment_refcount_of(&mut self, path: &Path) -> ResourceManagerResult<()> {
        match self.get_mut(path) {
            Some(ref_count) => {
                *ref_count += 1;
                Ok(())
            },
            None => {
                Err(ResourceManagerError::ResourceError(format!("Could not get the refcount of the resource at path {:?} in the refcount registry and increment it !", path)))
            }
        }
    }

    pub fn decrement_refcount_of(&mut self, path: &Path) -> ResourceManagerResult<()> {
        match self.get_mut(path) {
            Some(ref_count) => {
                *ref_count -= 1;
                Ok(())
            },
            None => {
                Err(ResourceManagerError::ResourceError(format!("Could not get the refcount of the resource at path {:?} in the refcount registry and decrement it !", path)))
            }
        }
    }

    pub fn add_refcount(&mut self, path: &Path) {
        self.insert(path.to_path_buf(), 1);
    }

    pub fn has_refcount(&self, path: &Path) -> bool {
        self.get(path).is_some()
    }
}

impl Deref for RefCountRegistry {
    type Target = HashMap<PathBuf, u8>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for RefCountRegistry {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}