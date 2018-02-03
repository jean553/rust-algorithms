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
    pub fn min(array: &[u8]) -> u8 {

        let mut minimum = array[0];

        for current in array.iter() {
            if *current < minimum {
                minimum = *current;
            }
        }

        minimum
    }
}

#[cfg(test)]
mod tests_min;
