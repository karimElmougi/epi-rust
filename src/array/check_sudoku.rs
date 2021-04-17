pub fn check(array: &[[i32; 9]; 9]) -> bool {
    // check rows
    for line in array.iter().take(9) {
        let mut set = 0u16;
        for val in line.iter().take(9) {
            let bit = 1 << val;
            let is_present = (set & bit) != 0;
            match (val, is_present) {
                (0, _) => continue,
                (_, true) => return false,
                _ => set |= bit,
            }
        }
    }

    // check columns
    for col in array.iter().take(9) {
        let mut set = 0u16;
        for val in col.iter().take(9) {
            let bit = 1 << val;
            let is_present = (set & bit) != 0;
            match (val, is_present) {
                (0, _) => continue,
                (_, true) => return false,
                _ => set |= bit,
            }
        }
    }

    // check squares
    // TODO: Maybe clean this up, couldn't be arsed
    for line_limit in (3..9).step_by(3) {
        for line in array.iter().take(line_limit).skip(line_limit - 3) {
            let mut set = 0u16;
            for col_limit in (3..9).step_by(3) {
                for val in line.iter().take(col_limit).skip(col_limit - 3) {
                    let bit = 1 << val;
                    let is_present = (set & bit) != 0;
                    match (val, is_present) {
                        (0, _) => continue,
                        (_, true) => return false,
                        _ => set |= bit,
                    }
                }
            }
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let grid = [
            [5, 3, 0, 0, 7, 0, 0, 0, 0],
            [6, 0, 0, 1, 9, 5, 0, 0, 0],
            [0, 9, 8, 0, 0, 0, 0, 6, 0],
            [8, 0, 0, 0, 6, 0, 0, 0, 3],
            [4, 0, 0, 8, 0, 3, 0, 0, 1],
            [7, 0, 0, 0, 2, 0, 0, 0, 6],
            [0, 6, 0, 0, 0, 0, 2, 8, 0],
            [0, 0, 0, 4, 1, 9, 0, 0, 5],
            [0, 0, 0, 0, 8, 0, 0, 7, 9],
        ];
        assert_eq!(check(&grid), true);
    }
}
