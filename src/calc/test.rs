struct Test;

impl Test {
    fn new() -> Self {
        Test
    }

    fn add(&self, a: i32, b: i32) -> i32 {
        a + b
    }
}