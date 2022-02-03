// Copyright (c) Microsoft Corporation.
// Licensed under the MIT license.

#![allow(non_camel_case_types)]

//==============================================================================
// Imports
//==============================================================================

use ::libc::{c_void, sockaddr_in};

//==============================================================================
// Constants
//==============================================================================

/// Maximum Length for Scatter-Gather Arrays
///
/// TODO: Rename this to `DMTR_SGARRAY_MAXLEN`
pub const DMTR_SGARRAY_MAXSIZE: usize = 1;

//==============================================================================
// Structures
//==============================================================================

/// Scatter-Gather Segment
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dmtr_sgaseg_t {
    pub sgaseg_buf: *mut c_void,
    pub sgaseg_len: u32,
}

/// Scatter-Gather Array
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dmtr_sgarray_t {
    pub sga_buf: *mut c_void,
    pub sga_numsegs: u32,
    pub sga_segs: [dmtr_sgaseg_t; DMTR_SGARRAY_MAXSIZE],
    pub sga_addr: sockaddr_in,
}
