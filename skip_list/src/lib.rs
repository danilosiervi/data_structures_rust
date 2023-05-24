use std::rc::Rc;
use std::cell::RefCell;

type Link<T> = Option<Rc<RefCell<Node<T>>>>;

#[derive(Clone)]
struct Node<T: Clone> {
    data: T,
    next: Vec<Link<T>>,
    pub id: u64,
}

impl<T: Clone> Node<T> {
    fn new(data: T, links: Vec<Link<T>>, id: u64) -> Rc<RefCell<Node<T>>> {
        Rc::new(RefCell::new(Node {
            data,
            next: links,
            id,
        }))
    }
}

#[derive(Clone)]
pub struct SkipList<T: Clone> {
    head: Link<T>,
    tails: Vec<Link<T>>,
    max_level: usize,
    pub length: u64,
}

impl<T: Clone> SkipList<T> {
    pub fn new(max_level: usize) -> Self {
        SkipList {
            head: None,
            tails: vec![None; max_level + 1],
            max_level,
            length: 0,
        }
    }

    fn get_level(&self) -> usize {
        let mut n = 0;
        while rand::random::<bool>() && n < self.max_level {
            n += 1;
        }
        n
    }

    pub fn append(&mut self, data: T) {
        let level = 1 + if self.head.is_none() {
            self.max_level
        } else {
            self.get_level()
        };

        let new_node = Node::new(data, vec![None; level], self.length + 1);

        for i in 0..level {
            if let Some(node) = self.tails[i].take() {
                let next = &mut node.borrow_mut().next;
                next[i] = Some(new_node.clone());
            }
            
            self.tails[i] = Some(new_node.clone());
        }

        if self.head.is_none() {
            self.head = Some(new_node.clone());
        }

        self.length += 1;
    }

    pub fn search(&self, id: u64) -> Option<T> {
        match self.head {
            Some(ref head) => {
                let mut start_level = self.max_level;
                let node = head.clone();
                let mut result = None;

                loop {
                    if node.borrow().next[start_level].is_some() {
                        break;
                    }
                    start_level -= 1;
                }

                let mut n = node;

                for level in (0..=start_level).rev() {
                    loop {
                        let next = n.clone();
                        match next.borrow().next[level] {
                            Some(ref next) if next.borrow().id <= id => n = next.clone(),
                            _ => break,
                        };
                    }

                    if n.borrow().id == id {
                        result = Some(n.borrow().data.clone());
                        break;
                    }
                }

                result
            }
            None => None,
        }
    }

    pub fn iter_level(&self, level: usize) -> Iter<T> {
        Iter::new(self.head.clone(), level)
    }
}

pub struct Iter<T: Clone> {
    current: Link<T>,
    level: usize,
}

impl<T: Clone> Iter<T> {
    fn new(start: Link<T>, level: usize) -> Self {
        Iter {
            current: start,
            level,
        }
    }
}

impl<T: Clone> Iterator for Iter<T> {
   type Item = (u64, T);

   fn next(&mut self) -> Option<Self::Item> {
        let current = &self.current;
        let mut result = None;

        self.current = match current {
            Some(ref current) => {
                let current = current.borrow();
                result = Some((current.id, current.data.clone()));
                current.next[self.level].clone()
            },
            None => None,
        };

        result
   }
}

impl<T: Clone> IntoIterator for SkipList<T> {
    type Item = (u64, T);
    type IntoIter = Iter<T>;

    fn into_iter(self) -> Self::IntoIter {
        Iter::new(self.head, 0)
    }
}

impl<T: Clone> std::fmt::Debug for SkipList<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.head {
            Some(ref _head) => {
                for level in (0..=self.max_level).rev() {
                    let _ = write!(f, "{}: ", level);

                    for n in self.iter_level(level) {
                        let _ = write!(f, "[{}] ", n.0);
                    }

                    let _ = writeln!(f, "");
                }
                Ok(())
            },
            None => write!(f, "The list is empty: []"),
        }
    }
}

