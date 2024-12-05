#[derive(Debug, PartialEq, Clone)]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn instantiate() {
        let _plus = Operation::PLUS;
    }

    #[test]
    fn plus_operation() {
        let plus = Operation::PLUS;
        let result = plus.operate(2, 3);
        assert_eq!(result, 5);
    }

    #[test]
    fn plus_symbol() {
        let plus = Operation::PLUS;
        let symbol = plus.symbol();
        assert_eq!(symbol, "+");
    }

    #[test]
    fn plus_clones() {
        let plus = Operation::PLUS;
        let _plus_clone = plus.clone();
    }
}