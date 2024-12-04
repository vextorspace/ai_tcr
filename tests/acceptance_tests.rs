#[cfg(test)]
mod tests {
    use ai_tcr_string_calc::calc::calculator::Calculator;

    #[test]
    fn addition() {
        let calculator = Calculator::new();
        let result = calculator.evaluate("2 + 3");
        assert_eq!(result, 5);
    }

    #[test]
    fn subtraction() {
        let calculator = Calculator::new();
        let result = calculator.evaluate("5 - 3");
        assert_eq!(result, 2);
    }
}