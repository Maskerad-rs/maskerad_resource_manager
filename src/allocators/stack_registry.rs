// Copyright 2017-2018 Maskerad Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use std::collections::HashMap;
use std::ops::Deref;
use maskerad_memory_allocators::stacks::{StackAllocator, StackAllocatorCopy, DoubleBufferedAllocator, DoubleBufferedAllocatorCopy, DoubleEndedStackAllocatorCopy, DoubleEndedStackAllocator};

pub struct StackRegistry(HashMap<String, StackAllocator>);
pub struct StackCopyRegistry(HashMap<String, StackAllocatorCopy>);
pub struct BufferedStackRegistry(HashMap<String, DoubleBufferedAllocator>);
pub struct BufferedStackCopyRegistry(HashMap<String, DoubleBufferedAllocatorCopy>);
pub struct DoubleEndedStackRegistry(HashMap<String, DoubleEndedStackAllocator>);
pub struct DoubleEndedStackCopyRegistry(HashMap<String, DoubleEndedStackAllocatorCopy>);

impl Deref for StackRegistry {
    type Target = HashMap<String, StackAllocator>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Deref for StackCopyRegistry {
    type Target = HashMap<String, StackAllocatorCopy>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Deref for BufferedStackRegistry {
    type Target = HashMap<String, DoubleBufferedAllocator>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Deref for BufferedStackCopyRegistry {
    type Target = HashMap<String, DoubleBufferedAllocatorCopy>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Deref for DoubleEndedStackRegistry {
    type Target = HashMap<String, DoubleEndedStackAllocator>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Deref for DoubleEndedStackCopyRegistry {
    type Target = HashMap<String, DoubleEndedStackAllocatorCopy>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}