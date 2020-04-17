use std::cmp::Ordering;
use std::ops::{AddAssign};

use pyo3::{
    mapping::{PyMappingProtocol, PyMappingLenProtocol},
    prelude::*,
};


#[pyclass]
struct Tree {
    root: Link,
    len: usize,
}

#[pymethods]
impl Tree {
    #[new]
    fn new() -> Self {
        Tree { root: None, len: 0 }
    }
}


#[pymethods]
impl Tree {
    pub fn insert(&mut self, element: i64) {
        let inserted = match self.root {
            Some(ref mut node) => node.insert(element),
            _ => {
                self.root = Some(Box::new(Node::new(element)));
                true
            }
        };

        if inserted {
            self.len.add_assign(1);
        }
    }

    pub fn get(&self, element: i64) -> Option<i64> {
        match self.root {
            Some(ref node) => node.get(element),
            _ => None,
        }
    }
}


type Link = Option<Box<Node>>;


#[pyclass]
#[derive(Clone, Debug, PartialEq, Default)]
struct Node {
    element: i64,
    left: Link,
    right: Link,
}


// pub struct IntoIter(Tree);
//
//
// pub struct Iter<'a> {
//     next: Option<&'a Node>,
// }
//
// pub struct IterMut<'a> {
//     next: Option<&'a mut Node>,
// }


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


impl<'p> PyMappingLenProtocol<'p> for Tree {
    type Result = PyResult<usize>;
}

impl<'p> PyMappingProtocol<'p> for Tree {
    fn __len__(&'p self) -> <Self as PyMappingLenProtocol>::Result {
        Ok(self.len)
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

    pub fn insert(&mut self, new_val: i64) -> bool {
        if self.element == new_val {
            return false;
        }

        let target_node = if new_val < self.element { &mut self.left } else { &mut self.right };
        match target_node {
            &mut Some(ref mut subnode) => { subnode.insert(new_val); },
            &mut None => {
                let new_node = Node { element: new_val, left: None, right: None };
                let boxed_node = Some(Box::new(new_node));
                *target_node = boxed_node;
            }
        }

        return true;
    }

    pub fn get(&self, element: i64) -> Option<i64> {
        match element.cmp(&self.element) {
            Ordering::Less =>
                match self.left {
                    Some(ref left) => left.get(element),
                    _ => None,
                }
            Ordering::Equal => Some(self.element),
            Ordering::Greater =>
                match self.right {
                    Some(ref right) => right.get(element),
                    _ => None,
                }
        }
    }
}


#[pymodule]
fn py_binary_tree(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add("__version__", env!("CARGO_PKG_VERSION"))?;
    m.add_class::<Tree>()?;
    m.add_class::<Node>()?;

    Ok(())
}
