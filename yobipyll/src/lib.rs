#![feature(proc_macro, specialization)]

extern crate libyobicash;
extern crate pyo3;

use pyo3::prelude::*;

///
/// Low-level python binding for libyobicash
///
/// You should NOT use this directly, it is intended as a simple glue layer between the rust and python code with minimal convenience functions
///
#[py::modinit(libyobipyll)]
fn init_mod(py: Python, m: &PyModule) -> PyResult<()> {
    
    #[pyfn(m, "get_libyobicash_version")]
    fn get_libyobicash_version_py() -> PyResult<String> {
       let out = libyobicash::VERSION.to_string();
       Ok(out)
    }

    Ok(())
}
