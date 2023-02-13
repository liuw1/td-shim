// Copyright (c) 2021 Intel Corporation
//
// SPDX-License-Identifier: BSD-2-Clause-Patent

// Auto-generated by `td-layout-config`, do not edit manually.

/*
    Mem Layout:
                                            Address
                    +--------------+ <-  0x0
                    |     Legacy   |
                    +--------------+ <-  0x00100000 (1M)
                    |   ........   |
                    +--------------+ <-  0x00800000
                    |    TD HOB    |
                    +--------------+ <-  0x00900000
                    | KERNEL PARAM |    (0x00001000)
                    +--------------+ <-  0x00901000
                    |    KERNEL    |    (0x02000000)
                    +--------------+
                    |   ........   |
                    +--------------+ <-  0x7DD9E000
                    |  UNACCEPTED  |    (0x00040000)
                    +--------------+ <-  0x7DDDE000
                    |     ACPI     |    (0x00100000)
                    +--------------+ <-  0x7DEDE000
                    |    PAYLOAD   |    (0x02000000)
                    +--------------+ <-  0x7FEDE000
                    |  Page Table  | <-  0x00020000
                    +--------------+ <-  0x7FEFE000
                    |              |    // (0 - 0x7FF, mailbox header and OS)
                    |    MAILBOX   |    // (0x800 - 0xBFF, BSP - AP communication)
                    | (0x00002000) |    // (0xC00 - 0xFFF, IDT and an exception hander)
                    |              |    // (0x1000 - 0x1FFF, AP loop function)
                    +--------------+ <-  0x7FF00000
                    | TD_EVENT_LOG |    (0x00100000)
                    +--------------+ <-  0x80000000 (2G) - for example
*/

pub const TD_PAYLOAD_EVENT_LOG_SIZE: u32 = 0x100000;
pub const TD_PAYLOAD_MAILBOX_SIZE: u32 = 0x2000;
pub const TD_PAYLOAD_PAGE_TABLE_SIZE: u32 = 0x20000;
pub const TD_PAYLOAD_ACPI_SIZE: u32 = 0x100000;
pub const TD_PAYLOAD_SIZE: u32 = 0x2000000;
pub const TD_PAYLOAD_UNACCEPTED_MEMORY_BITMAP_SIZE: u32 = 0x40000;

pub const TD_HOB_BASE: u64 = 0x800000;
pub const TD_HOB_SIZE: u64 = 0x20000;
pub const KERNEL_PARAM_BASE: u64 = 0x900000;
pub const KERNEL_PARAM_SIZE: u64 = 0x1000;
pub const KERNEL_BASE: u64 = 0x901000;
pub const KERNEL_SIZE: usize = 0x2000000;
