pub fn is_palindrome(s: &str) -> bool {
    let mut chars = s
        .chars()
        .map(|c| c.to_ascii_lowercase())
        .filter(|c| c.is_alphanumeric());

    loop {
        match (chars.next(), chars.next_back()) {
            (None, _) => return true,
            (Some(_), None) => return true,
            (Some(left), Some(right)) => {
                if left != right {
                    return false;
                }
            }
        }
    }
}

pub fn is_palindrome_simple(s: &str) -> bool {
    let s = s.as_bytes();
    let mut left = 0;
    let mut right = s.len() - 1;

    while left < right {
        while !(s[left] as char).is_alphanumeric() && left < right {
            left += 1;
        }

        while !(s[right] as char).is_alphanumeric() && left < right {
            right -= 1;
        }

        if (s[left] as char).to_ascii_lowercase() != (s[right] as char).to_ascii_lowercase() {
            return false;
        }

        left += 1;
        right -= 1;
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(is_palindrome(&"A man, a plan, a canal, Panama."), true);
        assert_eq!(is_palindrome(&"Able was I, ere I saw Elba!"), true);
        assert_eq!(is_palindrome(&"Ray a Ray"), false);
    }
}
