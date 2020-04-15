use pyo3::{
    basic::{PyObjectProtocol, PyObjectReprProtocol, PyObjectStrProtocol},
    mapping::{PyMappingProtocol, PyMappingLenProtocol},
    exceptions::RuntimeError,
    prelude::*,
};
use pyo3::wrap_pyfunction;


type Link = Option<Box<Node>>;


#[pyclass]
#[derive(Clone, Debug, PartialEq, Default)]
struct Node {
    element: i64,
    left: Link,
    right: Link,
}


// impl<'p> PyObjectReprProtocol<'p> for Node {
//     type Success = String;
//     type Result = PyResult<String>;
// }
//
// impl<'p> PyObjectStrProtocol<'p> for Node {
//     type Success = String;
//     type Result = PyResult<String>;
// }

// impl<'p> PyObjectProtocol<'p> for Node {
//     fn __repr__(&'p self) -> <Self as PyObjectReprProtocol>::Result {
//         Ok("Node".to_string())
//     }
//     fn __str__(&'p self) -> <Self as PyObjectStrProtocol>::Result {
//         self.__repr__()
//     }
// }


impl<'p> PyMappingLenProtocol<'p> for Node {
    type Result = PyResult<usize>;
}

impl<'p> PyMappingProtocol<'p> for Node {
    fn __len__(&'p self) -> <Self as PyMappingLenProtocol>::Result {
        Ok(2)
    }
}


#[pymethods]
impl Node {
    #[new]
    fn new(element: i64) -> Self {
        Node { element, left: None, right: None }
    }

    #[getter]
    fn element(&self) -> PyResult<i64> {
        Ok(self.element)
    }

    pub fn insert(&mut self, new_val: i64) {
        if self.element == new_val {
            return
        }

        let target_node = if new_val < self.element { &mut self.left } else { &mut self.right };
        match target_node {
            &mut Some(ref mut subnode) => subnode.insert(new_val),
            &mut None => {
                let new_node = Node { element: new_val, left: None, right: None };
                let boxed_node = Some(Box::new(new_node));
                *target_node = boxed_node;
            }
        }
    }
}


/// This module is a python module implemented in Rust.
#[pymodule]
fn py_binary_tree(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add("__version__", env!("CARGO_PKG_VERSION"))?;
    m.add_class::<Node>()?;

    Ok(())
}
