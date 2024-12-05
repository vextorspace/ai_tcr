use super::operation::Operation;

pub struct OperationBlock {
    operation: Option<Operation>,
}

impl OperationBlock {
    pub fn new() -> Self {
        OperationBlock { operation: None }
    }

    pub fn with_operation(mut self, op: Operation) -> Self {
        self.operation = Some(op);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_implementation() {
        let _operation_block = OperationBlock::new().with_operation(Operation::PLUS);
    }
}
