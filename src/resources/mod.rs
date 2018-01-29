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
use resources::flac_registry::FlacRegistry;
use claxon::FlacReader;
use lewton::inside_ogg::OggStreamReader;
use std::io::{Read, BufReader};
use std::fs::File;
use resources::ogg_registry::OggRegistry;

pub struct ResourceRegistry {
    gltf_registry: GltfRegistry,
    flac_registry: FlacRegistry,
    ogg_registry: OggRegistry,
}

impl ResourceRegistry {
    pub fn new() -> Self {
        ResourceRegistry {
            gltf_registry: GltfRegistry::new(),
            flac_registry: FlacRegistry::new(),
            ogg_registry: OggRegistry::new(),
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

    //_________________________FLAC_____________________
    pub fn get_flac(&self, path: &Path) -> ResourceManagerResult<Rc<FlacReader<BufReader<File>>>> {
        match self.flac_registry.get(path) {
            Some(flac) => {
                Ok(flac.clone())
            },
            None => {
                Err(ResourceManagerError::ResourceError(format!("Could not find the flac data at path {:?} in the flac registry !", path)))
            },
        }
    }

    pub fn add_flac(&mut self, path: &Path, flac_resource: FlacReader<BufReader<File>>) {
        self.flac_registry.insert(path.to_path_buf(), Rc::new(flac_resource));
    }

    pub fn remove_flac(&mut self, path: &Path) {
        self.flac_registry.remove(path);
    }

    pub fn has_flac(&self, path: &Path) -> bool {
        self.flac_registry.get(path).is_some()
    }

    pub fn is_flac_empty(&self) -> bool {
        self.flac_registry.is_empty()
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
}

mod gltf_registry;
mod flac_registry;
mod ogg_registry;
mod tga_registry;