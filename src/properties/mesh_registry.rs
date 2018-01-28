// Copyright 2017-2018 Maskerad Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use std::collections::HashMap;
use std::ops::{Deref, DerefMut};
use maskerad_gameobject_model::properties::mesh::Mesh;

pub struct MeshRegistry(HashMap<String, Mesh>);

impl MeshRegistry {
    pub fn new() -> Self {
        MeshRegistry(HashMap::new())
    }
}

impl Deref for MeshRegistry {
    type Target = HashMap<String, Mesh>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for MeshRegistry {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}