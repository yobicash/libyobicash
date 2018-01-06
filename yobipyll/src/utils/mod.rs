#![feature(proc_macro, specialization, const_fn)]

extern crate pyo3;
use pyo3::prelude::*;

#[py::class]
struct UtilsAPI { }

#[py::methods]
impl UtilsAPI {

  #[new]
  fn __new__(obj: &PyRawObject) -> PyResult<()> {
     obj.init(|t| UtilsAPI {})
  }
}
