pub fn to_int(s: &str) -> i32 {
    if s.is_empty() {
        return 0;
    }

    let mut n = 0i32;
    let is_negative = s.chars().nth(0).unwrap() == '-';
    let s = if is_negative { &s[1..] } else { s };

    for (rad, c) in s.chars().rev().enumerate() {
        n += c.to_digit(10).unwrap() as i32 * 10i32.pow(rad as u32);
    }

    if is_negative {
        -n
    } else {
        n
    }
}

pub fn from_int(n: i32) -> String {
    let mut s = String::new();
    let is_negative = n < 0;
    let mut n = n.abs();

    while n != 0 {
        let rem = (n % 10) as u32;
        n /= 10;
        s.push(std::char::from_digit(rem, 10).unwrap());
    }

    if is_negative {
        s.push('-');
    }

    s.chars().rev().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let s = "-123";
        assert_eq!(from_int(to_int(s)), s);
    }

    #[test]
    fn test_to_int() {
        assert_eq!(to_int("-123"), -123);
    }

    #[test]
    fn test_from_int() {
        assert_eq!(from_int(-123), "-123");
    }
}
