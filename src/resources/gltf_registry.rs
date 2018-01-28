// Copyright 2017-2018 Maskerad Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use gltf::Gltf;
use std::rc::Rc;
use std::ops::{Deref, DerefMut};

//TODO: not sure about that fucking shitty memory pool.
//use maskerad_memory_allocators::smart_pointers::SharedPtr;

pub struct GltfRegistry(HashMap<PathBuf, Rc<Gltf>>);

impl GltfRegistry {
    pub fn new() -> Self {
        GltfRegistry(HashMap::new())
    }
}

impl Deref for GltfRegistry {
    type Target = HashMap<PathBuf, Rc<Gltf>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for GltfRegistry {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}