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
use std::rc::Rc;
use std::ops::{Deref, DerefMut};


#[derive(Debug)]
pub struct TgaRegistry(HashMap<PathBuf, Rc<Image<u8>>>);

impl Default for TgaRegistry {
    fn default() -> Self {
        TgaRegistry(HashMap::default())
    }
}

impl TgaRegistry {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn get<I: AsRef<Path>>(&self, path: I) -> Option<&Rc<Image<u8>>> {
        self.0.get(path.as_ref())
    }

    pub fn remove<I: AsRef<Path>>(&mut self, path: I) -> Option<Rc<Image<u8>>> {
        self.0.remove(path.as_ref())
    }

    pub fn insert<I, J>(&mut self, path: I, tga_res: J) -> Option<Rc<Image<u8>>> where
        I: Into<PathBuf>,
        J: Into<Rc<Image<u8>>>
    {
        self.0.insert(path.into(),tga_res.into())
    }
}

