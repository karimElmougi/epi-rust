use crate::binary_tree;

use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

pub fn to_array(root: Rc<RefCell<binary_tree::Node<i32>>>) -> Vec<Vec<i32>> {
    let mut result = vec![];
    let mut current_level = VecDeque::new();
    current_level.push_back(root);

    while !current_level.is_empty() {
        let mut current_level_elements = vec![];
        let mut next_level = VecDeque::new();
        while let Some(node) = current_level.pop_front() {
            node.borrow()
                .children()
                .iter()
                .filter(|c| c.is_some())
                .map(|c| c.clone().unwrap())
                .for_each(|c| next_level.push_back(c));

            current_level_elements.push(node.borrow().elem);
        }
        result.push(current_level_elements);
        current_level = next_level;
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let root = binary_tree::new_test_tree();

        assert_eq!(
            to_array(root),
            vec![
                vec![314],
                vec![6, 6],
                vec![271, 561, 2, 271],
                vec![28, 0, 3, 1, 28],
                vec![17, 401, 257],
                vec![641]
            ]
        );
    }
}
