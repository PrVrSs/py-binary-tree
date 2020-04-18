use std::cmp::Ordering;
use std::collections::VecDeque;
use std::mem;
use std::ops::{AddAssign};

use pyo3::{
    basic::{PyObjectProtocol},
    mapping::{PyMappingProtocol},
    prelude::*,
};


#[pyclass]
pub struct Tree {
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


impl Drop for Tree {
    fn drop(&mut self) {
        let mut vector = VecDeque::with_capacity(self.len * 2);

        vector.push_back(self.root.take());

        while let Some(Some(mut boxed_node)) = vector.pop_front() {
            vector.push_back(boxed_node.left.take());
            vector.push_back(boxed_node.right.take());
        }
    }
}


type Link = Option<Box<Node>>;


#[pyclass]
#[derive(Clone, Debug, PartialEq, Default)]
pub struct Node {
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


#[pyproto]
impl PyObjectProtocol for Tree {
    fn __repr__(&self) -> PyResult<String> {
        Ok(format!(
            "BST({})",
            match self.root {
                Some(ref node) => node.show(),
                _ => "".to_string(),
            }
        ))
    }
}


#[pyproto]
impl PyMappingProtocol for Tree {
    fn __len__(&'p self) -> PyResult<usize> {
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
        match *target_node {
            Some(ref mut subnode) => { subnode.insert(new_val); },
            _ => {
                let new_node = Node { element: new_val, left: None, right: None };
                mem::replace(target_node, Some(Box::new(new_node)));
            }
        }

        true
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

impl Node {
    fn show(&self) -> String {
        let left = self.leaf_element(&self.left);
        let right = self.leaf_element(&self.right);

        format!("Node({} -> left({}) right({}))", self.element, left, right)
    }

    fn leaf_element(&self, leaf: &Link) -> String {
        match *leaf {
            Some(ref l) => l.show(),
            _ => "None".to_string(),
        }
    }
}
