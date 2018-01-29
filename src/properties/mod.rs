// Copyright 2017-2018 Maskerad Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use std::collections::HashMap;
use std::path::{PathBuf, Path};
use maskerad_gameobject_model::properties::transform::Transform;
use maskerad_gameobject_model::properties::mesh::Mesh;

use std::rc::Rc;

use properties::mesh_registry::MeshRegistry;
use properties::transform_registry::TransformRegistry;

use resource_manager_errors::{ResourceManagerError, ResourceManagerResult};

pub struct PropertiesRegistry {
    transform_registry: TransformRegistry,
    mesh_registry: MeshRegistry,
}

impl PropertiesRegistry {
    pub fn new() -> Self {
        PropertiesRegistry {
            transform_registry: TransformRegistry::new(),
            mesh_registry: MeshRegistry::new(),
        }
    }

    pub fn get_mesh_of(&self, gameobject_id: &str) -> ResourceManagerResult<&Mesh> {
        match self.mesh_registry.get(gameobject_id) {
            Some(mesh) => {
                Ok(mesh)
            },
            None => {
                Err(ResourceManagerError::ResourceError(format!("Could not find the mesh linked to the gameobject {} in the mesh registry !", gameobject_id)))
            },
        }
    }

    pub fn get_transform_of(&self, gameobject_id: &str) -> ResourceManagerResult<&Transform> {
        match self.transform_registry.get(gameobject_id) {
            Some(transform) => {
                Ok(transform)
            },
            None => {
                Err(ResourceManagerError::ResourceError(format!("Could not find the transform linked to the gameobject {} in the transform registry !", gameobject_id)))
            }
        }
    }

    pub fn add_mesh(&mut self, gameobject_id: &str, mesh: Mesh) {
        self.mesh_registry.insert(String::from(gameobject_id), mesh);
    }

    pub fn add_transform(&mut self, gameobject_id: &str, transform: Transform) {
        self.transform_registry.insert(String::from(gameobject_id), transform);
    }

    pub fn remove_mesh(&mut self, gameobject_id: &str) {
        self.mesh_registry.remove(gameobject_id);
    }

    pub fn remove_transfrom(&mut self, gameobject_id: &str) {
        self.transform_registry.remove(gameobject_id);
    }

    pub fn has_mesh(&self, gameobject_id: &str) -> bool {
        self.mesh_registry.get(gameobject_id).is_some()
    }

    pub fn has_transform(&self, gameobject_id: &str) -> bool {
        self.transform_registry.get(gameobject_id).is_some()
    }

    pub fn is_transform_empty(&self) -> bool {
        self.transform_registry.is_empty()
    }

    pub fn is_mesh_empty(&self) -> bool {
        self.mesh_registry.is_empty()
    }
}

mod transform_registry;
mod mesh_registry;