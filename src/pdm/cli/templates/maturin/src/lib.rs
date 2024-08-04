use pyo3::prelude::*;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn hello(name: String) -> PyResult<String> {
    Ok(format!("Hello {name}"))
}

/// A Python module implemented in Rust.
#[pymodule]
fn _rswrapper(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(hello, m)?)?;
    Ok(())
}
