pub struct Calculator;

impl Calculator {
    pub fn new() -> Self {
        Calculator
    }

    pub fn evaluate(&self, _expression: &str) -> String {
        "result".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn instantiate() {
        let _ = Calculator::new();
    }

    #[test]
    fn evaluate_returns_string() {
        let calc = Calculator::new();
        let result: String = calc.evaluate("2+3");
        assert!(!result.is_empty())
    }
}