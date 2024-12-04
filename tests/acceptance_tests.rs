pub struct Calculator;

impl Calculator {
    pub fn new() -> Self {
        Calculator
    }

    pub fn evaluate(&self, expression: &str) -> i32 {
        let parts: Vec<&str> = expression.split_whitespace().collect();
        let left: i32 = parts[0].parse().unwrap();
        let right: i32 = parts[2].parse().unwrap();
        left + right
    }
}

#[cfg(test)]
mod tests {
    use ai_tcr_string_calc::calc::calculator::Calculator;

    #[test]
    fn failing_test() {
        let calculator = Calculator::new();
        let result = calculator.evaluate("2 + 3");
        assert_eq!(result, 5);
    }
}