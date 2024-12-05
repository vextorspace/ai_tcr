enum Operation {
    PLUS,
}

impl Operation {
    fn operate(&self, a: i32, b: i32) -> i32 {
        match self {
            Operation::PLUS => a + b,
        }
    }

    fn symbol(&self) -> &str {
        match self {
            Operation::PLUS => "+",
        }
    }
}

struct OperationBlock;

impl OperationBlock {
    fn new() -> Self {
        OperationBlock
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_implementation() {
        let _ = OperationBlock::new();
    }
}