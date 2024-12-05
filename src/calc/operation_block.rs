use crate::calc::{expression::Expression, operation::Operation};

pub struct OperationBlock {
    operation: Option<Operation>,
    operand1: Option<Expression>,
    operand2: Option<Expression>,
}

impl OperationBlock {
    pub fn new() -> Self {
        OperationBlock {
            operation: None,
            operand1: None,
            operand2: None,
        }
    }

    pub fn with_operation(mut self, op: Operation) -> Self {
        self.operation = Some(op);
        self
    }

    pub fn with_operand1(mut self, expr: Expression) -> Self {
        self.operand1 = Some(expr);
        self
    }

    pub fn with_operand2(mut self, expr: Expression) -> Self {
        self.operand2 = Some(expr);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::calc::expression::Expression;

    #[test]
    fn test_implementation() {
        let block = OperationBlock::new()
            .with_operation(Operation::PLUS)
            .with_operand1(Expression::new("1"))
            .with_operand2(Expression::new("2"));

        assert_eq!(block.operation, Some(Operation::PLUS));
        assert_eq!(block.operand1, Some(Expression::new("1")));
        assert_eq!(block.operand2, Some(Expression::new("2")));
    }
}
