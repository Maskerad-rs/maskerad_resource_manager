// Copyright 2017-2018 Maskerad Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

//TODO:Custom allocators if possible

use std::collections::HashMap;
use std::path::{PathBuf, Path};
use lewton::inside_ogg::OggStreamReader;

use std::io::BufReader;
use std::fs::File;

pub struct OggRegistry<'a>(HashMap<PathBuf, &'a OggStreamReader<BufReader<File>>>);

impl<'a> Default for OggRegistry<'a> {
    fn default() -> Self {
        OggRegistry(HashMap::default())
    }
}

impl<'a> OggRegistry<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn get<I: AsRef<Path>>(&self, path: I) -> Option<&&OggStreamReader<BufReader<File>>> {
        self.0.get(path.as_ref())
    }

    pub fn remove<I: AsRef<Path>>(&mut self, path: I) -> Option<&OggStreamReader<BufReader<File>>> {
        self.0.remove(path.as_ref())
    }

    pub fn insert<I>(&mut self, path: I, ogg_res: &'a OggStreamReader<BufReader<File>>) -> Option<&OggStreamReader<BufReader<File>>> where
        I: Into<PathBuf>,
    {
        self.0.insert(path.into(),ogg_res)
    }
}