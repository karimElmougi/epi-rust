use std::cell::RefCell;
use std::rc::Rc;

pub struct Node<T> {
    pub elem: T,
    pub left: Option<Rc<RefCell<Self>>>,
    pub right: Option<Rc<RefCell<Self>>>,
}

impl<T> Node<T> {
    pub fn children(&self) -> Vec<Option<Rc<RefCell<Self>>>> {
        vec![self.left.clone(), self.right.clone()]
    }
}

pub fn new_test_tree() -> Rc<RefCell<Node<i32>>> {
    Rc::new(RefCell::new(Node {
        elem: 314,
        left: Some(Rc::new(RefCell::new(Node {
            elem: 6,
            left: Some(Rc::new(RefCell::new(Node {
                elem: 271,
                left: Some(Rc::new(RefCell::new(Node {
                    elem: 28,
                    left: None,
                    right: None,
                }))),
                right: Some(Rc::new(RefCell::new(Node {
                    elem: 0,
                    left: None,
                    right: None,
                }))),
            }))),
            right: Some(Rc::new(RefCell::new(Node {
                elem: 561,
                left: None,
                right: Some(Rc::new(RefCell::new(Node {
                    elem: 3,
                    left: Some(Rc::new(RefCell::new(Node {
                        elem: 17,
                        left: None,
                        right: None,
                    }))),
                    right: None,
                }))),
            }))),
        }))),
        right: Some(Rc::new(RefCell::new(Node {
            elem: 6,
            left: Some(Rc::new(RefCell::new(Node {
                elem: 2,
                left: None,
                right: Some(Rc::new(RefCell::new(Node {
                    elem: 1,
                    left: Some(Rc::new(RefCell::new(Node {
                        elem: 401,
                        left: None,
                        right: Some(Rc::new(RefCell::new(Node {
                            elem: 641,
                            left: None,
                            right: None,
                        }))),
                    }))),
                    right: Some(Rc::new(RefCell::new(Node {
                        elem: 257,
                        left: None,
                        right: None,
                    }))),
                }))),
            }))),
            right: Some(Rc::new(RefCell::new(Node {
                elem: 271,
                left: None,
                right: Some(Rc::new(RefCell::new(Node {
                    elem: 28,
                    left: None,
                    right: None,
                }))),
            }))),
        }))),
    }))
}
