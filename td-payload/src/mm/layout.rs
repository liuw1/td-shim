// Copyright (c) 2022 Intel Corporation
//
// SPDX-License-Identifier: BSD-2-Clause-Patent

pub const DEFAULT_HEAP_SIZE: usize = 0x1000000;
pub const DEFAULT_STACK_SIZE: usize = 0x800000;
pub const DEFAULT_PAGE_TABLE_SIZE: usize = 0x800000;
#[cfg(not(feature = "no-shared-mem"))]
pub const DEFAULT_SHARED_MEMORY_SIZE: usize = 0x100000;
#[cfg(feature = "cet-shstk")]
pub const DEFAULT_SHADOW_STACK_SIZE: usize = 0x10000;

#[derive(Debug)]
pub struct RuntimeLayout {
    pub heap_size: usize,
    pub stack_size: usize,
    pub page_table_size: usize,
    #[cfg(not(feature = "no-shared-mem"))]
    pub shared_memory_size: usize,
    #[cfg(feature = "cet-shstk")]
    pub shadow_stack_size: usize,
}

impl Default for RuntimeLayout {
    fn default() -> Self {
        Self {
            heap_size: DEFAULT_HEAP_SIZE,
            stack_size: DEFAULT_STACK_SIZE,
            page_table_size: DEFAULT_PAGE_TABLE_SIZE,
            #[cfg(not(feature = "no-shared-mem"))]
            shared_memory_size: DEFAULT_SHARED_MEMORY_SIZE,
            #[cfg(feature = "cet-shstk")]
            shadow_stack_size: DEFAULT_SHADOW_STACK_SIZE,
        }
    }
}
