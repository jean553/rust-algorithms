#[cfg(test)]
mod tests {

    use lib::get_sum_numbers_combinations_quantity_for_amount;

    #[test]
    fn tests_get_sum_numbers_combinations_quantity_for_amount() {

        let allowed_numbers: [u8; 3] = [1, 2, 3];
        let amount: u32 = 5;

        assert_eq!(
            get_sum_numbers_combinations_quantity_for_amount(
                &allowed_numbers,
                amount,
            ),
            5,
        );
    }

    #[test]
    fn tests_get_sum_numbers_combinations_quantity_for_amount_with_high_value() {

        let allowed_numbers: [u8; 3] = [2, 4, 5];
        let amount: u32 = 15;

        assert_eq!(
            get_sum_numbers_combinations_quantity_for_amount(
                &allowed_numbers,
                amount,
            ),
            4,
        );
    }
}
