struct Parser;

impl Parser {
    fn new() -> Self {
        Parser
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn instantiate() {
        let _ = Parser::new();
    }
}