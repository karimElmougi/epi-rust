use Bracket::{Curly, Paren, Square};

#[derive(Copy, Clone)]
enum Bracket {
    Square,
    Paren,
    Curly,
}

impl Bracket {
    fn is_square(self) -> bool {
        match self {
            Square => true,
            _ => false,
        }
    }

    fn is_paren(self) -> bool {
        match self {
            Paren => true,
            _ => false,
        }
    }

    fn is_curly(self) -> bool {
        match self {
            Curly => true,
            _ => false,
        }
    }
}

pub fn is_balanced(string: &str) -> bool {
    let mut brackets = vec![];

    for c in string.chars() {
        match c {
            '[' => brackets.push(Square),
            '(' => brackets.push(Paren),
            '{' => brackets.push(Curly),
            ']' => {
                if brackets.last().map_or(false, |b| b.is_square()) {
                    brackets.pop();
                } else {
                    return false;
                }
            }
            ')' => {
                if brackets.last().map_or(false, |b| b.is_paren()) {
                    brackets.pop();
                } else {
                    return false;
                }
            }
            '}' => {
                if brackets.last().map_or(false, |b| b.is_curly()) {
                    brackets.pop();
                } else {
                    return false;
                }
            }
            _ => unreachable!(),
        }
    }

    brackets.is_empty()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_naive() {
        assert_eq!(is_balanced(""), true);
        assert_eq!(is_balanced(")"), false);
        assert_eq!(is_balanced("("), false);
        assert_eq!(is_balanced("()"), true);
        assert_eq!(is_balanced("(()[])"), true);
        assert_eq!(is_balanced("((){[}])"), false);
    }
}
