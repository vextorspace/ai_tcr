enum Operation {
    PLUS,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn instantiate() {
        let plus = Operation::PLUS;
    }
}