#[macro_use(assert_approx_eq)] extern crate assert_approx_eq;
#[allow(dead_code)]

mod lib {

    /// Finds the minimum value of an array in O(n) time and O(1) space
    ///
    /// # Args:
    ///
    /// array - the concerned array
    ///
    /// # Returns:
    ///
    /// the minimum value from the array
    pub fn min<T>(array: &[T]) -> T
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
    pub fn max<T>(array: &[T]) -> T
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
        use std::cmp::{
            min,
            max,
        };

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

        use std::cmp::{
            min,
            max,
        };

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
}

#[cfg(test)]
mod tests_min;

#[cfg(test)]
mod tests_max;

#[cfg(test)]
mod tests_get_max_range;

#[cfg(test)]
mod tests_get_consecutive_max_range;
