use std::iter::Peekable;

pub fn from_roman(s: &str) -> i32 {
    let mut n = 0;

    let mut chars = s.chars().peekable();
    while let Some(c) = chars.next() {
        n += match c {
            'I' => match maybe_next(&mut chars, &['V', 'X']) {
                'V' => 4,
                'X' => 9,
                _ => 1,
            },
            'V' => 5,
            'X' => match maybe_next(&mut chars, &['L', 'C']) {
                'L' => 40,
                'C' => 90,
                _ => 10,
            },
            'L' => 50,
            'C' => match maybe_next(&mut chars, &['D', 'M']) {
                'D' => 400,
                'M' => 900,
                _ => 100,
            },
            'D' => 500,
            'M' => 1000,
            _ => 0,
        };
    }

    n
}

fn maybe_next<Iter>(it: &mut Peekable<Iter>, chars: &[char]) -> char
where
    Iter: Iterator<Item = char>,
{
    it.next_if(|c| chars.contains(c)).unwrap_or('_')
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(from_roman("XXXXXIIIIIIIII"), 59);
        assert_eq!(from_roman("XC"), 90);
    }
}
