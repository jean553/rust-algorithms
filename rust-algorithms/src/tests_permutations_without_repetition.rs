#[cfg(test)]
mod tests {

    use lib::{
        permutations_without_repetition,
        get_all_permutations_without_repetition,
    };

    use std::collections::HashSet;

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

    #[test]
    fn test_get_all_permutations_without_repetition_with_three_items() {

        let array: [u8; 3] = [1, 2, 3];

        let mut result: HashSet<Vec<u8>> = HashSet::new();
        result.insert(vec![1, 2, 3]);
        result.insert(vec![1, 3, 2]);
        result.insert(vec![2, 1, 3]);
        result.insert(vec![2, 3, 1]);
        result.insert(vec![3, 1, 2]);
        result.insert(vec![3, 2, 1]);

        assert_eq!(
            get_all_permutations_without_repetition(&array),
            result,
        );
    }

    #[test]
    fn test_get_all_permutations_without_repetition_with_two_items() {

        let array: [u8; 2] = [1, 2];

        let mut result: HashSet<Vec<u8>> = HashSet::new();
        result.insert(vec![1, 2]);
        result.insert(vec![2, 1]);

        assert_eq!(
            get_all_permutations_without_repetition(&array),
            result,
        );
    }
}
