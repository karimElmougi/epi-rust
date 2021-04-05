pub fn reverse_order_of_words_naive(s: &str) -> String {
    s.split_whitespace().rev().collect::<Vec<_>>().join(" ")
}

pub fn reverse_order_of_words(s: &mut str) {
    unsafe { s.as_bytes_mut() }.reverse();

    for word in s.split_ascii_whitespace() {
        let ptr = word as *const str as *mut str;
        let word = unsafe { &mut *ptr };
        unsafe { word.as_bytes_mut() }.reverse();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_naive() {
        assert_eq!(
            reverse_order_of_words_naive(&"Alice likes Bob"),
            "Bob likes Alice"
        );
    }

    #[test]
    fn test() {
        let mut s = "Alice likes Bob".to_string();
        reverse_order_of_words(&mut s);
        assert_eq!(s, "Bob likes Alice");
    }
}
