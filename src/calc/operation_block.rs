use super::operation::Operation;
use crate::calc::expression::Expression;

pub struct OperationBlock {
    operation: Option<Operation>,
    operand1: Option<Expression>,
}

impl OperationBlock {
    pub fn new() -> Self {
        OperationBlock { 
            operation: None,
            operand1: None,
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
}

#[cfg(test)]
mod tests {
    use crate::calc::expression::Expression;
    use super::*;

    #[test]
    fn test_implementation() {
        let _operation_block = OperationBlock::new()
            .with_operation(Operation::PLUS)
            .with_operand1(Expression::new("1"));
    }
}