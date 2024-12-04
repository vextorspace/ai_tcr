struct Calculator;

impl Calculator {
    fn new() -> Self {
        Calculator
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn instantiate() {
        let _ = Calculator::new();
    }
}