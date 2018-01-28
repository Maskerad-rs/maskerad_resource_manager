// Copyright 2017-2018 Maskerad Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use resources::gltf_registry::GltfRegistry;
use std::path::{PathBuf, Path};
use resource_manager_errors::{ResourceManagerError, ResourceManagerResult};
use std::rc::Rc;
use gltf::Gltf;

pub struct ResourceRegistry {
    gltf_registry: GltfRegistry,
}

impl ResourceRegistry {
    pub fn new() -> Self {
        ResourceRegistry {
            gltf_registry: GltfRegistry::new(),
        }
    }

    pub fn get_gltf(&self, path: &Path) -> ResourceManagerResult<Rc<Gltf>> {
        match self.gltf_registry.get(path) {
            Some(gltf) => {
                Ok(gltf.clone())
            },
            None => {
                Err(ResourceManagerError::ResourceError(format!("Could not find the gltf data at path {:?} in the gltf registry !", path)))
            },
        }
    }

    pub fn add_gltf(&mut self, path: &Path, gltf_resource: Gltf) {
        self.gltf_registry.insert(path.to_path_buf(), Rc::new(gltf_resource));
    }

    pub fn remove_gltf(&mut self, path: &Path) {
        self.gltf_registry.remove(path);
    }

    pub fn has_gltf(&self, path: &Path) -> bool {
        self.gltf_registry.get(path).is_some()
    }

    pub fn gltf_registry(&self) -> &GltfRegistry {
        &self.gltf_registry
    }
}

mod gltf_registry;