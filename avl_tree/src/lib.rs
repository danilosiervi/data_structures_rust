use std::cmp::Ordering;
use std::iter::FromIterator;

mod node;
use node::*;

mod side;

pub struct AvlTree<T: Ord> {
    root: Tree<T>,
    length: usize,
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

    pub fn remove(&mut self, value: &T) -> bool {
        let removed = remove(&mut self.root, value);

        if removed {
            self.length -= 1;
        }

        removed
    }

    pub fn is_empty(&self) -> bool {
        self.length == 0
    }

    fn node_iter(&self) -> NodeIter<T> {
        let cap = self.root.as_ref().map_or(0, |n| n.height);

        let mut node_iter = NodeIter {
            stack: Vec::with_capacity(cap),
        };

        let mut child = &self.root;
        while let Some(node) = child {
            node_iter.stack.push(node.as_ref());
            child = &node.left;
        }

        node_iter
    }

    pub fn iter(&self) -> Iter<T> {
        Iter {
            node_iter: self.node_iter()
        }
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

fn remove<T: Ord>(tree: &mut Tree<T>, value: &T) -> bool {
    if let Some(node) = tree {
        let removed = match value.cmp(&node.value) {
            Ordering::Less => remove(&mut node.left, value),
            Ordering::Greater => remove(&mut node.right, value),
            Ordering::Equal => {
                *tree = match (node.left.take(), node.right.take()) {
                    (None, None) => None,
                    (Some(b), None) | (None, Some(b)) => Some(b),
                    (Some(left), Some(right)) => Some(merge(left, right)),
                };

                return true;
            }
        };

        if removed {
            node.rebalance();
        }

        removed
    } else {
        false
    }
}

fn merge<T: Ord>(left: Box<Node<T>>, right: Box<Node<T>>) -> Box<Node<T>> {
    let mut op_right = Some(right);
    let mut root = take_min(&mut op_right).unwrap();

    root.left = Some(left);
    root.right = op_right;
    root.rebalance();

    root
}

fn take_min<T: Ord>(tree: &mut Tree<T>) -> Tree<T> {
    if let Some(mut node) = tree.take() {
        if let Some(small) = take_min(&mut node.left) {
            node.rebalance();
            *tree = Some(node);

            Some(small)
        } else {
            *tree = node.right.take();

            Some(node)
        }
    } else {
        None
    }
}

impl<T: Ord> Default for AvlTree<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Ord> FromIterator<T> for AvlTree<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut tree = AvlTree::new();

        for value in iter {
            tree.insert(value);
        }

        tree
    }
}

struct NodeIter<'a, T: Ord> {
    stack: Vec<&'a Node<T>>,
}

impl<'a, T: Ord> Iterator for NodeIter<'a, T> {
    type Item = &'a Node<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(node) = self.stack.pop() {
            let mut child = &node.right;

            while let Some(subtree) = child {
                self.stack.push(subtree.as_ref());
                child = &subtree.left;
            }

            Some(node)
        } else {
            None
        }
    }
}

pub struct Iter<'a, T: Ord> {
    node_iter: NodeIter<'a, T>,
}

impl<'a, T: Ord> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.node_iter.next() {
            Some(node) => Some(&node.value),
            None => None,
        }
    }
}
