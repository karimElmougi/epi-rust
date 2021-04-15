pub mod merge;

use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
pub struct Node<T> {
    pub elem: T,
    pub next: Option<Rc<RefCell<Self>>>,
}

impl<T> Default for Node<T>
where
    T: Default,
{
    fn default() -> Self {
        Self {
            elem: Default::default(),
            next: None,
        }
    }
}

impl<T> Node<T> {
    fn wrap(node: Self) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(node)))
    }

    pub fn insert_after(&mut self, elem: T) {
        let new_node = Self {
            elem,
            next: self.next.clone(),
        };
        self.next = Self::wrap(new_node);
    }

    pub fn delete_next(&mut self) {
        self.next = self
            .next
            .clone()
            .and_then(|n| n.as_ref().borrow().next.clone());
    }

    pub fn has_next(&self) -> bool {
        self.next.is_some()
    }
}

pub struct LinkedList<T> {
    pub head: Option<Rc<RefCell<Node<T>>>>,
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push_front(&mut self, elem: T) {
        self.head = Node::wrap(Node {
            elem,
            next: self.head.clone(),
        });
    }

    pub fn pop_front(&mut self) -> Option<Rc<RefCell<Node<T>>>> {
        let old_head = self.head.clone();
        self.head = self
            .head
            .clone()
            .and_then(|n| n.as_ref().borrow().next.clone());
        old_head
    }

    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }
}

impl<T> LinkedList<T>
where
    T: Eq,
{
    pub fn search(&self, elem: T) -> Option<Rc<RefCell<Node<T>>>> {
        let mut current = self.head.clone();
        while let Some(node) = current {
            if node.as_ref().borrow().elem == elem {
                return Some(node);
            }
            current = node.as_ref().borrow().next.clone();
        }

        None
    }
}

impl<T> Default for LinkedList<T> {
    fn default() -> Self {
        Self { head: None }
    }
}

pub struct IntoIter<T>(LinkedList<T>);

impl<T> Iterator for IntoIter<T> {
    type Item = Rc<RefCell<Node<T>>>;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop_front()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut list = LinkedList::new();

        assert_eq!(unwrap(list.pop_front()), None);

        list.push_front(1);
        list.push_front(2);
        list.push_front(3);

        assert_eq!(unwrap(list.pop_front()), Some(3));
        assert_eq!(unwrap(list.pop_front()), Some(2));

        list.push_front(4);
        list.push_front(5);

        assert_eq!(unwrap(list.pop_front()), Some(5));
        assert_eq!(unwrap(list.pop_front()), Some(4));

        assert_eq!(unwrap(list.pop_front()), Some(1));
        assert_eq!(unwrap(list.pop_front()), None);
    }

    fn unwrap<T: Copy>(node: Option<Rc<RefCell<Node<T>>>>) -> Option<T> {
        node.map(|n| n.as_ref().borrow().elem)
    }

    #[test]
    fn into_iter() {
        let mut list = LinkedList::new();
        list.push_front(1);
        list.push_front(2);
        list.push_front(3);

        let mut iter = list.into_iter();
        assert_eq!(unwrap(iter.next()), Some(3));
        assert_eq!(unwrap(iter.next()), Some(2));
        assert_eq!(unwrap(iter.next()), Some(1));
        assert_eq!(unwrap(iter.next()), None);
    }
}
