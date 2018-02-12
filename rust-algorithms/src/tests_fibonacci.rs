#[cfg(test)]
mod tests {

    use lib::{
        fibonacci_recursive,
        fibonacci_recursive_with_memoization,
    };

    #[test]
    fn test_fibonacci_recursive() {

        assert_eq!(
            fibonacci_recursive(6),
            8, // 0, 1, 1, 2, 3, 5, 8
        );
    }

    #[test]
    fn test_fibonacci_recursive_with_memoization() {

        assert_eq!(
            fibonacci_recursive_with_memoization(6),
            8,
        );
    }
}
