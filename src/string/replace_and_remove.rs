pub fn replace_and_remove(s: &str) -> String {
    let mut result = String::new();

    for c in s.chars() {
        match c {
            'a' => result += "dd",
            'b' => continue,
            _ => result.push(c),
        }
    }

    result
}

pub fn replace_and_remove_in_place(s: &mut str, size: usize) {
    let size = remove_b(unsafe { s.as_bytes_mut() }, size);
    let final_size = size + s.chars().filter(|&c| c == 'a').count();
    let mut insertion_i = final_size;
    let bytes = unsafe { s.as_bytes_mut() };

    for i in (0..size).rev() {
        insertion_i -= 1;
        if bytes[i] == b'a' {
            bytes[insertion_i] = b'd';
            insertion_i -= 1;
            bytes[insertion_i] = b'd';
        } else {
            bytes[insertion_i] = bytes[i];
        }
    }
}

pub fn remove_b(s: &mut [u8], size: usize) -> usize {
    let mut insertion_i = 0;
    let mut new_size = size;

    for i in 0..size {
        if s[i] == b'b' {
            new_size -= 1;
        } else {
            s.swap(insertion_i, i);
            insertion_i += 1;
        }
    }

    new_size
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(replace_and_remove(&"abac"), "ddddc");
    }

    #[test]
    fn test2() {
        let mut s = "ababbccabc___".to_string();
        replace_and_remove_in_place(&mut s, 10);
        assert_eq!(&s[0..9], "ddddccddc");
    }
}
