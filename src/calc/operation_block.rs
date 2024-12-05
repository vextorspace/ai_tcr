mod calc {
    pub mod operation {
        pub enum Operation {
            PLUS,
        }

        impl Operation {
            pub fn operate(&self, a: i32, b: i32) -> i32 {
                match self {
                    Operation::PLUS => a + b,
                }
            }

            pub fn symbol(&self) -> &str {
                match self {
                    Operation::PLUS => "+",
                }
            }
        }
    }

    pub mod operation_block {
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
    }
}

#[cfg(test)]
mod tests {
    use super::calc::operation::Operation;
    use super::calc::operation_block::OperationBlock;

    #[test]
    fn test_implementation() {
        let _operation_block = OperationBlock::new().with_operation(Operation::PLUS);
    }
}