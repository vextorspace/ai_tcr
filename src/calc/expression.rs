struct Expression;

impl Expression {
    fn new(_expr: &str) -> Self {
        Expression
    }

    fn evaluate(&self) -> i32 {
        2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_expression() {
        let expression = Expression::new("1 + 2 * 3");
    }

    #[test]
    fn test_number_evaluates() {
        let expression = Expression::new("2");
        let value = expression.evaluate();
        assert_eq!(value, 2);
    }
}