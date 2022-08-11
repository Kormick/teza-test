//! Graph expression node implementation.

use std::{cell::RefCell, rc::Rc};

use crate::ops::Operation;

/// Trait definition ofr computable types.
pub trait Computable {
    /// Computes result of this type.
    fn compute(&mut self) -> f32;
    /// Adds dependency from another `Computable` object.
    fn add_dependency(&mut self, dependency: Rc<RefCell<dyn Computable>>);
    /// Resets cache for this node.
    fn reset_cache(&mut self);
}

/// Graph expression node implementation.
#[derive(Clone)]
pub struct Node {
    /// Cached result.
    cache: Option<f32>,
    /// Holds references to nodes that depend from this node.
    dependencies: Vec<Rc<RefCell<dyn Computable>>>,
    /// Holds operation for this node.
    opp: Operation,
}

impl Node {
    /// Builds `Node` from given `Operation`.
    fn from_opp(opp: Operation) -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Self {
            cache: None,
            dependencies: Vec::default(),
            opp,
        }))
    }

    /// Builds `Node` for sum of two nodes.
    pub fn add(
        x: Rc<RefCell<dyn Computable>>,
        y: Rc<RefCell<dyn Computable>>,
    ) -> Rc<RefCell<Node>> {
        let obj = Self::from_opp(Operation::Add(x.clone(), y.clone()));
        x.borrow_mut().add_dependency(obj.clone());
        y.borrow_mut().add_dependency(obj.clone());
        obj
    }

    /// Builds `Node` for sum of variable amount of nodes.
    pub fn add_var(args: Vec<Rc<RefCell<dyn Computable>>>) -> Rc<RefCell<Node>> {
        let obj = Self::from_opp(Operation::AddVar(args.clone()));
        args.iter()
            .for_each(|arg| arg.borrow_mut().add_dependency(obj.clone()));
        obj
    }

    /// Builds `Node` for subtraction node of two nodes.
    pub fn sub(
        x: Rc<RefCell<dyn Computable>>,
        y: Rc<RefCell<dyn Computable>>,
    ) -> Rc<RefCell<Node>> {
        let obj = Self::from_opp(Operation::Sub(x.clone(), y.clone()));
        x.borrow_mut().add_dependency(obj.clone());
        y.borrow_mut().add_dependency(obj.clone());
        obj
    }

    /// Builds `Node` for multiplication of two nodes.
    pub fn mul(
        x: Rc<RefCell<dyn Computable>>,
        y: Rc<RefCell<dyn Computable>>,
    ) -> Rc<RefCell<Node>> {
        let obj = Self::from_opp(Operation::Mul(x.clone(), y.clone()));
        x.borrow_mut().add_dependency(obj.clone());
        y.borrow_mut().add_dependency(obj.clone());
        obj
    }

    /// Builds `Node` for exponentiation of node to given exponent.
    pub fn pow(x: Rc<RefCell<dyn Computable>>, pow: f32) -> Rc<RefCell<Node>> {
        let obj = Self::from_opp(Operation::Pow(x.clone(), pow));
        x.borrow_mut().add_dependency(obj.clone());
        obj
    }

    /// Builds `Node` for sin value of given node.
    pub fn sin(x: Rc<RefCell<dyn Computable>>) -> Rc<RefCell<Node>> {
        let obj = Self::from_opp(Operation::Sin(x.clone()));
        x.borrow_mut().add_dependency(obj.clone());
        obj
    }
}

impl Computable for Node {
    /// Returns computation result of this node.
    /// Takes cached value if available, otherwise computes the result and stores it in cache.
    fn compute(&mut self) -> f32 {
        self.cache.unwrap_or({
            let val = self.opp.compute();
            self.cache = Some(val);
            val
        })
    }

    /// Adds dependency from another `Computable` object.
    fn add_dependency(&mut self, dependency: Rc<RefCell<dyn Computable>>) {
        self.dependencies.push(dependency);
    }

    /// Resets cache for this node and all the dependable nodes.
    fn reset_cache(&mut self) {
        self.cache = None;
        self.dependencies
            .iter()
            .for_each(|d| d.borrow_mut().reset_cache());
    }
}

#[cfg(test)]
mod tests {
    use crate::input::{Input, InputNode};

    use super::*;

    fn round(x: f32, precision: u32) -> f32 {
        let m = 10i32.pow(precision) as f32;
        (x * m).round() / m
    }

    fn check_node(node: Rc<RefCell<Node>>, expected: f32) {
        assert_eq!(node.borrow().cache, None);
        let result = round(node.borrow_mut().compute(), 5);
        assert_eq!(result, round(expected, 5));
        assert_eq!(node.borrow().cache.map(|v| round(v, 5)), Some(expected));
    }

    #[test]
    fn add() {
        let x1 = InputNode::from_val(1.0);
        let x2 = InputNode::from_val(2.0);
        let x = Node::add(x1.clone(), x2.clone());
        check_node(x.clone(), 3.0);

        x1.borrow_mut().set(3.0);
        x2.borrow_mut().set(4.0);
        check_node(x, 7.0);
    }

    #[test]
    fn add_var() {
        let x1 = InputNode::from_val(1.0);
        let x2 = InputNode::from_val(2.0);
        let x3 = InputNode::from_val(3.0);
        let args: Vec<Rc<RefCell<dyn Computable>>> = vec![x1.clone(), x2.clone(), x3.clone()];
        let x = Node::add_var(args.clone());
        check_node(x.clone(), 6.0);

        x1.borrow_mut().set(4.0);
        x2.borrow_mut().set(5.0);
        x3.borrow_mut().set(6.0);
        check_node(x, 15.0);
    }

    #[test]
    fn sub() {
        let x1 = InputNode::from_val(1.0);
        let x2 = InputNode::from_val(2.0);
        let x = Node::sub(x1.clone(), x2.clone());
        check_node(x.clone(), -1.0);

        x1.borrow_mut().set(2.0);
        x2.borrow_mut().set(1.0);
        check_node(x, 1.0);
    }

    #[test]
    fn mul() {
        let x1 = InputNode::from_val(2.0);
        let x2 = InputNode::from_val(3.0);
        let x = Node::mul(x1.clone(), x2.clone());
        check_node(x.clone(), 6.0);

        x1.borrow_mut().set(4.0);
        x2.borrow_mut().set(5.0);
        check_node(x, 20.0);
    }

    #[test]
    fn pow() {
        let x1 = InputNode::from_val(2.0);
        let x = Node::pow(x1.clone(), 3.0);
        check_node(x.clone(), 8.0);

        x1.borrow_mut().set(3.0);
        check_node(x, 27.0);
    }

    #[test]
    fn sin() {
        let x1 = InputNode::from_val(std::f32::consts::PI);
        let x = Node::sin(x1.clone());
        check_node(x.clone(), 0.0);

        x1.borrow_mut().set(std::f32::consts::FRAC_PI_2);
        check_node(x, 1.0);
    }

    #[test]
    fn nested_expression() {
        let x1 = InputNode::from_val(1.0);
        let x2 = InputNode::from_val(2.0);
        let x3 = InputNode::from_val(3.0);
        let x = Node::add(
            x1.clone(),
            Node::mul(x2.clone(), Node::pow(x3.clone(), 2.0)),
        );
        check_node(x.clone(), 19.0);

        x1.borrow_mut().set(4.0);
        x2.borrow_mut().set(5.0);
        x3.borrow_mut().set(6.0);
        check_node(x, 184.0);
    }

    #[test]
    fn expression_with_same_input() {
        let x1 = InputNode::from_val(2.0);
        let x = Node::add(
            x1.clone(),
            Node::add_var(vec![
                x1.clone(),
                x1.clone(),
                Node::sub(
                    x1.clone(),
                    Node::mul(x1.clone(), Node::pow(Node::sin(x1.clone()), 3.0)),
                ),
            ]),
        );
        check_node(x.clone(), 6.49635);

        x1.borrow_mut().set(3.0);
        check_node(x, 11.99157);
    }

    #[test]
    fn custom_input() {
        struct Custom {
            val: f32,
        }

        impl Computable for Custom {
            fn compute(&mut self) -> f32 {
                self.val
            }

            fn add_dependency(&mut self, _dependency: Rc<RefCell<dyn Computable>>) {}

            fn reset_cache(&mut self) {}
        }

        impl Input for Custom {
            fn set(&mut self, val: f32) {
                self.val = val;
            }
        }

        let x1 = Rc::new(RefCell::new(Custom { val: 2.0 }));
        let x2 = InputNode::from_val(3.0);
        let x = Node::add(x1.clone(), x2.clone());
        check_node(x.clone(), 5.0);

        x1.borrow_mut().set(3.0);
        x2.borrow_mut().set(4.0);
        check_node(x, 7.0);
    }

    #[test]
    fn custom_node() {
        struct Custom {
            val: f32,
        }

        impl Computable for Custom {
            fn compute(&mut self) -> f32 {
                self.val
            }

            fn add_dependency(&mut self, _dependency: Rc<RefCell<dyn Computable>>) {}

            fn reset_cache(&mut self) {}
        }

        let x1 = InputNode::from_val(2.0);
        let x2 = Rc::new(RefCell::new(Custom { val: 3.0 }));
        let x = Node::add(x1.clone(), x2);
        check_node(x.clone(), 5.0);

        x1.borrow_mut().set(4.0);
        check_node(x, 7.0);
    }
}
