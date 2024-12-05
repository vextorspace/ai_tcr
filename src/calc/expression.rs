struct Expression {
    expr: String,
}

impl Expression {
    fn new(expr: &str) -> Self {
        Expression {
            expr: expr.to_string(),
        }
    }

    fn evaluate(&self) -> i32 {
        self.expr.parse().unwrap_or(0)
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
    fn test_positive_number_evaluates() {
        let expression = Expression::new("2");
        let value = expression.evaluate();
        assert_eq!(value, 2);
    }

    #[test]
    fn test_negative_number_evaluates() {
        let expression = Expression::new("-2");
        let value = expression.evaluate();
        assert_eq!(value, -2);
    }
}