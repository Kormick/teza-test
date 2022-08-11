use std::{cell::RefCell, rc::Rc};

use crate::node::Computable;

/// Represents set of available operations for computational graph.
#[derive(Clone)]
pub enum Operation {
    /// Sum of two values.
    Add(Rc<RefCell<dyn Computable>>, Rc<RefCell<dyn Computable>>),
    /// Sum of variable amount of values.
    AddVar(Vec<Rc<RefCell<dyn Computable>>>),
    /// Subtraction of two values.
    Sub(Rc<RefCell<dyn Computable>>, Rc<RefCell<dyn Computable>>),
    /// Multiplication of two values.
    Mul(Rc<RefCell<dyn Computable>>, Rc<RefCell<dyn Computable>>),
    /// Exponentiation of value to given exponent.
    Pow(Rc<RefCell<dyn Computable>>, f32),
    /// Sin result of given value.
    Sin(Rc<RefCell<dyn Computable>>),
}

impl Operation {
    /// Computes operation result depending on its type.
    pub fn compute(&self) -> f32 {
        use Operation::*;

        match self {
            Add(x, y) => {
                let x_res = x.borrow_mut().compute();
                let y_res = y.borrow_mut().compute();
                x_res + y_res
            }
            AddVar(args) => args
                .iter()
                .fold(0.0, |acc, arg| acc + arg.borrow_mut().compute()),
            Sub(x, y) => {
                let x_res = x.borrow_mut().compute();
                let y_res = y.borrow_mut().compute();
                x_res - y_res
            }
            Mul(x, y) => {
                let x_res = x.borrow_mut().compute();
                let y_res = y.borrow_mut().compute();
                x_res * y_res
            }
            Pow(x, pow) => x.borrow_mut().compute().powf(*pow),
            Sin(x) => x.borrow_mut().compute().sin(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Const {
        val: f32,
    }

    impl Const {
        fn from_val(val: f32) -> Rc<RefCell<Self>> {
            Rc::new(RefCell::new(Self { val }))
        }
    }

    impl Computable for Const {
        fn compute(&mut self) -> f32 {
            self.val
        }

        fn add_dependency(&mut self, _dependency: Rc<RefCell<dyn Computable>>) {}

        fn reset_cache(&mut self) {}
    }

    #[test]
    fn add() {
        let x1 = Const::from_val(1.0);
        let x2 = Const::from_val(2.0);
        let opp = Operation::Add(x1, x2);
        assert_eq!(opp.compute(), 3.0);
    }

    #[test]
    fn add_var() {
        let args: Vec<Rc<RefCell<dyn Computable>>> = vec![
            Const::from_val(1.0),
            Const::from_val(2.0),
            Const::from_val(3.0),
        ];
        let opp = Operation::AddVar(args);
        assert_eq!(opp.compute(), 6.0);
    }

    #[test]
    fn sub() {
        let x1 = Const::from_val(1.0);
        let x2 = Const::from_val(2.0);
        let opp = Operation::Sub(x1, x2);
        assert_eq!(opp.compute(), -1.0);
    }

    #[test]
    fn mul() {
        let x1 = Const::from_val(2.0);
        let x2 = Const::from_val(3.0);
        let opp = Operation::Mul(x1, x2);
        assert_eq!(opp.compute(), 6.0);
    }

    #[test]
    fn pow() {
        let x1 = Const::from_val(2.0);
        let opp = Operation::Pow(x1, 3.0);
        assert_eq!(opp.compute(), 8.0);
    }

    #[test]
    fn sin() {
        let x1 = Const::from_val(std::f32::consts::FRAC_PI_2);
        let opp = Operation::Sin(x1);
        assert_eq!(opp.compute(), 1.0);
    }
}
