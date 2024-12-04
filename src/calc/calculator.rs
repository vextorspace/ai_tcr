pub struct Calculator;

impl Calculator {
    pub fn new() -> Self {
        Calculator
    }

    pub fn evaluate(&self, expression: &str) -> i32 {
        let parts: Vec<&str> = expression.split_whitespace().collect();
        let left: i32 = parts[0].parse().unwrap();
        let right: i32 = parts[2].parse().unwrap();
        match parts[1] {
            "+" => left + right,
            "-" => left - right,
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
}