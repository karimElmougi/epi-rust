pub fn enumerate_primes(n: i32) -> Vec<i32> {
    let mut integers = (0..=n).map(|_| true).collect::<Vec<_>>();

    for i in 2..=n {
        if !integers[i as usize] {
            continue;
        }

        let mut multiples = i + i;
        while multiples <= n {
            integers[multiples as usize] = false;
            multiples += i;
        }
    }

    integers
        .into_iter()
        .enumerate()
        .skip(2)
        .filter(|(_, p)| *p)
        .map(|(i, _)| i as i32)
        .collect::<Vec<_>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(enumerate_primes(18), vec![2, 3, 5, 7, 11, 13, 17]);
    }
}
