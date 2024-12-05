struct Expression;

impl Expression {
    fn new(_expr: &str) -> Self {
        Expression
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_expression() {
        let expression = Expression::new("1 + 2 * 3");
    }
}