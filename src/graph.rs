use std::{cell::RefCell, rc::Rc};

use crate::ops::Ops;

pub trait Computable {
    fn compute(&mut self) -> f32;
}

#[derive(Debug, Clone)]
pub struct Node {
    cache: Option<f32>,
    deps: Vec<Rc<RefCell<Node>>>,
    val: Ops,
}

impl Node {
    fn from_ops(val: Ops) -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Node {
            cache: None,
            deps: Vec::default(),
            val,
        }))
    }

    pub fn input(x: f32) -> Rc<RefCell<Node>> {
        let obj = Self::from_ops(Ops::Input(x));
        obj.borrow_mut().cache = Some(x);
        obj
    }

    pub fn add(x: Rc<RefCell<Node>>, y: Rc<RefCell<Node>>) -> Rc<RefCell<Node>> {
        let obj = Self::from_ops(Ops::Add(x.clone(), y.clone()));
        x.borrow_mut().add_dep(obj.clone());
        y.borrow_mut().add_dep(obj.clone());
        obj
    }

    pub fn add_var(args: Vec<Rc<RefCell<Node>>>) -> Rc<RefCell<Node>> {
        let obj = Self::from_ops(Ops::AddVar(args.clone()));
        args.iter()
            .for_each(|arg| arg.borrow_mut().add_dep(obj.clone()));
        obj
    }

    pub fn sub(x: Rc<RefCell<Node>>, y: Rc<RefCell<Node>>) -> Rc<RefCell<Node>> {
        let obj = Self::from_ops(Ops::Sub(x.clone(), y.clone()));
        x.borrow_mut().add_dep(obj.clone());
        y.borrow_mut().add_dep(obj.clone());
        obj
    }

    pub fn mul(x: Rc<RefCell<Node>>, y: Rc<RefCell<Node>>) -> Rc<RefCell<Node>> {
        let obj = Self::from_ops(Ops::Mul(x.clone(), y.clone()));
        x.borrow_mut().add_dep(obj.clone());
        y.borrow_mut().add_dep(obj.clone());
        obj
    }

    pub fn pow(x: Rc<RefCell<Node>>, pow: f32) -> Rc<RefCell<Node>> {
        let obj = Self::from_ops(Ops::Pow(x.clone(), pow));
        x.borrow_mut().add_dep(obj.clone());
        obj
    }

    pub fn sin(x: Rc<RefCell<Node>>) -> Rc<RefCell<Node>> {
        let obj = Self::from_ops(Ops::Sin(x.clone()));
        x.borrow_mut().add_dep(obj.clone());
        obj
    }

    fn add_dep(&mut self, dep: Rc<RefCell<Node>>) {
        self.deps.push(dep)
    }

    fn reset_cache(&mut self) {
        self.cache = None;
        self.deps.iter().for_each(|d| d.borrow_mut().reset_cache())
    }

    pub fn set(&mut self, val: f32) {
        if let Ops::Input(_) = self.val {
            self.val.set(val);
            self.cache = Some(val);
            self.deps.iter().for_each(|d| d.borrow_mut().reset_cache())
        }
    }
}

impl Computable for Node {
    fn compute(&mut self) -> f32 {
        self.cache.unwrap_or({
            let val = self.val.compute();
            self.cache = Some(val);
            val
        })
    }
}

#[cfg(test)]
mod tests {
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
    fn compute_input() {
        let x = Node::input(2.0);
        assert_eq!(x.borrow().cache, Some(2.0));
        let result = x.borrow_mut().compute();
        assert_eq!(result, 2.0);
        assert_eq!(x.borrow().cache, Some(result));
    }

    #[test]
    fn compute_add() {
        let x1 = Node::input(1.0);
        let x2 = Node::input(2.0);
        let x = Node::add(x1.clone(), x2.clone());
        check_node(x.clone(), 3.0);

        x1.borrow_mut().set(3.0);
        x2.borrow_mut().set(4.0);
        check_node(x, 7.0);
    }

    #[test]
    fn compute_add_var() {
        let args = vec![Node::input(1.0), Node::input(2.0), Node::input(3.0)];
        let x = Node::add_var(args.clone());
        check_node(x.clone(), 6.0);

        args[0].borrow_mut().set(4.0);
        args[1].borrow_mut().set(5.0);
        args[2].borrow_mut().set(6.0);
        check_node(x, 15.0);
    }

    #[test]
    fn compute_sub() {
        let x1 = Node::input(1.0);
        let x2 = Node::input(2.0);
        let x = Node::sub(x1.clone(), x2.clone());
        check_node(x.clone(), -1.0);

        x1.borrow_mut().set(3.0);
        x2.borrow_mut().set(4.0);
        check_node(x, -1.0);
    }

    #[test]
    fn compute_mul() {
        let x1 = Node::input(2.0);
        let x2 = Node::input(3.0);
        let x = Node::mul(x1.clone(), x2.clone());
        check_node(x.clone(), 6.0);

        x1.borrow_mut().set(4.0);
        x2.borrow_mut().set(5.0);
        check_node(x, 20.0);
    }

    #[test]
    fn compute_pow() {
        let x1 = Node::input(2.0);
        let x = Node::pow(x1.clone(), 3.0);
        check_node(x.clone(), 8.0);

        x1.borrow_mut().set(3.0);
        check_node(x, 27.0);
    }

    #[test]
    fn compute_sin() {
        let x1 = Node::input(std::f32::consts::PI);
        let x = Node::sin(x1.clone());
        check_node(x.clone(), 0.0);

        x1.borrow_mut().set(std::f32::consts::FRAC_PI_2);
        check_node(x, 1.0);
    }

    #[test]
    fn compute_nested() {
        let x1 = Node::input(2.0);
        let x2 = Node::input(3.0);
        let x3 = Node::input(4.0);
        let x = Node::add(
            x1.clone(),
            Node::mul(x2.clone(), Node::pow(x3.clone(), 2.0)),
        );
        check_node(x.clone(), 50.0);
    }

    #[test]
    fn compute_with_same_inputs() {
        let x1 = Node::input(2.0);
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
}
