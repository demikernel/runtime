// Copyright (c) Microsoft Corporation.
// Licensed under the MIT license.

#![allow(non_camel_case_types)]

//==============================================================================
// Imports
//==============================================================================

use crate::types::{
    memory::dmtr_sgarray_t,
    queue::dmtr_qtoken_t,
};
use ::libc::{
    c_int,
    sockaddr_in,
};

//==============================================================================
// Structures
//==============================================================================

/// Operation Code
#[repr(C)]
#[derive(Debug, Eq, PartialEq)]
pub enum dmtr_opcode_t {
    DMTR_OPC_INVALID = 0,
    DMTR_OPC_PUSH,
    DMTR_OPC_POP,
    DMTR_OPC_ACCEPT,
    DMTR_OPC_CONNECT,
    DMTR_OPC_FAILED,
}

/// Result for `accept()`
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmtr_accept_result_t {
    pub qd: c_int,
    pub addr: sockaddr_in,
}

#[repr(C)]
pub union dmtr_qr_value_t {
    pub sga: dmtr_sgarray_t,
    pub ares: dmtr_accept_result_t,
}

/// Result
#[repr(C)]
pub struct dmtr_qresult_t {
    pub qr_opcode: dmtr_opcode_t,
    pub qr_qd: c_int,
    pub qr_qt: dmtr_qtoken_t,
    pub qr_value: dmtr_qr_value_t,
}
