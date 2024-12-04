pub struct Calculator;

impl Calculator {
    pub fn new() -> Self {
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
