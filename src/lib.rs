mod tree;

use pyo3::prelude::*;


#[pymodule]
fn py_binary_tree(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add("__version__", env!("CARGO_PKG_VERSION"))?;
    m.add_class::<tree::Tree>()?;
    m.add_class::<tree::Node>()?;

    Ok(())
}
