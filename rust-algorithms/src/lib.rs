#[macro_use(assert_approx_eq)] extern crate assert_approx_eq;
#[allow(dead_code)]

mod lib {

    use std::cmp::{
        min,
        max,
    };

    /// Finds the minimum value of an array in O(n) time and O(1) space
    ///
    /// # Args:
    ///
    /// array - the concerned array
    ///
    /// # Returns:
    ///
    /// the minimum value from the array
    pub fn min_value<T>(array: &[T]) -> T
        where T: PartialOrd + Copy
    {

        let mut minimum = array[0];

        for current in array.iter() {
            if *current < minimum {
                minimum = *current;
            }
        }

        minimum
    }

    /// Finds the maximum value of an array in O(n) time and O(1) space
    ///
    /// # Args:
    ///
    /// array - the concerned array
    ///
    /// # Returns:
    ///
    /// the maximum value from the array
    pub fn max_value<T>(array: &[T]) -> T
        where T: PartialOrd + Copy
    {
        let mut maximum = array[0];

        for current in array.iter() {
            if *current > maximum {
                maximum = *current;
            }
        }

        maximum
    }

    /// Finds the greatest possible distance between two items from array in O(n) time and O(1) space
    ///
    /// # Args:
    ///
    /// array - the concerned array
    ///
    /// # Returns:
    ///
    /// The maximum distance between two numbers of the array
    pub fn get_max_range(array: &[i32]) -> i32
    {
        let mut minimum = array[0];
        let mut maximum = array[1];

        for current in array.iter() {

            minimum = min(
                minimum,
                *current,
            );

            maximum = max(
                maximum,
                *current,
            );
        }

        (maximum - minimum).abs()
    }

    /// Find the greatest possible distance between consecutive items from array in O(n) time and O(1) space
    ///
    /// # Args:
    ///
    /// array - the concerned array
    ///
    /// # Returns:
    ///
    /// The maximum distance between two numbers of the array
    pub fn get_consecutive_max_range(array: &[i32]) -> i32 {

        let mut minimum = array[0];
        let mut maximum = array[1];

        let mut distance = maximum - minimum;

        for current in array.iter() {

            minimum = min(
                minimum,
                *current,
            );

            maximum = max(
                maximum,
                *current,
            );

            let updated_distance = max(
                distance,
                (maximum - minimum).abs(),
            );

            if updated_distance != distance {
                distance = updated_distance;
                minimum = *current;
                maximum = *current;
            }
        }

        distance
    }

    /// Find the missing number into an array of consecutive numbers in O(n) time and O(1) space
    ///
    /// # Args:
    ///
    /// array - the concerned array
    ///
    /// # Returns:
    ///
    /// The maximum distance between two numbers of the array
    pub fn get_missing_number(array: &[u32]) -> u32 {

        /* add one in order to get the expected size */
        let array_size = (array.len() + 1) as u32;

        /* n(n+1)/2 = (n^2+n)/2 */
        let expected_sum = (array_size.pow(2) + array_size) / 2;
        let current_sum: u32 = array.iter().sum();

        expected_sum - current_sum
    }
}

#[cfg(test)]
mod tests_min;

#[cfg(test)]
mod tests_max;

#[cfg(test)]
mod tests_get_max_range;

#[cfg(test)]
mod tests_get_consecutive_max_range;

#[cfg(test)]
mod tests_get_missing_number;
