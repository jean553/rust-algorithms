#[cfg(test)]
mod tests {

    use lib::min;

    #[test]
    fn test_min_with_u8() {

        let array: [u8; 6] = [
            255,
            125,
            78,
            140,
            88,
            87,
        ];

        assert_eq!(min(&array), 78);
    }

    #[test]
    fn test_min_with_i8() {

        let array: [i8; 6] = [
            -56,
            67,
            32,
            109,
            -67,
            98,
        ];
        assert_eq!(min(&array), -67);
    }
}
