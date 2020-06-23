struct Node<T> {
    value: T,
    visited: bool,
    edges: Option<Box<Node<T>>>,
}

struct Edge {
    node: Box<Node>,
    next: &Edge,
}

fn main() {}
