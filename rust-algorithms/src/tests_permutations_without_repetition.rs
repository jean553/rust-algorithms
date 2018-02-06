#[cfg(test)]
mod tests {

    use lib::permutations_without_repetition;

    #[test]
    fn test_permutations_without_repetition() {

        let total_items_amount = 5;
        let selection_items_amount = 3;

        assert_eq!(
            permutations_without_repetition(
                total_items_amount,
                selection_items_amount,
            ),
            60,
        );
    }
}
