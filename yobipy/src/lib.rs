#![feature(proc_macro, specialization)]

extern crate libyobicash;
extern crate pyo3;

use pyo3::prelude::*;

#[py::modinit(yobipy)]
fn init_mod(py: Python, m: &PyModule) -> PyResult<()> {
    Ok(())
}
