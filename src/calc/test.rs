struct Test;

impl Test {
    fn new() -> Self {
        Test
    }

    fn add(&self, a: i32, b: i32) -> i32 {
        a + b
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn good_test() {
        assert_eq!(1, 1);
    }

    #[test]
    fn add_test() {
        let result = Test::new().add(1, 2);
        assert_eq!(result, 3);
    }
}