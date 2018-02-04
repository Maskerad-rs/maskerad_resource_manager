// Copyright 2017-2018 Maskerad Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use std::collections::HashMap;
use maskerad_gameobject_model::properties::mesh::Mesh;

#[derive(Debug)]
pub struct MeshRegistry(HashMap<String, Mesh>);

impl Default for MeshRegistry {
    fn default() -> Self {
        MeshRegistry(HashMap::default())
    }
}

impl MeshRegistry {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn get<I: AsRef<str>>(&self, path: I) -> Option<&Mesh> {
        self.0.get(path.as_ref())
    }

    pub fn remove<I: AsRef<str>>(&mut self, path: I) -> Option<Mesh> {
        self.0.remove(path.as_ref())
    }

    pub fn insert<I, J>(&mut self, path: I, mesh: J) -> Option<Mesh> where
        I: Into<String>,
        J: Into<Mesh>
    {
        self.0.insert(path.into(),mesh.into())
    }
}
