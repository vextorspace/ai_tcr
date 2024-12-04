fn add(a: i32, b: i32) -> i32 {
    a + b
}

struct Calculator;

impl Calculator {
    fn new() -> Self {
        Calculator
    }

    fn evaluate(&self, expression: &str) -> i32 {
        let parts: Vec<&str> = expression.split_whitespace().collect();
        let a: i32 = parts[0].parse().unwrap();
        let b: i32 = parts[2].parse().unwrap();
        add(a, b)
    }
}

#[test]
fn failing_test() {
    let calculator = Calculator::new();

    calculator.evaluate("2 + 3");
}