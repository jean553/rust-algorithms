# rust-algorithms

## Create the container

```sh
vagrant up
```

## Connect to the container

```sh
vagrant ssh
```

## Build the library

```sh
cargo build --release
```

## Run the unit tests

```sh
cargo test
```

## Generate documentation

```sh
cargo rustdoc -- --no-defaults
```

## Implemented methods

### min_value(array)

Returns the minimum value from an array, O(n) time and O(1) space.

### max_value(array)

Returns the maximum value from an array, O(n) time and O(1) space.

### get_max_range(array)

Returns the highest distance possible between two items of the array, O(n) time and O(1) space.

```rust
let array: [u8; 7] = [0, 1, -2, 0, 3, 2, 5];
let max_distance = get_max_range(&array); // 7
```

### get_max_consecutive_range(array)

Returns the highest distance possible between consecutive items of the array, O(n) time and O(1) space.

```rust
let array: [u8; 7] = [0, 1, -2, 0, 3, 2, 5];
let max_distance = get_max_consecutive_range(&array); // 5
```

### get_missing_value(array)

Returns the missing value from an array of consecutives value, O(n) time and O(1) space.

```rust
let array: [u32; 4] = [4, 2, 1, 5];
let missing_value = get_missing_value(&array); // 3
```

### product_all_items_except_current(array)

Returns the product of all the array items except the current item, O(n) time and O(n) space.
An array of identical space is built. The original array is browsed twice.

```rust
let array: [u32; 4] = [2, 5, 3, 4];
let result = product_all_items_except_current(&array); // [60, 24, 40, 30]
```

### get_max_product_of_three(array)

Returns the maximum product of three numbers from an array, taking O(n) time and O(1) space.

```rust
let array: [i32; 5] = [-4, 3, -2, -5, 6];
let max = get_max_product_of_three(&array); // -5 * -4 * 6 = 120
```

### get_merge_ranges(array)

Merge array of ranges, taking O(n log n) time worst case (O(n) best case) and O(n) space.
The array is sorted before processing, so it explains the time complexity.
A resulting array is created from the first one, it explains the space complexity.

```rust
let ranges: [(i32, i32); 5] = [
    (4, 7),
    (3, 5),
    (8, 10),
    (2, 3),
    (-6, -3),
];

let result = get_merge_ranges(&ranges); // [(-6, -3), (2, 7), (8, 10)]
```

### get_sum_numbers_combinations_quantity_for_amount(allowed_numbers, amount)

Find the quantity of the all the possible combinations to make a sum
using the "allowed numbers" that is equal to the expected "amount".
The time complexity is O(n * m) as two arrays of different content
are browsed (one with the possible numbers, one with the results).
The space complexity is O(n) as we have to create a new array
based on the given input.

```rust
let allowed_numbers: [u8; 3] = [2, 4, 5];
let amount: u32 = 15;

let sums_amount = get_sum_numbers_combinations_quantity_for_amount(
    &allowed_numbers,
    amount,
);

/* sums_amount = 4

   solutions:
   5, 5, 5
   2, 4, 4, 5
   2, 2, 2, 4, 5
   2, 2, 2, 2, 2, 5
*/
```

### get_sum_numbers_combinations_for_amount(allowed_numbers, amount)

Find all the possible combinations of numbers (using the "allowed numbers" only)
to make the sum for a given amount "amount".
The time complexity is O(n * m * o) and the space complexity is O(n * m).

```rust
let allowed_numbers: [u8; 3] = [2, 4, 5];
let amount: u32 = 15;

let sums_amount = get_sum_numbers_combinations_for_amount(
    &allowed_numbers,
    amount,
);

/*
    [
        [5, 5, 5],
        [2, 4, 4, 5],
        [2, 2, 2, 4, 5],
        [2, 2, 2, 2, 2, 5],
    ]
*/
```

### permutations_with_repetitions(total_amount, selection_amount)

Returns all the possible permutations amount (with repetitions)
according to a total amount of items and an items amount to include into the selection.

```
The amount of possible permutations (order matters) with repetitions for a total set of length n and a selection of length k is n^k.
```

```rust
let possible_permutations_amount = permutations_with_repetitions(10, 4); // 10000
```

### get_all_permutations_with_repetitions(array, selection_amount)
(handled by **get_all_permutations(array, selection, allow_repetition: true)**)

Returns all the possible permutations (with repetitions) of the given items from the array.

```rust
let array: [u8; 2] = [1, 2];
let permutations = get_all_permutations_with_repetitions(&array, 2);

/*
[1, 1],
[1, 2],
[2, 1],
[2, 2]
*/
```

This is also possible to define the size of all the returned permutations:

```rust
let array: [u8; 3] = [1, 2, 3];
let permutations = get_all_permutations_with_repetitions(&array, 2);

/*
[1, 1],
[1, 2],
[1, 3],
[2, 1],
[2, 2],
[2, 3],
[3, 1],
[3, 2],
[3, 3]
*/
```

### factorial(value)

Recursive factorial calculation.

```rust
let result = factorial(4); // 24
```

### permutations_without_repetitions(total_amount, selection_amount)

Returns all the possible permutations amount without allowing repetitions,
according to a total amount of items and an items amount to include into the selection.

```
The amount of possible permutations (order matters) without repetitions for a total set of length n and a selection of length k is n!/(n-k)!.
```

```rust
let possible_permutations_amount = permutations_without_repetition(5, 2); // 60
```

### get_all_permutations_without_repetition(array)
(handled by **get_all_permutations(array, selection, allow_repetition: false)**)

Returns all the possible permutations without allowing repetitions.

```rust
let items: [u8; 2] = [1, 2];
let permutations = get_all_permutations_without_repetition(items);

/*
[1, 2]
[2, 1]
*/
```

### at_linked_list(list, index)

Returns the value at the given index from the given `LinkedListNode` item in O(n) time and O(1) space.
