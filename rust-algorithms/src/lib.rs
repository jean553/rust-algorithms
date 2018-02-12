#[macro_use(assert_approx_eq)] extern crate assert_approx_eq;
#[allow(dead_code)]

mod lib {

    use std::cmp::{
        min,
        max,
    };

    use std::default::Default;

    use std::collections::HashSet;

    pub enum LinkedListNode {
        Next(
            u8,
            Box<LinkedListNode>,
        ),
        End,
    }

    /// Returns the item at the given index from the linked list
    ///
    /// # Args:
    ///
    /// `node` - reference to the linked list to use
    /// `index` - the index of the item to get
    ///
    /// # Returns:
    ///
    /// the data at the given index
    pub fn at_linked_list(
        mut node: &LinkedListNode,
        index: usize,
    ) -> u8 {

        for _ in 0..index {

            node = match node {
                &LinkedListNode::Next(_, ref next) => &*next,
                &LinkedListNode::End => &LinkedListNode::End,
            };
        }

        if let LinkedListNode::Next(data, ref _next) = *node {
            return data;
        }

        /* FIXME: should not return 0 */
        0
    }

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

        maximum - minimum
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

    /// Get the quantity of numbers combinations to make the sum for a given amount, taking O(n * m) time and O(n) space.
    ///
    /// # Args:
    ///
    /// `allowed_numbers` - the numbers allowed to make the sum
    /// `amount` - the amount for which one combinations of sums amount must be found
    ///
    /// # Returns:
    ///
    /// Quantity of possible sums with given numbers for the specified amount
    pub fn get_sum_numbers_combinations_quantity_for_amount(
        allowed_numbers: &[u8],
        amount: u32,
    ) -> u32 {

        /* add 1 because 0 must be considered */
        let mut results: Vec<u32> = vec![0; (amount + 1) as usize];

        /* there is one combination to have an amount of 0 */
        results[0] = 1;

        for allowed_number in allowed_numbers.iter() {

            for index in 0..results.len() {

                if index < *allowed_number as usize {
                    continue;
                }

                results[index] += results[index - *allowed_number as usize];
            }
        }

        results[amount as usize]
    }

    /// Returns all the combinations of numbers for which the sums make the given amount suing the given allowed numbers
    ///
    /// # Args:
    ///
    /// `allowed_numbers` - the numbers allowed to make the sum
    /// `amount` - the amount for which one combinations of sums amount must be found
    ///
    /// # Returns:
    ///
    /// Possible combinations with given numbers for the specified amount
    pub fn get_sum_numbers_combinations_for_amount(
        allowed_numbers: &[u8],
        amount: u32,
    ) -> Vec<Vec<u32>> {

        let mut results: Vec<Vec<Vec<u32>>> = vec![
            vec![];
            (amount + 1) as usize
        ];

        for allowed_number in allowed_numbers.iter() {

            for index in 0..results.len() {

                if index < *allowed_number as usize {
                    continue;
                }

                let previous_index = index - *allowed_number as usize;

                if previous_index == 0 {
                    results[index].push(vec![*allowed_number as u32]);
                    continue;
                }

                let mut new_combinations = results[previous_index].clone();
                for new_combination in new_combinations.iter_mut() {
                    new_combination.push(*allowed_number as u32);
                    results[index].push(new_combination.clone());
                }
            }
        }

        results[amount as usize].clone()
    }

    /// Returns the number of possible permutations (order matters) from a set when repetitions are allowed
    ///
    /// # Args:
    ///
    /// `total_items_amount` - the total items amount from the source set
    /// `selection_items_amount` - the amount of items to select into one permutation
    ///
    /// # Returns:
    ///
    /// The possible amount of permutations
    pub fn permutations_with_repetitions(
        total_items_amount: u32,
        selection_items_amount: u32,
    ) -> u32 {
        /* permutations amount = total_amount^selection_amount */
        total_items_amount.pow(selection_items_amount)
    }

    /// Prints all the permutations (with repetitions) for the given array.
    ///
    /// # Args:
    ///
    /// `array` - the source array to use with all the items
    /// `selection_amount` - the selection items amount (max = array length)
    ///
    /// # Returns:
    ///
    /// set with all the permutations with repetitions
    pub fn get_all_permutations(
        array: &[u8],
        selection_amount: usize,
        repetitions_allowed: bool,
    ) -> HashSet<Vec<u8>> {

        let length = array.len();

        let mut buffer: Vec<u8> = vec![0; length as usize];
        let mut results: HashSet<Vec<u8>> = HashSet::new();
        let depth = 0;

        let browsed: Vec<u8> = Vec::new();

        permutations(
            &array,
            &mut buffer,
            &mut results,
            length,
            depth,
            repetitions_allowed,
            browsed,
        );

        let mut filtered_results: HashSet<Vec<u8>> = HashSet::new();

        for result in results.iter() {

            let mut result = (*result).clone();
            result.truncate(selection_amount);
            filtered_results.insert(result);
        }

        filtered_results
    }

    /// Recursive function for permutations calculation.
    ///
    /// # Args:
    ///
    /// `array` - the array with all the items of the set
    /// `buffer` - mutable reference to the buffer used to store every single permutation
    /// `results` - the set where to insert the solution one by one (guarantees unicity)
    /// `length` - the length of the items array
    /// `depth` - the current depth during browsing of solutions
    /// `repetitions_allowed` - indicates if repetitions are allowed into the results
    /// `browsed` - the browsed items into the current recursion (prevents repetitions if required)
    fn permutations(
        array: &[u8],
        mut buffer: &mut Vec<u8>,
        mut results: &mut HashSet<Vec<u8>>,
        length: usize,
        depth: usize,
        repetitions_allowed: bool,
        browsed: Vec<u8>,
    ) {

        for value in array.iter() {

            /* FIXME: bad design, if repetitions are not allowed,
               we simply create an useless container for every iteration */
            let mut new_browsed: Vec<u8> = browsed.clone();

            if depth < length {

                if !repetitions_allowed {

                    if new_browsed.contains(value) {
                        continue;
                    }

                    new_browsed.push(*value);
                }

                buffer[depth as usize] = *value;

                permutations(
                    &array,
                    &mut buffer,
                    &mut results,
                    length,
                    depth + 1,
                    repetitions_allowed,
                    new_browsed.clone(),
                );
            } else {

                let mut one_result: Vec<u8> = vec![0; length];
                one_result.clone_from_slice(buffer);
                results.insert(one_result);
            }
        }
    }

    /// Recursive factorial calculation
    ///
    /// # Args:
    ///
    /// `value` - the value to compute
    ///
    /// # Returns:
    ///
    /// factorial result
    pub fn factorial(value: u32) -> u32 {

        if value == 1 {
            return 1;
        }

        return value * factorial(value - 1);
    }

    /// Returns the number of possible permutations (order matters) from a set when repetitions are not allowed
    ///
    /// # Args:
    ///
    /// `total_items_amount` - the total items amount from the source set
    /// `selection_items_amount` - the amount of items to select into one permutation
    ///
    /// # Returns:
    ///
    /// The possible amount of permutations
    pub fn permutations_without_repetition(
        total_items_amount: u32,
        selection_items_amount: u32,
    ) -> u32 {

        (
            factorial(total_items_amount) /
            factorial(total_items_amount - selection_items_amount)
        )
    }

    /// Returns the number at the given index into the fibonacci serie.
    ///
    /// # Args:
    ///
    /// `index` - the fibonacci index
    ///
    /// # Returns:
    ///
    /// the fibonacci number
    pub fn fibonacci_recursive(index: u32) -> u32 {

        if index == 0 || index == 1 {
            return index;
        }

        return
            fibonacci_recursive(index - 1) +
            fibonacci_recursive(index - 2)
        ;
    }
}

#[cfg(test)]
mod tests_min;
mod tests_max;
mod tests_get_max_range;
mod tests_get_consecutive_max_range;
mod tests_get_missing_number;
mod tests_product_all_items_except_current;
mod tests_get_max_product_of_three;
mod tests_merge_ranges;
mod tests_get_sum_numbers_combinations_quantity_for_amount;
mod tests_get_sum_numbers_combinations_for_amount;
mod tests_permutations_with_repetitions;
mod tests_factorial;
mod tests_permutations_without_repetition;
mod tests_remove_from_linked_list;
mod tests_fibonacci;
