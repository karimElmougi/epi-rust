use std::collections::{HashMap, HashSet};

use rand::Rng;

pub fn random_subset_naive(n: usize, k: usize) -> Vec<usize> {
    let mut set = HashSet::new();
    let mut rng = rand::thread_rng();

    while set.len() != k {
        set.insert(rng.gen_range(0..n));
    }
    
    set.into_iter().collect()
}

pub fn random_subset_less_naive(n: usize, k: usize) -> Vec<usize> {
    let mut v = (0..n).collect::<Vec<_>>();
    let mut rng = rand::thread_rng();
    
    for i in 0..k {
        let rand_i = rng.gen_range(i..v.len());
        v.swap(i, rand_i);
    }
    
    v.resize(k, 0);
    v
}

// TODO: Revisit
pub fn random_subset(n: usize, k: usize) -> Vec<usize> {
    let mut map = HashMap::<usize, usize>::new();
    let mut rng = rand::thread_rng();

    for i in 0..k {
        let rand_i = rng.gen_range(0..n);
        let mapped_rand_i = map.get(&rand_i).map_or(rand_i, |&i| i);
        let mapped_i = map.get(&i).map_or(i, |&i| i);

        map.insert(rand_i, mapped_i);
        map.insert(i, mapped_rand_i);
    }

    (0..k).map(|i| map[&i]).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_naive() {
        let v = random_subset_naive(50, 3);
        println!("{:?}", v);
        //assert_ne!(&array[0..3], &vec![0, 1, 2]);
    }

    #[test]
    fn test_less_naive() {
        let v = random_subset_less_naive(50, 3);
        println!("{:?}", v);
        //assert_ne!(&array[0..3], &vec![0, 1, 2]);
    }
}
