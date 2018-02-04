// Copyright 2017-2018 Maskerad Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use resources::gltf_registry::GltfRegistry;
use std::path::{Path, PathBuf};
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

impl Default for ResourceRegistry {
    fn default() -> Self {
        ResourceRegistry {
            gltf_registry: GltfRegistry::default(),
            ogg_registry: OggRegistry::default(),
            tga_registry: TgaRegistry::default(),
        }
    }
}

impl ResourceRegistry {
    pub fn new() -> Self {
        Default::default()
    }

    //____________________GLTF____________________________
    pub fn get_gltf<I: AsRef<Path>>(&self, path: I) -> ResourceManagerResult<Rc<Gltf>> {
        match self.gltf_registry.get(&path) {
            Some(gltf) => {
                Ok(gltf.clone())
            },
            None => {
                Err(ResourceManagerError::ResourceError(format!("Could not find the gltf data at path {} in the gltf registry !", path.as_ref().display())))
            },
        }
    }

    pub fn add_gltf<I, J>(&mut self, path: I, gltf_resource: J) -> Option<Rc<Gltf>> where
        I: Into<PathBuf>,
        J: Into<Rc<Gltf>>
    {
        self.gltf_registry.insert(path, gltf_resource)
    }

    pub fn remove_gltf<I: AsRef<Path>>(&mut self, path: I) {
        self.gltf_registry.remove(path);
    }

    pub fn has_gltf<I: AsRef<Path>>(&self, path: I) -> bool {
        self.gltf_registry.get(path).is_some()
    }

    pub fn is_gltf_empty(&self) -> bool {
        self.gltf_registry.is_empty()
    }


    //_________________________OGG______________________
    pub fn get_ogg<I: AsRef<Path>>(&self, path: I) -> ResourceManagerResult<Rc<OggStreamReader<BufReader<File>>>> {
        match self.ogg_registry.get(&path) {
            Some(ogg) => {
                Ok(ogg.clone())
            },
            None => {
                Err(ResourceManagerError::ResourceError(format!("Could not find the ogg data at path {} in the ogg registry !", path.as_ref().display())))
            },
        }
    }

    pub fn add_ogg<I, J>(&mut self, path: I, ogg_resource: J) -> Option<Rc<OggStreamReader<BufReader<File>>>> where
        I: Into<PathBuf>,
        J: Into<Rc<OggStreamReader<BufReader<File>>>>,
    {
        self.ogg_registry.insert(path, ogg_resource)
    }

    pub fn remove_ogg<I: AsRef<Path>>(&mut self, path: I) {
        self.ogg_registry.remove(path);
    }

    pub fn has_ogg<I: AsRef<Path>>(&self, path: I) -> bool {
        self.ogg_registry.get(path).is_some()
    }

    pub fn is_ogg_empty(&self) -> bool {
        self.ogg_registry.is_empty()
    }

    //__________________________TGA_____________________
    pub fn get_tga<I: AsRef<Path>>(&self, path: I) -> ResourceManagerResult<Rc<Image<u8>>> {
        match self.tga_registry.get(&path) {
            Some(tga) => {
                Ok(tga.clone())
            },
            None => {
                Err(ResourceManagerError::ResourceError(format!("Could not find the tga data at path {} in the tga registry !", path.as_ref().display())))
            },
        }
    }

    pub fn add_tga<I, J>(&mut self, path: I, tga_resource: J) -> Option<Rc<Image<u8>>> where
        I: Into<PathBuf>,
        J: Into<Rc<Image<u8>>>,
    {
        self.tga_registry.insert(path, tga_resource)
    }

    pub fn remove_tga<I: AsRef<Path>>(&mut self, path: I) {
        self.tga_registry.remove(path);
    }

    pub fn has_tga<I: AsRef<Path>>(&self, path: I) -> bool {
        self.tga_registry.get(path).is_some()
    }

    pub fn is_tga_empty(&self) -> bool {
        self.tga_registry.is_empty()
    }
}

mod gltf_registry;
mod ogg_registry;
mod tga_registry;