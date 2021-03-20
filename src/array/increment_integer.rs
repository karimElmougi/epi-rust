pub fn increment(n: &mut Vec<i32>) {
    let mut carry = 1;
    for digit in n.iter_mut().rev() {
        let new_carry = (*digit + carry) / 10;
        *digit = (*digit + carry) % 10;
        carry = new_carry;
    }

    if carry == 1 {
        n.insert(0, 1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let mut n = vec![9, 9, 9];
        increment(&mut n);
        assert_eq!(n, vec![1, 0, 0, 0]);
    }

    #[test]
    fn test_2() {
        let mut n = vec![8];
        increment(&mut n);
        assert_eq!(n, vec![9]);
    }
}
