struct Calculator;

impl Calculator {
    fn new() -> Self {
        Calculator
    }

    fn evaluate(&self, expression: &str) -> i32 {
        let parts: Vec<&str> = expression.split_whitespace().collect();
        let left: i32 = parts[0].parse().unwrap();
        let right: i32 = parts[2].parse().unwrap();
        left + right
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
}