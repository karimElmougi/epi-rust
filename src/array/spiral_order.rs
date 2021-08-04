use Direction::{East, North, South, West};

enum Direction {
    North,
    South,
    East,
    West,
}

pub fn spiral_iter<T, const N: usize>(array: &[[T; N]; N]) -> Vec<T>
where
    T: Clone,
{
    let mut north_guard = 0;
    let mut south_guard = N - 1;
    let mut east_guard = N - 1;
    let mut west_guard = 0;

    let mut direction = East;

    let mut v = vec![];
    loop {
        match direction {
            East => {
                for i in west_guard..=east_guard {
                    v.push(array[north_guard][i].clone());
                }
                north_guard += 1;
                direction = South;
            }
            South => {
                #[allow(clippy::needless_range_loop)]
                for i in north_guard..=south_guard {
                    v.push(array[i][east_guard].clone());
                }
                east_guard -= 1;
                direction = West;
            }
            West => {
                for i in (west_guard..=east_guard).rev() {
                    v.push(array[south_guard][i].clone());
                }
                south_guard -= 1;
                direction = North;
            }
            North => {
                for i in (north_guard..=south_guard).rev() {
                    v.push(array[i][west_guard].clone());
                }
                west_guard += 1;
                direction = East;
            }
        }

        if west_guard > east_guard || north_guard > south_guard {
            return v;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let grid = [[0, 1, 2, 3], [11, 12, 13, 4], [10, 15, 14, 5], [9, 8, 7, 6]];
        let spiral = spiral_iter(&grid);
        assert_eq!(spiral, (0..16).collect::<Vec<_>>());
    }
}
