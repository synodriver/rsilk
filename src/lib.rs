use pyo3::prelude::*;

use wrapper::*;

mod wrapper;

#[pymodule]
fn _silk(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(encode, m)?)?;
    m.add_function(wrap_pyfunction!(decode, m)?)?;
    m.add("SilkError", _py.get_type::<wrapper::SilkError>())?;
    Ok(())
}
