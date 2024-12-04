#[test]
fn addition() {
    use ai_tcr_string_calc::calc::calculator::Calculator;

    let calc = Calculator::new();
    let result = calc.evaluate("2+3");
    assert_eq!(result, "5");
}
