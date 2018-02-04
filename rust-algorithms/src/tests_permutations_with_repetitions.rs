#[cfg(test)]
mod tests {

    use lib::permutations_with_repetitions;

    #[test]
    fn test_permutations_with_repetitions() {

        let total_amount: u32 = 10;
        let selection_amount: u32 = 4;

        assert_eq!(
            permutations_with_repetitions(
                total_amount,
                selection_amount,
            ),
            10000,
        );
    }
}
