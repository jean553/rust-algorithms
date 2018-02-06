#[cfg(test)]
mod tests {

    use lib::factorial;

    #[test]
    fn test_factorial() {

        let value: u32 = 4;
        let expected_result: u32 = 24;

        assert_eq!(
            expected_result,
            factorial(value),
        );
    }
}
