# Learn Rust (yet again)

## Ops

- To compile a release build: `cargo run --release`
  - release build is faster at runtime but slower to compile
- append `-q` to `cargo` to run in quiet mode

## Basic Types

- `usize` and `isize` - pointer sized integers
- `f32` and `f64` - floating point
- **tuple** - `(val, val, ...)`
  - fixed sequences of values \***\*on stack\*\***
- **array** - `[val, val, ...]`
  - collection of _similar_ (same type) elements with fixed length known at _compile time_
  - of type `[T;N]`
- **slices** - collection of _similar_ elements with length known at _runtime_
- `str` - string slice - text with length known at _runtime_
- `as` - numeric type conversions
- constants - ALL_CAP_SNAKE_CASE - **must always** have explicit types
-
