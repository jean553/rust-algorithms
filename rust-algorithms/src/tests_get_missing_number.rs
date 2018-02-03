#[cfg(test)]
mod tests {

    use lib::get_missing_number;

    #[test]
    fn test_get_missing_number() {

        let array: [u32; 4] = [4, 2, 1, 5];
        assert_eq!(get_missing_number(&array), 3);
    }
}
