use pyo3::prelude::*;
use pyo3::exceptions;
use pyo3::types::PyBytes;
use pyo3::create_exception;

use silk_rs;

create_exception!(_silk, SilkError, exceptions::PyException);

/// encode(input: bytes, sample_rate: int, bit_rate: int, max_internal_sample_rate: int = 24000, packet_loss_percentage: int = 0, complexity: int = 2, use_inband_fec: bool = False, use_dtx: bool = False, tencent: bool = True) -> bytes
///
/// encode pcm to silk
#[pyfunction(max_internal_sample_rate = "24000", packet_loss_percentage = "0", complexity = "2", use_inband_fec = "false", use_dtx = "false", tencent = "true")]
#[pyo3(text_signature = "(input: bytes, sample_rate: int, bit_rate: int, packet_loss_percentage: int = 0, complexity: int = 2, use_inband_fec: bool = False, use_dtx: bool = False, tencent: bool = True) -> bytes")]
pub fn encode<'a>(py: Python, input: &'a [u8], sample_rate: i32, bit_rate: i32, max_internal_sample_rate: i32, packet_loss_percentage: i32, complexity: i32, use_inband_fec: bool, use_dtx: bool, tencent: bool) -> PyResult<PyObject> {
    match silk_rs::encode_silk(input, sample_rate, bit_rate, max_internal_sample_rate, packet_loss_percentage, complexity, use_inband_fec, use_dtx, tencent) {
        Ok(data) => Ok(PyBytes::new(py, &data).into()),
        Err(e) => Err(SilkError::new_err(e.to_string())),
    }
}

/// decode(src: bytes, sample_rate: int, frame_size: int = 0, frames_per_packet: int = 1, more_internal_decoder_frames: bool = False, in_band_fec_offset: int = 0, loss: bool = False) -> bytes
///
/// decode silk to pcm
#[pyfunction(frame_size = "0", frames_per_packet = "1", more_internal_decoder_frames = "false", in_band_fec_offset = "0", loss = "false")]
#[pyo3(text_signature = "(src: bytes, sample_rate: int, frame_size: int = 0, frames_per_packet: int = 1, more_internal_decoder_frames: bool = False, in_band_fec_offset: int = 0, loss: bool = False) -> bytes")]
pub fn decode<'a>(py: Python, src: &'a [u8], sample_rate: i32, frame_size: i32, frames_per_packet: i32, more_internal_decoder_frames: bool, in_band_fec_offset: i32, loss: bool) -> PyResult<PyObject> {
    match silk_rs::decode_silk(src, sample_rate, frame_size, frames_per_packet, more_internal_decoder_frames, in_band_fec_offset, loss) {
        Ok(data) => Ok(PyBytes::new(py, &data).into()),
        Err(e) => Err(SilkError::new_err(e.to_string())),
    }
}