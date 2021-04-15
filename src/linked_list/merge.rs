use crate::linked_list::LinkedList;

pub fn merge<T>(left: LinkedList<T>, right: LinkedList<T>) -> LinkedList<T>
where
    T: PartialOrd,
{
    let mut left = left.into_iter().peekable();
    let mut right = right.into_iter().peekable();

    let new_head = match (left.peek(), right.peek()) {
        (Some(l), Some(r)) => {
            if l.as_ref().borrow().elem < r.as_ref().borrow().elem {
                let l = l.clone();
                left.next();
                l
            } else {
                let r = r.clone();
                right.next();
                r
            }
        }
        (Some(l), None) => l.clone(),
        (None, Some(r)) => r.clone(),
        (None, None) => return Default::default(),
    };

    let mut curr = new_head.clone();

    loop {
        match (left.peek(), right.peek()) {
            (Some(l), Some(r)) => {
                if l.borrow().elem < r.borrow().elem {
                    curr.borrow_mut().next = Some(l.clone());
                    curr = l.clone();
                    left.next();
                } else {
                    curr.borrow_mut().next = Some(r.clone());
                    curr = r.clone();
                    right.next();
                }
            }
            (Some(l), None) => {
                curr.borrow_mut().next = Some(l.clone());
                curr = l.clone();
                left.next();
            }
            (None, Some(r)) => {
                curr.borrow_mut().next = Some(r.clone());
                curr = r.clone();
                right.next();
            }
            (None, None) => break,
        }
    }

    LinkedList {
        head: Some(new_head),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut left = LinkedList::new();
        left.push_front(10);
        left.push_front(8);
        left.push_front(6);
        left.push_front(4);
        left.push_front(2);

        let mut right = LinkedList::new();
        right.push_front(9);
        right.push_front(7);
        right.push_front(5);
        right.push_front(3);
        right.push_front(1);

        let result = merge(left, right)
            .into_iter()
            .map(|n| n.borrow().elem)
            .collect::<Vec<_>>();

        assert_eq!(result, (1..=10).collect::<Vec<_>>());
    }
}
