#[cfg(test)]
mod tests {
    // Import the necessary crates and functions
    use rstest::rstest;

    // Write your test functions
    #[rstest]
    fn test_addition() {
        assert_eq!(2 + 2, 4);
    }

    #[rstest]
    fn test_subtraction() {
        assert_eq!(4 - 2, 2);
    }
}