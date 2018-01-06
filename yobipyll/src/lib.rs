#![feature(proc_macro, specialization)]

extern crate libyobicash;
extern crate pyo3;

pub mod utils;

// include build metadata
pub mod built_info {
   include!(concat!(env!("OUT_DIR"), "/built.rs"));
}


use pyo3::prelude::*;
https://github.com/PyO3/pyo3/blob/master/src/class/macros.rs
///
/// Low-level python binding for libyobicash
///
/// You should NOT use this directly, it is intended as a simple glue layer between the rust and python code with minimal convenience functions
///
#[py::modinit(libyobipyll)]
fn init_mod(py: Python, m: &PyModule) -> PyResult<()> {
    
    #[pyfn(m, "get_ll_version")]
    fn get_ll_version_py() -> PyResult<String> {
       let out = built_info::PKG_VERSION.to_string();
       Ok(out)
    }

    #[pyfn(m, "get_libyobicash_version")]
    fn get_libyobicash_version_py() -> PyResult<String> {
       let out = libyobicash::VERSION.to_string();
       Ok(out)
    }

    m.add_class::<utils::UtilsAPI>()?;

    Ok(())
}
