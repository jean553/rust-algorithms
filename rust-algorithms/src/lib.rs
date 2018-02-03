#[macro_use(assert_approx_eq)] extern crate assert_approx_eq;
#[allow(dead_code)]

mod lib {

    use std::cmp::{
        min,
        max,
    };

    use std::default::Default;

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

        let mut highest_distance = maximum - minimum;

        for current in array.iter() {

            minimum = min(
                minimum,
                *current,
            );

            maximum = max(
                maximum,
                *current,
            );

            /* if an higher distance than before has been found,
               set it as the highest one and reset the minimum
               and maximum value at the current iterated item
               (previous iterations do not matter anymore
               as a new highest distance has been found) */
            let current_distance = (maximum - minimum).abs();
            if current_distance > highest_distance {
                highest_distance = current_distance;
                minimum = *current;
                maximum = *current;
            }
        }

        highest_distance
    }

    /// Find the missing number into an array of consecutive numbers in O(n) time and O(1) space
    ///
    /// # Args:
    ///
    /// array - the concerned array
    ///
    /// # Returns:
    ///
    /// The missing value from the array.
    pub fn get_missing_number(array: &[u32]) -> u32 {

        /* calculate the expected array size: current size + 1
           because there is one missing item into the array */

        /* add one in order to get the expected size */
        let array_size = (array.len() + 1) as u32;

        /* calculate the current sum (O(n)) and the expected sum
           based on the formula n + (n-1) + (n-2) + ... + 1;
           calculate the difference between the two sums:
           this is the expected missing value */

        /* n(n+1)/2 = (n^2+n)/2 */
        let expected_sum = (array_size.pow(2) + array_size) / 2;
        let current_sum: u32 = array.iter().sum(); // O(n) time

        expected_sum - current_sum
    }

    /// Update array by calculating products of all items except the current one, takes O(n) time and O(n) space
    ///
    /// # Args:
    ///
    /// `array` - the concerned array
    ///
    /// # Returns:
    ///
    /// updated vector
    pub fn product_all_items_except_current(array: &[u32]) -> Vec<u32> {

        /* browse the array in both directions; during each browsing process,
           multiply the current iterated item by the product of all the items
           iterated before; the product is accumulated and reset between each iteration */

        let mut result = vec![1; array.len()];
        let mut product = 1;

        for (index, value) in result.iter_mut().enumerate() {

            if index == 0 {
                continue;
            }

            product *= array[index - 1];
            *value = product;
        }

        let mut index = array.len() - 1;
        product = 1;
        for (cursor, value) in result.iter_mut().rev().enumerate() {

            if cursor == 0 {
                continue;
            }

            product *= array[index];
            *value *= product;
            index -= 1;
        }

        result
    }

    /// Returns the maximum product of three numbers from array, taking O(n) time and O(1) space.
    ///
    /// # Args:
    ///
    /// `array` - the concerned array
    ///
    /// # Returns:
    ///
    /// maximum product of three numbers that can be found into the array
    pub fn get_max_product_of_three(array: &[i32]) -> i32 {

        /* browse the array once, keep track of the maximum product
           got with three items (starting with the three first items);
           keep also track of the product of two numbers (starting with the two first ones),
           keep track of the maximum product of these two numbers;
           at each iteration, check if the product of three numbers can be increased
           with the iterated value and also check if the product of two numbers can be increased
           with the iterated value; this second two numbers product is generated at each iteration
           if a higher product of two numbers can be found;
           there is also a product of two lowest numbers, in case of negative numbers */

        let mut highest = max(
            array[0],
            array[1],
        );
        let mut lowest = min(
            array[0],
            array[1],
        );

        let mut product_of_two = array[0] * array[1];
        let mut product_of_two_lowest = product_of_two;
        let mut product_of_three = product_of_two * array[2];

        for current in array.iter().skip(2) {

            product_of_three = max(
                max(
                    product_of_three,
                    product_of_two * *current,
                ),
                product_of_two_lowest * *current,
            );

            product_of_two = max(
                product_of_two,
                highest * *current,
            );

            product_of_two_lowest = max(
                product_of_two_lowest,
                lowest * *current,
            );

            highest = max(
                *current,
                highest,
            );

            lowest = min(
                *current,
                lowest,
            );
        }

        product_of_three
    }

    /// Merge ranges of tuples (start, end), taking O(n log n) time worst case (O(n) best case) and O(n) space.
    ///
    /// # Args:
    ///
    /// `array` - the concerned array
    ///
    /// # Returns:
    ///
    /// a new array with the merged items
    pub fn merge_ranges(ranges: &[(i32, i32); 5]) -> Vec<(i32, i32)> {

        /* sort the array first to make computation easier */

        let mut copied_ranges: [(i32, i32); 5] = Default::default();
        copied_ranges.copy_from_slice(ranges);
        copied_ranges.sort(); // O(n log n)

        let mut result: Vec<(i32, i32)> = Vec::new();
        result.push(copied_ranges[0]);

        /* for each item, check if the start is equal or lesser
           than the previous item end: if yes, merge the two;
           if no, append the current item into the result;
           as the array is sorted, we can browse it only once */

        for range in copied_ranges.iter().skip(1) {

            {
                let previous_range = &mut result.last_mut().unwrap();

                if range.0 <= previous_range.1 {
                    previous_range.1 = range.1;
                    continue;
                }
            }

            result.push(*range);
        }

        result
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

#[cfg(test)]
mod tests_product_all_items_except_current;

#[cfg(test)]
mod tests_get_max_product_of_three;

#[cfg(test)]
mod tests_merge_ranges;
