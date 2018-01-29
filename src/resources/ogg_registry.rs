// Copyright 2017-2018 Maskerad Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

//TODO:Custom allocators if possible

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use lewton::inside_ogg::OggStreamReader;
use std::rc::Rc;
use std::ops::{Deref, DerefMut};

use std::io::BufReader;
use std::fs::File;

use std::io::Read;

pub struct OggRegistry(HashMap<PathBuf, Rc<OggStreamReader<BufReader<File>>>>);

impl OggRegistry {
    pub fn new() -> Self {
        OggRegistry(HashMap::new())
    }
}

impl Deref for OggRegistry {
    type Target = HashMap<PathBuf, Rc<OggStreamReader<BufReader<File>>>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for OggRegistry {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}