pub fn remove_dup(array: &mut Vec<i32>) {
    if array.is_empty() {
        return;
    }

    let mut insertion_index = 1;

    for i in 1..array.len() {
        if array[i] != array[i - 1] {
            array[insertion_index] = array[i];
            insertion_index += 1;
        }
    }

    array.truncate(insertion_index);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut a = vec![1, 2, 2, 2, 3, 4, 5, 6, 6];
        remove_dup(&mut a);
        assert_eq!(a, vec![1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn test_2() {
        let mut a = vec![0, 0, 0, 0, 0, 0, 0];
        remove_dup(&mut a);
        assert_eq!(a, vec![0]);
    }

    #[test]
    fn test_3() {
        let mut a = vec![1, 2, 3, 4];
        remove_dup(&mut a);
        assert_eq!(a, vec![1, 2, 3, 4]);
    }
}
