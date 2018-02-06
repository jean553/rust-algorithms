#[cfg(test)]
mod tests {

    use lib::{
        permutations_with_repetitions,
        get_all_permutations,
    };

    use std::collections::HashSet;

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

    #[test]
    fn test_print_all_permutations_with_repetitions() {

        let array: [u8; 2] = [1, 2];

        let all_permutations = get_all_permutations(
            &array,
            2,
            true,
        );

        let mut result: HashSet<Vec<u8>> = HashSet::new();
        result.insert(vec![1, 1]);
        result.insert(vec![1, 2]);
        result.insert(vec![2, 1]);
        result.insert(vec![2, 2]);

        assert_eq!(
            all_permutations,
            result,
        );
    }

    #[test]
    fn test_print_some_permutations_with_repetitions() {

        let array: [u8; 3] = [1, 2, 3];

        let all_permutations = get_all_permutations(
            &array,
            2,
            true,
        );

        let mut result: HashSet<Vec<u8>> = HashSet::new();
        result.insert(vec![1, 1]);
        result.insert(vec![1, 2]);
        result.insert(vec![1, 3]);
        result.insert(vec![2, 1]);
        result.insert(vec![2, 2]);
        result.insert(vec![2, 3]);
        result.insert(vec![3, 1]);
        result.insert(vec![3, 2]);
        result.insert(vec![3, 3]);

        assert_eq!(
            all_permutations,
            result,
        );
    }
}
