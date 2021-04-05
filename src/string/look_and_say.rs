pub fn look_and_say(n: usize) -> String {
    (1..n).fold("1".to_string(), |s, _| next_look_and_say(s))
}

fn next_look_and_say(s: String) -> String {
    let mut res = String::new();

    let mut chars = s.chars().peekable();
    let mut curr = *chars.peek().unwrap();
    let mut count = 0;

    while let Some(c) = chars.next() {
        if c == curr {
            count += 1;
        } else {
            res.push_str(&format!("{}{}", count, curr));
            curr = c;
            count = 1;
        }

        if chars.peek().is_none() {
            res.push_str(&format!("{}{}", count, curr));
        }
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_naive() {
        assert_eq!(look_and_say(1), "1");
        assert_eq!(look_and_say(2), "11");
        assert_eq!(look_and_say(3), "21");
        assert_eq!(look_and_say(4), "1211");
        assert_eq!(look_and_say(5), "111221");
        assert_eq!(look_and_say(6), "312211");
        assert_eq!(look_and_say(7), "13112221");
        assert_eq!(look_and_say(8), "1113213211");
        assert_eq!(look_and_say(9), "31131211131221");
    }
}
