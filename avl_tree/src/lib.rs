type Link<T> = Option<Box<Node<T>>>;

struct Node<T: Ord> {
    value: T,
    height: usize,
    left: Link<T>,
    right: Link<T>,
}

pub struct AvlTree<T: Ord> {
    root: Link<T>,
    length: usize,
}

#[derive(Clone, Copy)]
enum Side {
    Left,
    Right,
}
