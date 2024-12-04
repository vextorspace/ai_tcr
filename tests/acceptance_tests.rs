#[test]
fn failing_test() {
    let calculator = Calculator::new();

    let result = calculator.evaluate("2 + 3");

    assert_eq!(result, 5);
}