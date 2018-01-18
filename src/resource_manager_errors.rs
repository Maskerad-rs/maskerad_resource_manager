// Copyright 2017-2018 Maskerad Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use std::error::Error;
use std::fmt;
use maskerad_filesystem::filesystem_error::FileSystemError;
use gltf::Error as GltfError;


#[derive(Debug)]
pub enum ResourceManagerError {
    FilesystemError(String, FileSystemError),
    GltfError(String, GltfError),
    ResourceError(String),
}

impl fmt::Display for ResourceManagerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &ResourceManagerError::FilesystemError(ref description, _) => {
                write!(f, "Filesystem error: {}", description)
            },
            &ResourceManagerError::GltfError(ref description, _) => {
                write!(f, "Gltf error: {}", description)
            },
            &ResourceManagerError::ResourceError(ref description) => {
                write!(f, "Resource error: {}", description)
            }
        }
    }
}

impl Error for ResourceManagerError {
    fn description(&self) -> &str {
        match self {
            &ResourceManagerError::FilesystemError(_, _) => {
                "ResourceNotFound"
            },
            &ResourceManagerError::GltfError(_, _) => {
                "GltfError"
            },
            &ResourceManagerError::ResourceError(_) => {
                "ResourceError"
            },
        }
    }

    fn cause(&self) -> Option<&Error> {
        match self {
            &ResourceManagerError::FilesystemError(_, ref file_system_error) => {
                Some(file_system_error)
            },
            &ResourceManagerError::GltfError(_, ref gltf_error) => {
                Some(gltf_error)
            },
            &ResourceManagerError::ResourceError(_) => {
                None
            },
        }
    }
}

pub type ResourceManagerResult<T> = Result<T, ResourceManagerError>;

impl From<FileSystemError> for ResourceManagerError {
    fn from(error: FileSystemError) -> Self {
        ResourceManagerError::FilesystemError(format!("Error while dealing with the filesystem."), error)
    }
}

impl From<GltfError> for ResourceManagerError {
    fn from(error: GltfError) -> Self {
        ResourceManagerError::GltfError(format!("Error while dealing with a gltf structure."), error)
    }
}



