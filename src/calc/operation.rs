enum Operation {
    PLUS,
}

impl Operation {
    fn operate(&self, a: i32, b: i32) -> i32 {
        match self {
            Operation::PLUS => a + b,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn instantiate() {
        let plus = Operation::PLUS;
    }

    #[test]
    fn plus() {
        let plus = Operation::PLUS;
        let result = plus.operate(2, 3);
        assert_eq!(result, 5);
    }
}