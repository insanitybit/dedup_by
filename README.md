# dedup_by
A helper function for removing consecutive duplicates from a vector of complex types

# Example

```rust
use dedup_by::dedup_by;

let mut vec = vec![(1, 2), (2, 2), (3, 1)];
dedup_by(&mut vec, |a, b| { a.1 == b.1 });

assert_eq!(vec, [(1, 2), (3, 1)]);
```
