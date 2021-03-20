pub fn solve_naive(array: &[i32], index: usize) -> Vec<i32> {
    let mut new = vec![];
    let pivot = array[index];

    for item in array {
        if *item < pivot {
            new.push(*item);
        }
    }

    for item in array {
        if *item == pivot {
            new.push(*item);
        }
    }

    for item in array {
        if *item > pivot {
            new.push(*item);
        }
    }

    new
}

pub fn solve_constant_space(array: &mut [i32], index: usize) {
    let pivot = array[index];
    let mut swap_index = 0;

    for i in 0..array.len() {
        if array[i] < pivot {
            array.swap(i, swap_index);
            swap_index += 1;
        }
    }

    for i in 0..array.len() {
        if array[i] == pivot {
            array.swap(i, swap_index);
            swap_index += 1;
        }
    }

    for i in 0..array.len() {
        if array[i] > pivot {
            array.swap(i, swap_index);
            swap_index += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn naive_1() {
        let input = vec![0, 1, 2, 0, 2, 1, 1];
        let output = solve_naive(&input, 2);
        assert_eq!(output, vec![0, 1, 0, 1, 1, 2, 2]);
    }

    #[test]
    fn naive_2() {
        let input = vec![0, 1, 2, 0, 2, 1, 1];
        let output = solve_naive(&input, 1);
        assert_eq!(output, vec![0, 0, 1, 1, 1, 2, 2]);
    }

    #[test]
    fn constant_space_1() {
        let mut array = vec![0, 1, 2, 0, 2, 1, 1];
        solve_constant_space(&mut array, 2);
        assert_eq!(array, vec![0, 1, 0, 1, 1, 2, 2]);
    }

    #[test]
    fn constant_space_2() {
        let mut array = vec![0, 1, 2, 0, 2, 1, 1];
        solve_constant_space(&mut array, 1);
        assert_eq!(array, vec![0, 0, 1, 1, 1, 2, 2]);
    }
}
