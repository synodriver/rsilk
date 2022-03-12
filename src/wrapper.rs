use pyo3::prelude::*;
use pyo3::exceptions;
use pyo3::create_exception;

use crate::pbytes::PBytes;
use silk_rs;

create_exception!(_silk, SilkError, exceptions::PyException);

/// encode(input: bytes, sample_rate: int, bit_rate: int, tencent: bool) -> bytes
///
/// encode pcm to silk
#[pyfunction]
pub fn encode(input: Vec<u8>, sample_rate: i32, bit_rate: i32, tencent: bool) -> PyResult<PBytes> {
    match silk_rs::encode_silk(input, sample_rate, bit_rate, tencent)
    {
        Ok(value) => Ok(PBytes::from(value)),
        Err(e) => Err(SilkError::new_err(e.to_string())),
    }
}

/// decode(src: bytes, sample_rate: int) -> bytes
///
/// decode silk to pcm
#[pyfunction]
pub fn decode(src: Vec<u8>, sample_rate: i32) -> PyResult<PBytes> {
    match silk_rs::decode_silk(src, sample_rate)
    {
        Ok(value) => Ok(PBytes::from(value)),
        Err(e) => Err(SilkError::new_err(e.to_string())),
    }
}