#[cfg(test)]
mod tests {

    use lib;

    #[test]
    fn test_min() {

        let array: [u8; 6] = [3, 5, 6, 1, 7, 4];
        assert_eq!(
            lib::min(&array),
            1,
            "Unexpected minimum value",
        );
    }
}
