// Copyright (c) Microsoft Corporation.
// Licensed under the MIT license.

#![allow(non_camel_case_types)]

//==============================================================================
// Imports
//==============================================================================

use ::libc::{
    c_void,
    sockaddr_in,
};

//==============================================================================
// Constants
//==============================================================================

/// Maximum Length for Scatter-Gather Arrays
pub const DMTR_SGARRAY_MAXLEN: usize = 1;

//==============================================================================
// Structures
//==============================================================================

/// Scatter-Gather Array Segment
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmtr_sgaseg_t {
    /// Underlying data.
    pub sgaseg_buf: *mut c_void,
    /// Length of underlying data.
    pub sgaseg_len: u32,
}

/// Scatter-Gather Array
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmtr_sgarray_t {
    pub sga_buf: *mut c_void,
    pub sga_numsegs: u32,
    pub sga_segs: [dmtr_sgaseg_t; DMTR_SGARRAY_MAXLEN],
    pub sga_addr: sockaddr_in,
}
