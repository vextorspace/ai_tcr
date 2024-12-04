pub struct Calculator;

impl Calculator {
    pub fn new() -> Self {
        Calculator
    }

    pub fn evaluate(&self, expression: &str) -> i32 {
        let parts: Vec<&str> = expression.split_whitespace().collect();
        if parts.len() == 1 {
            return parts[0].parse().unwrap();
        }
        let left: i32 = parts[0].parse().unwrap();
        let right: i32 = parts[2].parse().unwrap();
        match parts[1] {
            "+" => left + right,
            "-" => left - right,
            "*" => left * right,
            "/" => left / right,
            _ => panic!("Unsupported operation"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let calculator = Calculator::new();
        assert_eq!(calculator.evaluate("2 + 3"), 5);
    }

    #[test]
    fn test_subtract() {
        let calculator = Calculator::new();
        assert_eq!(calculator.evaluate("5 - 3"), 2);
    }

    #[test]
    fn test_multiply() {
        let calculator = Calculator::new();
        assert_eq!(calculator.evaluate("4 * 3"), 12);
    }

    #[test]
    fn test_divide() {
        let calculator = Calculator::new();
        assert_eq!(calculator.evaluate("6 / 2"), 3);
    }

    #[test]
    fn test_single_number() {
        let calculator = Calculator::new();
        assert_eq!(calculator.evaluate("5"), 5);
    }
}