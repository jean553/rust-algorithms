#[cfg(test)]
mod tests {

    use lib::fibonacci_recursive;

    #[test]
    fn test_fibonacci_recursive() {

        assert_eq!(
            fibonacci_recursive(6),
            8, // 0, 1, 1, 2, 3, 5, 8
        );
    }
}
