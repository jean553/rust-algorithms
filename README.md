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

Returns the missing value from an array of consecutives values.

```rust
let array: [u32; 4] = [4, 2, 1, 5];
let missing_value = get_missing_value(&array); // 3
```
