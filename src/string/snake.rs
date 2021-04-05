pub fn snake_string(s: &str) -> String {
    let mut result = String::new();

    s.chars().skip(1).step_by(4).for_each(|c| result.push(c));
    s.chars().step_by(2).for_each(|c| result.push(c));
    s.chars().skip(3).step_by(4).for_each(|c| result.push(c));

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(snake_string(&"Hello World!"), "e lHloWrdlo!");
    }
}
