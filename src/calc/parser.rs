use crate::calc::expression::Expression;

pub struct Parser;

impl Parser {
    pub fn new() -> Self {
        Parser
    }

    pub fn evaluate(&self, expr: Expression) -> Result<i32, std::fmt::Error> {
        expr.evaluate()
    }

    pub fn parse(&self, expr: Expression) -> Result<(), std::fmt::Error> {
        // Placeholder implementation
        Ok(())
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
    fn evaluate_number_expression_gives_number() {
        let parser = Parser::new();
        let result = parser.evaluate(Expression::new("1"));
        assert_eq!(result, Ok(1));
    }

    #[test]
    fn parse_sum_expression_gives_block_op() {
        let parser = Parser::new();
        let _op_block = parser.parse(Expression::new("1+2"));
    }
}