#[cfg(test)]
mod tests {

    use lib::max;

    #[test]
    fn test_max_with_u8() {

        let array: [u8; 6] = [254, 125, 78, 140, 88, 87];
        assert_eq!(max(&array), 254);
    }

    #[test]
    fn test_max_with_f32() {

        let array: [f32; 6] = [-56.56, 67.32, 32.99, 109.12, -67.78, 98.43];
        assert_approx_eq!(max(&array), 109.12);
    }
}
