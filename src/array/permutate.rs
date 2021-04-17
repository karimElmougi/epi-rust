pub fn permutate_naive(a: &[char], p: &[usize]) -> Vec<char> {
    let mut tuples = a
        .iter()
        .enumerate()
        .map(|(i, c)| (*c, p[i]))
        .collect::<Vec<_>>();
    tuples.sort_by_key(|(_, i)| *i);

    tuples.into_iter().map(|(c, _)| c).collect::<Vec<_>>()
}

pub fn permutate_linear(a: &[char], p: &[usize]) -> Vec<char> {
    let mut vec = (0..a.len()).map(|_| '\0').collect::<Vec<_>>();

    for (i, &c) in a.iter().enumerate() {
        vec[p[i]] = c;
    }

    vec
}

// TODO: Revisit, not sure if correct
pub fn permutate_in_place(a: &mut [char], mut p: Vec<isize>) {
    for i in 0..a.len() {
        if p[i] >= 0 {
            let j = p[i] as usize;
            a.swap(i, j);
            p[i] = -1;
            p[j] = -1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_naive() {
        let result = permutate_naive(&vec!['a', 'b', 'c', 'd'], &vec![2, 0, 1, 3]);
        assert_eq!(result, vec!['b', 'c', 'a', 'd']);
    }

    #[test]
    fn test_linear() {
        let result = permutate_linear(&vec!['a', 'b', 'c', 'd'], &vec![2, 0, 1, 3]);
        assert_eq!(result, vec!['b', 'c', 'a', 'd']);
    }

    #[test]
    fn test_in_place() {
        let mut input = vec!['a', 'b', 'c', 'd'];
        permutate_in_place(&mut input, vec![2, 0, 1, 3]);
        assert_eq!(input, vec!['b', 'c', 'a', 'd']);
    }

    #[test]
    fn test_in_place2() {
        let mut input = vec!['a', 'b', 'c', 'd'];
        permutate_in_place(&mut input, vec![3, 2, 1, 0]);
        assert_eq!(input, vec!['d', 'c', 'b', 'a']);
    }
}
