pub struct OperationBlock;

impl OperationBlock {
    pub fn new() -> Self {
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
