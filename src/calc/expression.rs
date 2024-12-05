#[derive(Debug, PartialEq)]
pub struct Expression {
    expr: String,
}

impl Expression {
    pub fn new(expr: &str) -> Self {
        Expression {
            expr: expr.to_string(),
        }
    }

    pub fn evaluate(&self) -> Result<i32, std::fmt::Error> {
        self.expr.parse().map_err(|_| std::fmt::Error)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_expression() {
        let _expression = Expression::new("1 + 2 * 3");
    }

    #[test]
    fn test_positive_number_evaluates() {
        let expression = Expression::new("2");
        let value = expression.evaluate().unwrap();
        assert_eq!(value, 2);
    }

    #[test]
    fn test_negative_number_evaluates() {
        let expression = Expression::new("-2");
        let value = expression.evaluate();
        assert!(value.is_ok());
        assert_eq!(value.unwrap(), -2);
    }

    #[test]
    fn test_sum_evaluates_to_error() {
        let expression = Expression::new("1 + 2");
        let value = expression.evaluate();
        assert!(value.is_err());
    }

    #[test]
    fn test_two_equal_numbers_are_equal() {
        let expression1 = Expression::new("2");
        let expression2 = Expression::new("2");
        assert_eq!(expression1, expression2);
    }
}