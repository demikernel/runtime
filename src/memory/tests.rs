// Copyright (c) Microsoft Corporation.
// Licensed under the MIT license.

use super::Buffer;
use super::Bytes;
use std::sync::Arc;

/// Tests for buffer adjust.
#[test]
fn buf_adjust() {
    let data: [u8; 4] = [1, 2, 3, 4];
    let mut buf = Bytes::new(Some(Arc::new(data)), 0, 4);
    buf.adjust(2);
    assert_eq!(*buf, data[2..]);
}

/// Tests for buffer trim.
#[test]
fn buf_trim() {
    let data: [u8; 4] = [1, 2, 3, 4];
    let mut buf = Bytes::new(Some(Arc::new(data)), 0, 4);
    buf.trim(2);
    assert_eq!(*buf, data[..2]);
}
