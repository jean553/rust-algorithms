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

    #[test]
    fn test_min_with_f32() {

        let array: [f32; 6] = [
            -56.56,
            67.32,
            32.99,
            109.12,
            -67.78,
            98.43,
        ];
        assert_approx_eq!(min(&array), -67.78);
    }
}
