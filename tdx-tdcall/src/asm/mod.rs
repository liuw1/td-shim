// Copyright (c) 2020-2025 Intel Corporation
//
// SPDX-License-Identifier: BSD-2-Clause-Patent

use core::arch::global_asm;
use core::ffi::c_void;

#[cfg(feature = "use_tdx_emulation")]
global_asm!(include_str!("tdcall_emu.asm"));

#[cfg(all(feature = "use_tdx_emulation", not(feature = "no-tdvmcall")))]
global_asm!(include_str!("tdvmcall_emu.asm"));

#[cfg(not(feature = "use_tdx_emulation"))]
global_asm!(include_str!("tdcall.asm"));

#[cfg(all(not(feature = "use_tdx_emulation"), not(feature = "no-tdvmcall")))]
global_asm!(include_str!("tdvmcall.asm"));

#[cfg(all(not(feature = "use_tdx_emulation"), not(feature = "no-tdvmcall")))]
global_asm!(include_str!("tdvmcall_ex.asm"));

extern "win64" {
    pub(crate) fn asm_td_call(args: *mut c_void) -> u64;
    #[cfg(not(feature = "no-tdvmcall"))]
    pub(crate) fn asm_td_vmcall(args: *mut c_void, do_sti: u64) -> u64;
    #[cfg(not(feature = "no-tdvmcall"))]
    pub(crate) fn asm_td_vmcall_ex(args: *mut c_void, do_sti: u64) -> u64;
}
