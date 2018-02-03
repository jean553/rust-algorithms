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

### min(array)

Returns the minimum value from an array, O(n) time and O(1) space.

### max(array)

Returns the maximum value from an array, O(n) time and O(1) space.

### get_max_range(array)

Returns the highest distance possible between two items of the array, O(n) time and O(1) space.
