// Copyright (c) Microsoft Corporation.
// Licensed under the MIT license.

mod memory;
mod ops;
mod queue;

//==============================================================================
// Exports
//==============================================================================

pub use self::{
    memory::{dmtr_sgarray_t, dmtr_sgaseg_t, DMTR_SGARRAY_MAXLEN},
    ops::{dmtr_accept_result_t, dmtr_opcode_t, dmtr_qr_value_t, dmtr_qresult_t},
    queue::dmtr_qtoken_t,
};
