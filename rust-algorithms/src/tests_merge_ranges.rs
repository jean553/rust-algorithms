#[cfg(test)]
mod tests {

    use lib::merge_ranges;

    #[test]
    fn test_merge_ranges() {

        let array: [(i32, i32); 5] = [
            (4, 7),
            (3, 5),
            (8, 10),
            (2, 3),
            (-6, -3),
        ];

        let expected: Vec<(i32, i32)> = vec![
            (-6, -3),
            (2, 7),
            (8, 10),
        ];

        assert_eq!(
            merge_ranges(&array),
            expected,
        );
    }
}
