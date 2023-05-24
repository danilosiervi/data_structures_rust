use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug, Clone)]
struct Node<T: Clone> {
    data: T,
    next: Link<T>,
}

impl<T: Clone> Node<T> {
    fn new(data: T) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Node {
            data,
            next: None,
        }))
    }
}

type Link<T> = Option<Rc<RefCell<Node<T>>>>;

#[derive(Debug, Clone)]
pub struct LinkedList<T: Clone> {
    head: Link<T>,
    tail: Link<T>,
    pub length: usize,
}

impl<T: Clone> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList {
            head: None,
            tail: None,
            length: 0,
        }
    }

    pub fn append(&mut self, value: T) {
        let new_node = Node::new(value);

        match self.tail.take() {
            Some(tail) => tail.borrow_mut().next = Some(new_node.clone()),
            None => self.head = Some(new_node.clone()),
        }
        
        self.tail = Some(new_node);
        self.length += 1;
    }

    pub fn prepend(&mut self, value: T) {
        let new_node = Node::new(value);

        match self.head.take() {
            Some(head) => new_node.borrow_mut().next = Some(head.clone()),
            None => self.tail = Some(new_node.clone()),
        }

        self.length += 1;
        self.head = Some(new_node);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|head| {
            if let Some(node) = head.borrow_mut().next.take() {
                self.head = Some(node);
            } else {
                self.tail.take();
            }

            self.length -= 1;
            Rc::try_unwrap(head).ok()
                .unwrap()
                .into_inner().data
        })
    }

    pub fn iter(&self) -> Iter<T> {
        Iter::new(self.head.clone())
    }
}

pub struct Iter<T: Clone> {
    current: Link<T>,
}

impl<T: Clone> Iter<T> {
    fn new(start: Link<T>) -> Self {
        Iter {
            current: start,
        }
    }
}

impl<T: Clone> Iterator for Iter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let current = &self.current;
        let mut result = None;

        self.current = match current {
            Some(ref current) => {
                let current = current.borrow();
                result = Some(current.data.clone());
                current.next.clone()
            },
            None => None,
        };

        result
    }
}

impl<T: Clone> IntoIterator for LinkedList<T> {
    type Item = T;
    type IntoIter = Iter<T>;

    fn into_iter(self) -> Self::IntoIter {
        Iter::new(self.head)
    }
}

