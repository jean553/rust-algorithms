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
    fn test_min_with_u16() {

        let array: [u16; 6] = [
            60000,
            32546,
            10000,
            65535,
            222,
            8909,
        ];
        assert_eq!(min(&array), 222);
    }
}
