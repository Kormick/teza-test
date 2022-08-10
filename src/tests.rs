use crate::graph::*;

#[test]
fn test() {
    fn round(x: f32, precision: u32) -> f32 {
        let m = 10i32.pow(precision) as f32;
        (x * m).round() / m
    }

    let x1 = Node::input(1.0);
    let x2 = Node::input(2.0);
    let x3 = Node::input(3.0);
    let graph = Node::add(
        x1.clone(),
        Node::mul(
            x2.clone(),
            Node::sin(Node::add(x2.clone(), Node::pow(x3.clone(), 3.0))),
        ),
    );

    let mut result = graph.borrow_mut().compute();
    result = round(result, 5);
    assert_eq!(result, -0.32727);

    x1.borrow_mut().set(2.0);
    x2.borrow_mut().set(3.0);
    x3.borrow_mut().set(4.0);
    result = graph.borrow_mut().compute();
    result = round(result, 5);
    assert_eq!(result, -0.56656);
}

#[test]
fn add_two_variables() {
    let x1 = Node::input(1.0);
    let x2 = Node::input(2.0);
    let graph = Node::add(x1.clone(), x2.clone());

    let result = graph.borrow_mut().compute();
    assert_eq!(result, 3.0);

    let result = graph.borrow_mut().compute();
    assert_eq!(result, 3.0);

    x1.borrow_mut().set(3.0);

    let result = graph.borrow_mut().compute();
    assert_eq!(result, 5.0);

    let result = graph.borrow_mut().compute();
    assert_eq!(result, 5.0);
}

#[test]
fn test_3() {
    let x1 = Node::input(2.0);
    let x2 = Node::input(3.0);
    let x3 = Node::input(4.0);
    let graph = Node::add(
        Node::mul(x1.clone(), x3.clone()),
        Node::mul(x2.clone(), x3.clone()),
    );

    let result = graph.borrow_mut().compute();
    assert_eq!(result, 20.0);
    let result = graph.borrow_mut().compute();
    assert_eq!(result, 20.0);

    x1.borrow_mut().set(5.0);
    let result = graph.borrow_mut().compute();
    assert_eq!(result, 32.0);
    let result = graph.borrow_mut().compute();
    assert_eq!(result, 32.0);

    x3.borrow_mut().set(1.0);
    let result = graph.borrow_mut().compute();
    assert_eq!(result, 8.0);
    let result = graph.borrow_mut().compute();
    assert_eq!(result, 8.0);
}

#[test]
fn test_4() {
    let x1 = Node::input(2.0);
    let args = vec![x1.clone(), x1.clone()];
    let graph = Node::mul(x1.clone(), Node::add_var(args.clone()));

    let result = graph.borrow_mut().compute();
    assert_eq!(result, 8.0);
}
