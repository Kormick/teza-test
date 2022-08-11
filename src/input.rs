//! Graph input node implementation.

use std::{cell::RefCell, rc::Rc};

use crate::node::Computable;

/// Trait definition for inputable types.
pub trait Input: Computable {
    /// Sets new input value.
    fn set(&mut self, val: f32);
}

/// Graph input node implementation.
#[derive(Clone)]
pub struct InputNode {
    val: f32,
    /// Holds references to nodes that depend from this node.
    dependencies: Vec<Rc<RefCell<dyn Computable>>>,
}

impl InputNode {
    pub fn from_val(val: f32) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            val,
            dependencies: Vec::default(),
        }))
    }
}

impl Computable for InputNode {
    /// Just returns stored value.
    fn compute(&mut self) -> f32 {
        self.val
    }

    /// Adds dependency from another `Computable` object.
    fn add_dependency(&mut self, dependency: Rc<RefCell<dyn Computable>>) {
        self.dependencies.push(dependency)
    }

    /// Doesn't have cache, so doing nothing.
    fn reset_cache(&mut self) {}
}

impl Input for InputNode {
    /// Sets new input value to node and resets cache for all dependable nodes.
    fn set(&mut self, val: f32) {
        self.val = val;
        self.dependencies
            .iter()
            .for_each(|d| d.borrow_mut().reset_cache())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn set_compute() {
        let mut x = InputNode {
            val: 42.0,
            dependencies: Vec::default(),
        };
        assert_eq!(x.compute(), 42.0);

        x.set(43.0);
        assert_eq!(x.compute(), 43.0);
    }

    #[test]
    fn set_reset_dependencies() {
        struct S {
            cache: Option<f32>,
        }

        impl Computable for S {
            fn compute(&mut self) -> f32 {
                0.0
            }

            fn reset_cache(&mut self) {
                self.cache = None;
            }

            fn add_dependency(&mut self, _dependency: Rc<RefCell<dyn Computable>>) {}
        }

        let cached = Rc::new(RefCell::new(S { cache: Some(1.0) }));
        let mut x = InputNode {
            val: 42.0,
            dependencies: Vec::default(),
        };

        x.add_dependency(cached.clone());
        assert!(!x.dependencies.is_empty());
        assert_eq!(cached.borrow().cache, Some(1.0));

        x.set(43.0);
        assert!(!x.dependencies.is_empty());
        assert_eq!(cached.borrow().cache, None);
    }
}
