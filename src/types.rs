// Copyright (c) Microsoft Corporation.
// Licensed under the MIT license.

#![allow(non_camel_case_types)]

use libc::{c_int, c_void, sockaddr_in};

pub type dmtr_qtoken_t = u64;

pub const DMTR_SGARRAY_MAXSIZE: usize = 1;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct dmtr_sgaseg_t {
    pub sgaseg_buf: *mut c_void,
    pub sgaseg_len: u32,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct dmtr_sgarray_t {
    pub sga_buf: *mut c_void,
    pub sga_numsegs: u32,
    pub sga_segs: [dmtr_sgaseg_t; DMTR_SGARRAY_MAXSIZE],
    pub sga_addr: sockaddr_in,
}

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

#[derive(Copy, Clone)]
#[repr(C)]
pub struct dmtr_accept_result_t {
    pub qd: c_int,
    pub addr: sockaddr_in,
}

#[repr(C)]
pub union dmtr_qr_value_t {
    pub sga: dmtr_sgarray_t,
    pub ares: dmtr_accept_result_t,
}

#[repr(C)]
pub struct dmtr_qresult_t {
    pub qr_opcode: dmtr_opcode_t,
    pub qr_qd: c_int,
    pub qr_qt: dmtr_qtoken_t,
    pub qr_value: dmtr_qr_value_t,
}
