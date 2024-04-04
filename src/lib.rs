use pyo3::prelude::*;
use regex::Regex;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

#[pyfunction]
fn is_match(pattern: String, st: String) -> PyResult<bool> {
    let re = Regex::new(&pattern).unwrap();
    return Ok(re.is_match(&st));
}

/// A Python module implemented in Rust.
#[pymodule]
fn py_rust_regex(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_function(wrap_pyfunction!(is_match, m)?)?;
    Ok(())
}
