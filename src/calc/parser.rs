use crate::calc::expression::Expression;

struct Parser;

impl Parser {
    fn new() -> Self {
        Parser
    }

    fn parse(&self, expr: Expression) -> Result<i32, std::fmt::Error> {
        expr.evaluate()
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