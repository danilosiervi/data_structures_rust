use std::cmp::{max, Ordering};
use std::mem;
use std::ops::Not;

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

impl<T: Ord> Node<T> {
    fn child(&self, side: Side) -> &Tree<T> {
        match side {
            Side::Left => &self.left,
            Side::Right => &self.right,
        }
    }

    fn child_mut(&mut self, side: Side) -> &mut Tree<T> {
        match side {
            Side::Left => &mut self.left,
            Side::Right => &mut self.right,
        }
    }

    fn height(&self, side: Side) -> usize {
        self.child(side).as_ref().map_or(0, |n| n.height)
    }

    fn balance_factor(&self) -> i8 {
        let (left, right) = (self.height(Side::Left), self.height(Side::Right));

        if left < right {
            (right - left) as i8
        } else {
            -((left - right) as i8)
        }
    }

    fn update_height(&mut self) {
        self.height = 1 + max(self.height(Side::Left), self.height(Side::Right));
    }

    fn rotate(&mut self, side: Side) {
        let mut subtree = self.child_mut(!side).take().unwrap();

        *self.child_mut(!side) = subtree.child_mut(side).take();
        self.update_height();

        mem::swap(self, subtree.as_mut());

        *self.child_mut(side) = Some(subtree);
        self.update_height();
    }
}

impl Not for Side {
    type Output = Side;

    fn not(self) -> Self::Output {
        match self {
            Side::Left => Side::Right,
            Side::Right => Side::Left,
        }
    }
}
