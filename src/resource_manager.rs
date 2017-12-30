// Copyright 2017 Maskerad Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use std::collections::HashMap;
use std::path::PathBuf;
use gltf::Gltf;
use maskerad_filesystem::filesystem::Filesystem;

pub struct ResourceManager {
    registry: HashMap<PathBuf, Gltf>,
    resource_ref_count: HashMap<PathBuf, u8>,

}

impl ResourceManager {
    fn new() -> Self {
        ResourceManager {
            registry: HashMap::new(),
            resource_ref_count: HashMap::new(),
        }
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
}