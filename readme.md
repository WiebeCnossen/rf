# Random file

Generates a file with random bytes of given size.

## Build

Requires stable Rust toolchain edition 2021.

```
cargo build --release
```

## Usage

```
rf [size [filename [shift]]]
```

* `size` (default=35): the number of blocks in the file
* `filename` (default=rf.pdf): the file name
* `shift` (default=20): the number of shift lefts to get from 1 to the block size;
  default block size corresponds to 1MB
