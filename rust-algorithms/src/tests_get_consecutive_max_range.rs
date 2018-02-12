#[cfg(test)]
mod tests {

    use lib::get_consecutive_max_range;

    #[test]
    fn test_get_consecutive_max_range() {

        let array: [i32; 7] = [0, 1, -2, 0, 3, 2, 5];
        assert_eq!(get_consecutive_max_range(&array), 5);
    }

    #[test]
    fn test_get_consecutive_max_range_revert_order() {

        let array: [i32; 7] = [5, 2, 3, 0, -2, 1, 0];
        assert_eq!(get_consecutive_max_range(&array), 5);
    }

    #[test]
    fn test_get_consecutive_max_range_highest_distance_at_beginning() {

        let array: [i32; 7] = [5, -8, 3, 0, -2, 1, 0];
        assert_eq!(get_consecutive_max_range(&array), 13);
    }

    #[test]
    fn test_get_consecutive_max_range_with_descending_array() {

        let array: [i32; 9] = [5, 4, 3, 2, 1, 0, -1, -2, -3];
        assert_eq!(get_consecutive_max_range(&array), 8);
    }
}
