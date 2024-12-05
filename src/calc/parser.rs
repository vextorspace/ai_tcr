struct Parser;

impl Parser {
    fn new() -> Self {
        Parser
    }

    fn parse(&self, expr: Expression) -> Result<i32, std::fmt::Error> {
        expr.evaluate()
    }
}

struct Expression {
    value: i32,
}

impl Expression {
    fn new(expr: &str) -> Self {
        let value = expr.parse::<i32>().unwrap_or(0);
        Expression { value }
    }

    fn evaluate(&self) -> Result<i32, std::fmt::Error> {
        Ok(self.value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn instantiate() {
        let _ = Parser::new();
    }

    #[test]
    fn parse_number_expression_gives_number() {
        let parser = Parser::new();
        let result = parser.parse(Expression::new("1"));
        assert_eq!(result, Ok(1));
    }
}