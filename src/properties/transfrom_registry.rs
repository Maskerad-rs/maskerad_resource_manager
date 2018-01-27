// Copyright 2017-2018 Maskerad Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use maskerad_gameobject_model::properties::transform::Transform;

pub struct TransformRegistry(HashMap<PathBuf, Transform>);

impl Deref for TransformRegistry {
    type Target = HashMap<PathBuf, Transform>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}