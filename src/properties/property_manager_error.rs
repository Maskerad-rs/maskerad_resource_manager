// Copyright 2017-2018 Maskerad Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum PropertyManagerError {
    TransformError(String),
    MeshError(String),
}

unsafe impl Send for PropertyManagerError {}
unsafe impl Sync for PropertyManagerError {}

impl fmt::Display for PropertyManagerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &PropertyManagerError::TransformError(ref description) => {
                write!(f, "Transform registry error: {}", description)
            },
            &PropertyManagerError::MeshError(ref description) => {
                write!(f, "Mesh registry error: {}", description)
            },
        }
    }
}

impl Error for PropertyManagerError {
    fn description(&self) -> &str {
        match self {
            &PropertyManagerError::TransformError(_) => {
                "TransformError"
            },
            &PropertyManagerError::MeshError(_) => {
                "MeshError"
            }
        }
    }

    fn cause(&self) -> Option<&Error> {
        match self {
            &PropertyManagerError::TransformError(_) => {
                None
            },
            &PropertyManagerError::MeshError(_) => {
                None
            }
        }
    }
}

pub type PropertyManagerResult<T> = Result<T, PropertyManagerError>;
