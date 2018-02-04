// Copyright 2017-2018 Maskerad Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use maskerad_gameobject_model::properties::transform::Transform;
use maskerad_gameobject_model::properties::mesh::Mesh;

use properties::mesh_registry::MeshRegistry;
use properties::transform_registry::TransformRegistry;

use resource_manager_errors::{ResourceManagerError, ResourceManagerResult};

#[derive(Debug, Default)]
pub struct PropertyRegistry {
    transform_registry: TransformRegistry,
    mesh_registry: MeshRegistry,
}

impl PropertyRegistry {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn get_mesh_of<I: AsRef<str>>(&self, gameobject_id: I) -> ResourceManagerResult<&Mesh> {
        match self.mesh_registry.get(&gameobject_id) {
            Some(mesh) => {
                Ok(mesh)
            },
            None => {
                Err(ResourceManagerError::ResourceError(format!("Could not find the mesh linked to the gameobject {} in the mesh registry !", gameobject_id.as_ref())))
            },
        }
    }

    pub fn get_transform_of<I: AsRef<str>>(&self, gameobject_id: I) -> ResourceManagerResult<&Transform> {
        match self.transform_registry.get(&gameobject_id) {
            Some(transform) => {
                Ok(transform)
            },
            None => {
                Err(ResourceManagerError::ResourceError(format!("Could not find the transform linked to the gameobject {} in the transform registry !", gameobject_id.as_ref())))
            }
        }
    }

    pub fn add_mesh<I, J>(&mut self, gameobject_id: I, mesh: J) -> Option<Mesh> where
        I: Into<String>,
        J: Into<Mesh>,
    {
        self.mesh_registry.insert(gameobject_id, mesh)
    }

    pub fn add_transform<I, J>(&mut self, gameobject_id: I, transform: J) -> Option<Transform> where
        I: Into<String>,
        J: Into<Transform>
    {
        self.transform_registry.insert(gameobject_id, transform)
    }

    pub fn remove_mesh<I: AsRef<str>>(&mut self, gameobject_id: I) -> Option<Mesh> {
        self.mesh_registry.remove(gameobject_id)
    }

    pub fn remove_transfrom<I: AsRef<str>>(&mut self, gameobject_id: I) -> Option<Transform> {
        self.transform_registry.remove(gameobject_id)
    }

    pub fn has_mesh<I: AsRef<str>>(&self, gameobject_id: I) -> bool {
        self.mesh_registry.get(gameobject_id).is_some()
    }

    pub fn has_transform<I: AsRef<str>>(&self, gameobject_id: I) -> bool {
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