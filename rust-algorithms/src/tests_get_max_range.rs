#[cfg(test)]
mod tests {

    use lib::get_max_range;

    #[test]
    fn test_get_max_range() {

        let array: [i32; 7] = [
            -15,
            110,
            78,
            32,
            -45,
            120,
            -50,
        ];

        assert_eq!(get_max_range(&array), 170);

        let array: [i32; 7] = [0, 1, -2, 0, 3, 2, 5];
        assert_eq!(get_max_range(&array), 7);
    }

    #[test]
    fn test_get_max_range_only_signed() {

        let array: [i32; 7] = [
            -15,
            -110,
            -78,
            -32,
            -45,
            -120,
            -50,
        ];

        assert_eq!(get_max_range(&array), 105);
    }
}
