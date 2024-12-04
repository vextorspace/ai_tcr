pub mod calculator;
pub mod parser {
    pub struct Parser;

    impl Parser {
        pub fn new() -> Self {
            Parser
        }
    }
}

#[cfg(test)]
mod tests {
    use super::parser::Parser;

    #[test]
    fn test_parser_creation() {
        let _ = Parser::new();
    }
}