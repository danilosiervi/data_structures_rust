use std::cmp::Ordering;

type Tree<T> = Option<Box<Node<T>>>;

struct Node<T: Ord> {
    value: T,
    height: usize,
    left: Tree<T>,
    right: Tree<T>,
}

pub struct AvlTree<T: Ord> {
    root: Tree<T>,
    length: usize,
}

#[derive(Clone, Copy)]
enum Side {
    Left,
    Right,
}

impl<T: Ord> AvlTree<T> {
    pub fn new() -> Self {
        AvlTree {
            root: None,
            length: 0,
        }
    }

    pub fn contains(&self, value: &T) -> bool {
        let mut current = &self.root;

        while let Some(node) = current {
            current = match value.cmp(&node.value) {
                Ordering::Equal => return true,
                Ordering::Less => &node.left,
                Ordering::Greater => &node.right,
            }
        }

        false
    }

    pub fn insert(&mut self, value: T) -> bool {
        let inserted = insert(&mut self.root, value);

        if inserted {
            self.length += 1;
        }

        inserted
    }
}

fn insert<T: Ord>(tree: &mut Tree<T>, value: T) -> bool {
    if let Some(node) = tree {
        let inserted = match value.cmp(&node.value) {
            Ordering::Equal => false,
            Ordering::Less => insert(&mut node.left, value),
            Ordering::Greater => insert(&mut node.right, value),
        };

        if inserted {
            node.rebalance();
        }

        inserted
    } else {
        *tree = Some(Box::new(Node {
            value,
            height: 1,
            left: None,
            right: None,
        }));

        true
    }
}
