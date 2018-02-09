// Copyright 2017-2018 Maskerad Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

//TODO:Custom allocators if possible

use std::collections::HashMap;
use std::path::{PathBuf, Path};
use imagefmt::Image;


#[derive(Debug)]
pub struct TgaRegistry<'a>(HashMap<PathBuf, &'a Image<u8>>);

impl<'a> Default for TgaRegistry<'a> {
    fn default() -> Self {
        debug!("Creating a default TgaRegistry.");
        TgaRegistry(HashMap::default())
    }
}

impl<'a> TgaRegistry<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn is_empty(&self) -> bool {
        debug!("Checking if the TgaRegistry is empty.");
        self.0.is_empty()
    }

    pub fn get<I: AsRef<Path>>(&self, path: I) -> Option<&&Image<u8>> {
        debug!("Trying to get a reference to a tga image in the TgaRegistry.");
        self.0.get(path.as_ref())
    }

    pub fn remove<I: AsRef<Path>>(&mut self, path: I) -> Option<&Image<u8>> {
        debug!("Removing a tga image in the TgaRegistry.");
        self.0.remove(path.as_ref())
    }

    pub fn insert<I>(&mut self, path: I, tga_res: &'a Image<u8>) -> Option<&Image<u8>> where
        I: Into<PathBuf>,
    {
        debug!("Inserting a tga image in the TgaRegistry.");
        self.0.insert(path.into(),tga_res)
    }
}

