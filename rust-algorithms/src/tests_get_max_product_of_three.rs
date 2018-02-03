#[cfg(test)]
mod tests {

    use lib::get_max_product_of_three;

    #[test]
    fn test_get_max_product_of_three() {

        let array: [i32; 5] = [4, 3, 2, 5, 6];
        assert_eq!(get_max_product_of_three(&array), 120);
    }

    #[test]
    fn test_get_max_product_of_three_with_negatives() {

        let array: [i32; 5] = [-4, 3, -2, -5, 6];
        assert_eq!(get_max_product_of_three(&array), 120);
    }
}
