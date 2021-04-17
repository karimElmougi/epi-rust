use Operation::{Add, Div, Mul, Sub};

enum Operation {
    Add,
    Sub,
    Mul,
    Div,
}

pub fn evaluate(expression: &str) -> i32 {
    let mut value_stack = vec![];
    for s in expression.split(',') {
        match s {
            "+" => apply(Add, &mut value_stack),
            "-" => apply(Sub, &mut value_stack),
            "x" => apply(Mul, &mut value_stack),
            "/" => apply(Div, &mut value_stack),
            s => value_stack.push(s.parse::<i32>().unwrap()),
        }
    }
    *value_stack.last().unwrap()
}

fn apply(op: Operation, value_stack: &mut Vec<i32>) {
    let rhs = value_stack.pop().unwrap();
    let lhs = value_stack.pop().unwrap();
    let result = match op {
        Add => lhs + rhs,
        Sub => lhs - rhs,
        Mul => lhs * rhs,
        Div => lhs / rhs,
    };
    value_stack.push(result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_naive() {
        assert_eq!(evaluate("0"), 0);
        assert_eq!(evaluate("-1"), -1);
        assert_eq!(evaluate("2,3,x"), 6);
        assert_eq!(evaluate("3,4,+,2,x,1,+"), 15);
    }
}
