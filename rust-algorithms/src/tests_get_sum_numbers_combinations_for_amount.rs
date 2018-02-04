#[cfg(test)]
mod tests {

    use lib::get_sum_numbers_combinations_for_amount;

    #[test]
    fn tests_get_sum_numbers_combinations_for_amount() {

        let allowed_numbers: [u8; 4] = [1, 2, 4, 5];
        let amount: u32 = 5;

        assert_eq!(
            get_sum_numbers_combinations_for_amount(
                &allowed_numbers,
                amount,
            ),
            vec![
                vec![1, 1, 1, 1, 1],
                vec![1, 1, 1, 2],
                vec![1, 2, 2],
                vec![1, 4],
                vec![5],
            ]
        );
    }

    #[test]
    fn tests_get_sum_numbers_combinations_for_amount_higher_amount() {

        let allowed_numbers: [u8; 3] = [2, 4, 5];
        let amount: u32 = 15;

        assert_eq!(
            get_sum_numbers_combinations_for_amount(
                &allowed_numbers,
                amount,
            ),
            vec![
                vec![2, 2, 2, 2, 2, 5],
                vec![2, 2, 2, 4, 5],
                vec![2, 4, 4, 5],
                vec![5, 5, 5],
            ]
        );
    }
}
