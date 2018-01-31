// Copyright 2017-2018 Maskerad Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use resources::gltf_registry::GltfRegistry;
use std::path::Path;
use resource_manager_errors::{ResourceManagerError, ResourceManagerResult};
use std::rc::Rc;
use gltf::Gltf;
use lewton::inside_ogg::OggStreamReader;
use std::io::BufReader;
use std::fs::File;
use resources::ogg_registry::OggRegistry;
use resources::tga_registry::TgaRegistry;
use imagefmt::Image;

pub struct ResourceRegistry {
    gltf_registry: GltfRegistry,
    ogg_registry: OggRegistry,
    tga_registry: TgaRegistry,
}

impl ResourceRegistry {
    pub fn new() -> Self {
        ResourceRegistry {
            gltf_registry: GltfRegistry::new(),
            ogg_registry: OggRegistry::new(),
            tga_registry: TgaRegistry::new(),
        }
    }

    //____________________GLTF____________________________
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

    pub fn is_gltf_empty(&self) -> bool {
        self.gltf_registry.is_empty()
    }


    //_________________________OGG______________________
    pub fn get_ogg(&self, path: &Path) -> ResourceManagerResult<Rc<OggStreamReader<BufReader<File>>>> {
        match self.ogg_registry.get(path) {
            Some(ogg) => {
                Ok(ogg.clone())
            },
            None => {
                Err(ResourceManagerError::ResourceError(format!("Could not find the ogg data at path {:?} in the ogg registry !", path)))
            },
        }
    }

    pub fn add_ogg(&mut self, path: &Path, ogg_resource: OggStreamReader<BufReader<File>>) {
        self.ogg_registry.insert(path.to_path_buf(), Rc::new(ogg_resource));
    }

    pub fn remove_ogg(&mut self, path: &Path) {
        self.ogg_registry.remove(path);
    }

    pub fn has_ogg(&self, path: &Path) -> bool {
        self.ogg_registry.get(path).is_some()
    }

    pub fn is_ogg_empty(&self) -> bool {
        self.ogg_registry.is_empty()
    }

    //__________________________TGA_____________________
    pub fn get_tga(&self, path: &Path) -> ResourceManagerResult<Rc<Image<u8>>> {
        match self.tga_registry.get(path) {
            Some(tga) => {
                Ok(tga.clone())
            },
            None => {
                Err(ResourceManagerError::ResourceError(format!("Could not find the tga data at path {:?} in the tga registry !", path)))
            },
        }
    }

    pub fn add_tga(&mut self, path: &Path, tga_resource: Image<u8>) {
        self.tga_registry.insert(path.to_path_buf(), Rc::new(tga_resource));
    }

    pub fn remove_tga(&mut self, path: &Path) {
        self.tga_registry.remove(path);
    }

    pub fn has_tga(&self, path: &Path) -> bool {
        self.tga_registry.get(path).is_some()
    }

    pub fn is_tga_empty(&self) -> bool {
        self.tga_registry.is_empty()
    }
}

mod gltf_registry;
mod ogg_registry;
mod tga_registry;