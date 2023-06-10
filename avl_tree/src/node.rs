use std::cmp::max;
use std::mem;
use super::side::*;

pub type Tree<T> = Option<Box<Node<T>>>;

pub struct Node<T: Ord> {
    pub value: T,
    pub height: usize,
    pub left: Tree<T>,
    pub right: Tree<T>,
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

    pub fn rebalance(&mut self) {
        self.update_height();

        let side = match self.balance_factor() {
            -2 => Side::Left,
            2 => Side::Right,
            _ => return,
        };

        let subtree = self.child_mut(side).as_mut().unwrap();

        if let (Side::Left, 1) | (Side::Right, -1) = (side, subtree.balance_factor()) {
            subtree.rotate(side);
        }

        self.rotate(!side);
    }
}
