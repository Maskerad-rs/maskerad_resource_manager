// Copyright 2017-2018 Maskerad Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

//TODO:Custom allocators if possible

use std::collections::HashMap;
use std::path::PathBuf;
use imagefmt::Image;
use std::rc::Rc;
use std::ops::{Deref, DerefMut};


pub struct TgaRegistry(HashMap<PathBuf, Rc<Image<u8>>>);

impl TgaRegistry {
    pub fn new() -> Self {
        TgaRegistry(HashMap::new())
    }
}

impl Deref for TgaRegistry {
    type Target = HashMap<PathBuf, Rc<Image<u8>>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for TgaRegistry {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}