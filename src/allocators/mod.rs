// Copyright 2017-2018 Maskerad Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use maskerad_memory_allocators::StackAllocator;

pub struct AllocatorRegistry {
    persistent_allocator: StackAllocator,
    one_frame_allocator: StackAllocator,
    dynamic_allocators: (),
}


mod stack_registry;
mod object_pool_registry;