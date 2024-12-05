use super::operation::Operation;
use super::operation_block::OperationBlock;
use std::fmt;

#[derive(Debug, PartialEq, Clone)]
pub struct Expression {
    expr: String,
}

impl Expression {
    pub fn new(expr: &str) -> Self {
        Expression {
            expr: expr.trim().to_string(),
        }
    }

    pub fn evaluate(&self) -> Result<i32, std::fmt::Error> {
        self.expr.parse().map_err(|_| std::fmt::Error)
    }

    pub fn find_operation(&self, op: Operation) -> Result<OperationBlock, std::fmt::Error> {
        if self.expr.contains(op.as_str()) {
            let operation_block = OperationBlock::new().with_operation(op);
            Ok(operation_block)
        } else {
            Err(std::fmt::Error)
        }
    }
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.expr)
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

    #[test]
    fn test_two_different_numbers_are_not_equal() {
        let expression1 = Expression::new("2");
        let expression2 = Expression::new("3");
        assert_ne!(expression1, expression2);
    }

    #[test]
    fn test_two_same_numbers_but_different_whitespace_are_equal() {
        let expression1 = Expression::new("2");
        let expression2 = Expression::new(" 2 ");
        assert_eq!(expression1, expression2);
    }

    #[test]
    fn test_can_make_string() {
        let expression = Expression::new("2 + 3");
        assert_eq!(expression.to_string(), "2 + 3".to_string());
    }

    #[test]
    fn find_operation_failure_gives_error() {
        let expression = Expression::new("1 - 2");
        let value = expression.find_operation(Operation::PLUS);
        assert!(value.is_err());
    }

    #[test]
    fn find_operation_success_gives_operation_block() {
        let expression = Expression::new("1 + 2");
        let value: Result<OperationBlock, std::fmt::Error> =
            expression.find_operation(Operation::PLUS);
        assert!(value.is_ok());

        let block = value.unwrap();
        assert_eq!(block.operation, Some(Operation::PLUS));
    }
}