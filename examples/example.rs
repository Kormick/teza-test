use teza::graph::*;

fn main() {
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
    println!("Graph output = {}", result);
    assert_eq!(result, -0.32727);

    x1.borrow_mut().set(2.0);
    x2.borrow_mut().set(3.0);
    x3.borrow_mut().set(4.0);
    result = graph.borrow_mut().compute();
    result = round(result, 5);
    println!("Graph output = {}", result);
    assert_eq!(result, -0.56656);
}
