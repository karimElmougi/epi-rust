use std::cell::RefCell;
use std::cmp::max;
use std::convert::From;
use std::rc::Rc;

pub struct NodeRef<T>(Option<Rc<RefCell<Node<T>>>>);

impl<T> Clone for NodeRef<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<T> Default for NodeRef<T> {
    fn default() -> Self {
        Self(None)
    }
}

impl<T> From<T> for NodeRef<T> {
    fn from(elem: T) -> Self {
        Self::new(elem, None, None)
    }
}

impl<T> NodeRef<T> {
    pub fn with_left(elem: T, left: Self) -> Self {
        Self::new(elem, left.0, None)
    }

    pub fn with_right(elem: T, right: Self) -> Self {
        Self::new(elem, None, right.0)
    }

    pub fn with_left_right(elem: T, left: Self, right: Self) -> Self {
        Self::new(elem, left.0, right.0)
    }

    pub fn is_some(&self) -> bool {
        self.0.is_some()
    }

    pub fn is_none(&self) -> bool {
        self.0.is_some()
    }

    pub fn unwrap(self) -> Rc<RefCell<Node<T>>> {
        self.0.unwrap()
    }

    pub fn has_children(&self) -> bool {
        match &self.0 {
            Some(node) => node.borrow().has_children(),
            None => false,
        }
    }

    pub fn left(&self) -> Self {
        match &self.0 {
            Some(node) => node.borrow().left.clone(),
            None => Self(None),
        }
    }

    pub fn right(&self) -> Self {
        match &self.0 {
            Some(node) => node.borrow().right.clone(),
            None => Self(None),
        }
    }

    pub fn height_rec(&self) -> usize {
        match &self.0 {
            Some(node) => node.borrow().height_rec(),
            None => 0,
        }
    }

    fn new(
        elem: T,
        left: Option<Rc<RefCell<Node<T>>>>,
        right: Option<Rc<RefCell<Node<T>>>>,
    ) -> Self {
        Self(Some(Rc::new(RefCell::new(Node {
            elem,
            left: Self(left),
            right: Self(right),
        }))))
    }
}

impl<T: Clone> NodeRef<T> {
    pub fn elem(&self) -> Option<T> {
        match &self.0 {
            Some(node) => Some(node.borrow().elem.clone()),
            None => None,
        }
    }
}

pub struct Node<T> {
    pub elem: T,
    pub left: NodeRef<T>,
    pub right: NodeRef<T>,
}

impl<T: Clone> Node<T> {
    pub fn children(&self) -> Vec<NodeRef<T>> {
        vec![self.left.clone(), self.right.clone()]
    }
}

impl<T> Node<T> {
    pub fn has_children(&self) -> bool {
        self.left.is_some() || self.right.is_some()
    }

    pub fn height_rec(&self) -> usize {
        1 + max(self.left.height_rec(), self.right.height_rec())
    }

    pub fn is_height_balanced_rec(&self) -> bool {
        (self.left.height_rec() as isize - self.right.height_rec() as isize).abs() <= 1
    }
}

impl<T: PartialEq + Clone> Node<T> {
    pub fn is_symmetric(&self) -> bool {
        Self::are_mirrors(self.left.clone(), self.right.clone())
    }

    fn are_mirrors(left: NodeRef<T>, right: NodeRef<T>) -> bool {
        match (left.is_some(), right.is_some()) {
            (false, false) => return true,
            (false, true) => return false,
            (true, false) => return false,
            _ => (),
        }

        if left.elem() != right.elem() {
            return false;
        }

        Self::are_mirrors(left.left(), right.right()) && Self::are_mirrors(left.right(), right.left())
    }
}

pub fn new_test_tree() -> Rc<RefCell<Node<i32>>> {
    Rc::new(RefCell::new(Node {
        elem: 314,
        left: NodeRef::with_left_right(
            6,
            NodeRef::with_left_right(271, NodeRef::from(28), NodeRef::from(0)),
            NodeRef::with_right(561, NodeRef::with_left(3, NodeRef::from(17))),
        ),
        right: NodeRef::with_left_right(
            6,
            NodeRef::with_right(
                2,
                NodeRef::with_left_right(
                    1,
                    NodeRef::with_right(401, NodeRef::from(641)),
                    NodeRef::from(257),
                ),
            ),
            NodeRef::with_right(271, NodeRef::from(28)),
        ),
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let test_tree = new_test_tree();
        assert_eq!(test_tree.borrow().height_rec(), 6);
        assert!(test_tree.borrow().is_height_balanced_rec());
    }

    #[test]
    fn mirror_test() {
        let tree = Node {
            elem: 314,
            left: NodeRef::with_right(6, NodeRef::with_right(2, NodeRef::from(3))),
            right: NodeRef::with_left(6, NodeRef::with_left(2, NodeRef::from(3))),
        };
        assert!(tree.is_symmetric());

        let tree = Node {
            elem: 314,
            left: NodeRef::with_right(6, NodeRef::with_right(561, NodeRef::from(3))),
            right: NodeRef::with_left(6, NodeRef::with_left(2, NodeRef::from(3))),
        };
        assert!(!tree.is_symmetric());

        let tree = Node {
            elem: 314,
            left: NodeRef::with_right(6, NodeRef::with_right(561, NodeRef::from(3))),
            right: NodeRef::with_left(6, NodeRef::from(561)),
        };
        assert!(!tree.is_symmetric());
    }
}
