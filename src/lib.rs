use pyo3::prelude::*;

use wrapper::*;

pub mod wrapper;
pub mod pbytes;

#[pymodule]
fn _silk(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(encode, m)?)?;
    m.add_function(wrap_pyfunction!(decode, m)?)?;
    m.add("SilkError", _py.get_type::<wrapper::SilkError>())?;
    Ok(())
}

// #[cfg(test)]
// mod tests {
//     #[test]
//     fn it_works() {
//         let result = 2 + 2;
//         assert_eq!(result, 4);
//     }
// }
