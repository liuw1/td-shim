// Copyright (c) 2021 Intel Corporation
//
// SPDX-License-Identifier: BSD-2-Clause-Patent

use core::fmt;
use core::mem::{size_of, zeroed};
use core::ptr::{slice_from_raw_parts, slice_from_raw_parts_mut};
use scroll::{Pread, Pwrite};

use crate::{td_call, TdCallError, TdcallArgs, TDCALL_STATUS_SUCCESS, TDCALL_TDREPORT};

pub const TD_REPORT_SIZE: usize = 0x400;
pub const TD_REPORT_ADDITIONAL_DATA_SIZE: usize = 64;

#[repr(C)]
#[derive(Debug, Pread, Pwrite, Clone, Copy)]
pub struct ReportType {
    /// Trusted Execution Environment (TEE) type
    /// 0x81 - TDX
    pub r#type: u8,
    /// Type-specific subtype
    pub subtype: u8,
    /// Type-specific version
    pub version: u8,
    pub reserved: u8,
}

#[repr(C)]
#[derive(Debug, Pread, Pwrite, Clone, Copy)]
pub struct ReportMac {
    /// Type header structure
    pub report_type: ReportType,
    pub reserved0: [u8; 12],
    /// CPU SVN
    pub cpu_svn: [u8; 16],
    /// SHA384 of the TEE_TCB_INFO
    pub tee_tcb_info_hash: [u8; 48],
    /// SHA384 of the TEE_INFO (TDG.VP.INFO)
    pub tee_info_hash: [u8; 48],
    /// A set of data used for communication between the caller and the target
    pub report_data: [u8; 64],
    pub reserved1: [u8; 32],
    /// The MAC over the REPORTMACSTRUCT with model-specific MAC
    pub mac: [u8; 32],
}

impl fmt::Display for ReportMac {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Report MAC:\n\tReport Type:\n\ttype: {:x?}\tsubtype: {:x?}\
                        \tversion: {:x?}\n\tCPU SVN:\n\t{:x?}\n\
                        \tTEE TCB Info Hash:\n\t{:x?}\n\tTEE Info Hash:\n\t{:x?}\n\
                        \tReport Data:\n\t{:x?}\n\tMAC:\n\t{:x?}\n",
            self.report_type.r#type,
            self.report_type.subtype,
            self.report_type.version,
            self.cpu_svn,
            self.tee_tcb_info_hash,
            self.tee_info_hash,
            self.report_data,
            self.mac
        )
    }
}

#[repr(C)]
#[derive(Debug, Pread, Pwrite, Clone, Copy)]
pub struct TeeTcbInfo {
    pub valid: [u8; 8],
    pub tee_tcb_svn: [u8; 16],
    pub mrseam: [u8; 48],
    pub mrsigner_seam: [u8; 48],
    pub attributes: [u8; 8],
    /// Reserved. Must be zero
    pub reserved: [u8; 111],
}

impl fmt::Display for TeeTcbInfo {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "TEE TCB Info:\n\tValid:\n\t{:x?}\n\tTEE TCB SVN:\n\t{:x?}\n\
                        \tMR SEAM:\n\t{:x?}\n\tMR Signer SEAM:\n\t{:x?}\n\
                        \tAttributes:\n\t{:x?}\n",
            self.valid, self.tee_tcb_svn, self.mrseam, self.mrsigner_seam, self.attributes
        )
    }
}

/// Defined as the TDX-specific TEE_INFO part of the TDG.MR.REPORT.
/// Contains the measurements and initial configuration of the TD that
/// The output of the TDG.MR.REPORT function
#[repr(C)]
#[derive(Debug, Pread, Pwrite, Clone, Copy)]
pub struct TdInfo {
    /// TD's attributes
    pub attributes: [u8; 8],
    /// TD's XFAM
    pub xfam: [u8; 8],
    /// Measurement of the initial contents of the TD
    pub mrtd: [u8; 48],
    /// Software-defined ID for non-owner-defined configuration of
    /// the guest TD
    pub mrconfig_id: [u8; 48],
    /// Software-defined ID for the guest TD's owner
    pub mrowner: [u8; 48],
    /// Software-defined ID for owner-defined configuration of
    /// the guest TD
    pub mrownerconfig: [u8; 48],
    /// Runtime extendable measurement registers
    pub rtmr0: [u8; 48],
    pub rtmr1: [u8; 48],
    pub rtmr2: [u8; 48],
    pub rtmr3: [u8; 48],
    /// Reserved. Must be zero
    pub reserved: [u8; 112],
}

impl fmt::Display for TdInfo {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "TdInfo:\n\tAttributes:\n\t{:x?}\n\txfam:\n\t{:x?}\n\
                        \tMR TD:\n\t{:x?}\n\tMR Config ID:\n\t{:x?}\n\
                        \tMR Owner:\n\t{:x?}\n\tMR Owner Config:\n\t{:x?}\n\
                        \tRTMR[0]:\n\t{:x?}\n\tRTMR[1]:\n\t{:x?}\n\
                        \tRTMR[2]:\n\t{:x?}\n\tRTMR[3]:\n\t{:x?}\n",
            self.attributes,
            self.xfam,
            self.mrtd,
            self.mrconfig_id,
            self.mrowner,
            self.mrownerconfig,
            self.rtmr0,
            self.rtmr1,
            self.rtmr2,
            self.rtmr3
        )
    }
}

/// Known as the TDREPORT_STRUCT defined in TDX Module Spec.
/// The output of the TDG.MR.REPORT function
///
/// Detailed information can be found in TDX Module Spec section 'TDREPORT_STRUCT'
#[repr(C, packed)]
#[derive(Debug, Pread, Pwrite)]
pub struct TdxReport {
    pub report_mac: ReportMac,
    pub tee_tcb_info: TeeTcbInfo,
    pub reserved: [u8; 17],
    pub td_info: TdInfo,
}

impl fmt::Display for TdxReport {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "TDX Report:\n{}\n{}\n{}\n",
            self.report_mac, self.tee_tcb_info, self.td_info
        )
    }
}

impl TdxReport {
    pub fn as_bytes(&self) -> &[u8] {
        unsafe { &*slice_from_raw_parts(self as *const Self as *const u8, size_of::<Self>()) }
    }

    pub fn as_bytes_mut(&mut self) -> &mut [u8] {
        unsafe { &mut *slice_from_raw_parts_mut(self as *mut Self as *mut u8, size_of::<Self>()) }
    }
}

impl Default for TdxReport {
    fn default() -> Self {
        unsafe { zeroed() }
    }
}

#[repr(C, align(1024))]
struct TdxReportBuf(TdxReport);

#[repr(C, align(64))]
struct AdditionalDataBuf([u8; TD_REPORT_ADDITIONAL_DATA_SIZE]);

/// Create a TDREPORT_STRUCT structure that contains the measurements/configuration
/// information of the guest TD, measurements/configuration information of the Intel
/// TDX module and a REPORTMACSTRUCT
///
/// Details can be found in TDX module ABI spec section 'TDG.MR.REPORT'
pub fn tdcall_report(
    additional_data: &[u8; TD_REPORT_ADDITIONAL_DATA_SIZE],
) -> Result<TdxReport, TdCallError> {
    let mut report_buf = TdxReportBuf(TdxReport::default());
    let additional_data_buf = AdditionalDataBuf(*additional_data);

    let mut args = TdcallArgs {
        rax: TDCALL_TDREPORT,
        rcx: &mut report_buf as *mut _ as u64,
        rdx: &additional_data_buf as *const _ as u64,
        ..Default::default()
    };

    let ret = td_call(&mut args);
    if ret != TDCALL_STATUS_SUCCESS {
        return Err(args.r10.into());
    }

    Ok(report_buf.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tdx_report_size() {
        assert_eq!(size_of::<TdxReport>(), 0x400);
    }
}
