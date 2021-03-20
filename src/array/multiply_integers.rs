pub fn multiply(a: &[i32], b: &[i32]) -> Vec<i32> {
    let mut result = vec![0; a.len() + b.len()];
    let mut carry = 0;

    for (i, digit) in result.iter_mut().enumerate().rev() {
        let mut accumulator = 0;

        let skip_val = std::cmp::max(0, i as isize - a.len() as isize) as usize;
        for (index_a, digit_b) in (0..i)
            .rev()
            .zip(b.iter())
            .skip(skip_val)
        {
            accumulator += digit_b.abs() * a[index_a].abs();
        }

        accumulator += carry;
        *digit = accumulator % 10;
        carry = accumulator / 10;
    }

    let mut result = result
        .into_iter()
        .skip_while(|n| *n == 0)
        .collect::<Vec<_>>();

    let sign = if (a[0] < 0) ^ (b[0] < 0) { -1 } else { 1 };

    if result.is_empty() {
        vec![0]
    } else {
        result[0] *= sign;
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let a = vec![-9, 9, 9];
        let b = vec![2];
        let result = multiply(&a, &b);
        assert_eq!(result, vec![-1, 9, 9, 8]);
    }

    #[test]
    fn test_2() {
        let a = vec![-9, 9, 9];
        let b = vec![2, 3];
        let result = multiply(&a, &b);
        assert_eq!(result, vec![-2, 2, 9, 7, 7]);
    }

    #[test]
    fn test_3() {
        let a = vec![-9, 9, 9];
        let b = vec![0];
        let result = multiply(&a, &b);
        assert_eq!(result, vec![0]);
    }
}
