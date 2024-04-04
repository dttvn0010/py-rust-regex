use std::sync::Arc;

use pyo3::prelude::*;
use regex::Regex;

#[pyclass]
#[derive(Clone)]
struct Range{
    #[pyo3(get)]
    start: usize,
    #[pyo3(get)]
    end: usize
}

#[pyclass]
struct Match {
    #[pyo3(get)]
    len: usize,
    #[pyo3(get)]
    range: Range
}

impl Match {
    pub fn new(m: &regex::Match) -> Self {
        Self{
            len: m.len(),
            range :Range {
                start: m.start(),
                end: m.end()
            }
        }
    }
}

#[pyclass]
struct RegexWrapper {
    regex: Arc<Regex>
}

#[pymethods]
impl RegexWrapper {
    fn is_match(&self, st: String) -> bool {
        self.regex.is_match(&st)
    }

    fn is_match_at(&self, st: String, start: usize) -> bool {
        self.regex.is_match_at(&st, start)
    }

    fn find(&self, st: String) -> Option<Py<Match>>{
        Python::with_gil(|py|{
            if let Some(m) = self.regex.find(&st) {
                return Py::new(py, Match::new(&m)).ok();
            }else{
                return None;
            }
        })
    }

    fn find_at(&self, st: String, start: usize) -> Option<Py<Match>> {
        Python::with_gil(|py|{
            if let Some(m) = self.regex.find_at(&st, start) {
                return Py::new(py, Match::new(&m)).ok();
            }else{
                return None;
            }
        })
    }

    fn find_all(&self, st: String) -> Vec<Py<Match>> {
        Python::with_gil(|py|{
            let mut vec = Vec::new();
            let matches = self.regex.find_iter(&st);
            for m in matches {
                if let Some(item) = Py::new(py, Match::new(&m)).ok() {
                    vec.push(item);
                }
            }
            return vec;
        })
    }

    fn replace(&self, st: String, rep: String) -> String {
        let res = self.regex.replace(&st, &rep);
        res.into_owned()
    }

    fn replace_all(&self, st: String, rep: String) -> String {
        let res = self.regex.replace_all(&st, &rep);
        res.into_owned()
    }
}

#[pyfunction]
fn new_regex(pattern: String) -> PyResult<RegexWrapper> {
    Ok(
        RegexWrapper{
            regex: Arc::new(Regex::new(&pattern).unwrap())
        }
    )
}

#[pymodule]
fn py_rust_regex(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(new_regex, m)?)?;
    m.add_class::<RegexWrapper>()?;
    Ok(())
}
