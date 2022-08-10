use std::{cell::RefCell, rc::Rc};

use crate::graph::{Computable, Node};

#[derive(Debug, Clone)]
pub enum Ops {
    Input(f32),
    Add(Rc<RefCell<Node>>, Rc<RefCell<Node>>),
    AddVar(Vec<Rc<RefCell<Node>>>),
    Sub(Rc<RefCell<Node>>, Rc<RefCell<Node>>),
    Mul(Rc<RefCell<Node>>, Rc<RefCell<Node>>),
    Pow(Rc<RefCell<Node>>, f32),
    Sin(Rc<RefCell<Node>>),
}

impl Ops {
    pub fn set(&mut self, val: f32) {
        if let Ops::Input(x) = self {
            *x = val
        }
    }
}

impl Computable for Ops {
    fn compute(&mut self) -> f32 {
        use Ops::*;

        match self {
            Input(x) => *x,
            Add(x, y) => {
                let xr = x.borrow_mut().compute();
                let yr = y.borrow_mut().compute();
                xr + yr
            }
            AddVar(args) => args
                .iter()
                .fold(0f32, |acc, arg| acc + arg.borrow_mut().compute()),
            Sub(x, y) => {
                let xr = x.borrow_mut().compute();
                let yr = y.borrow_mut().compute();
                xr - yr
            }
            Mul(x, y) => {
                let xr = x.borrow_mut().compute();
                let yr = y.borrow_mut().compute();
                xr * yr
            }
            Pow(x, pow) => x.borrow_mut().compute().powf(*pow),
            Sin(x) => x.borrow_mut().compute().sin(),
        }
    }
}
