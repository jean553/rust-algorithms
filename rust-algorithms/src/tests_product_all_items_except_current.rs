#[cfg(test)]
mod tests {

    use lib::product_all_items_except_current;

    #[test]
    fn test_product_all_items_except_current() {

        let array: [u32; 4] = [2, 5, 3, 4];
        let result: Vec<u32> = vec![60, 24, 40, 30];

        assert_eq!(
            product_all_items_except_current(&array),
            result,
        );
    }
}
