use rand::Rng;

pub fn sample(array: &mut [u32], size: usize) {
    let mut rng = rand::thread_rng();

    for i in 0..size {
        let rand_i = rng.gen_range(i..array.len());
        array.swap(i, rand_i);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut array = (0..50).collect::<Vec<_>>();
        sample(&mut array, 3);
        // println!("{:?}", &array[0..3]);
        assert_ne!(&array[0..3], &vec![0, 1, 2]);
    }
}
