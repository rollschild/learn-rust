# Learn Rust (yet again)

## Ops

- To compile a release build: `cargo run --release`
  - release build is faster at runtime but slower to compile
- append `-q` to `cargo` to run in quiet mode

## Basic Types

- default integer is `i32`
- default float is `f64`
- `usize` and `isize` - pointer sized integers
  - 4 bytes on 32-bit arch
  - 8 bytes on 64-bit arch
- `f32` and `f64` - floating point
- **tuple** - `(val, val, ...)`
  - of different types
  - 12 elements max?
  - fixed sequences of values \***\*on stack\*\***
  - to destructure: `let (a, b, c ...) = <some_tuple>;`
- **array** - `[val, val, ...]`
  - collection of _similar_ (same type) elements with fixed length known at _compile time_
  - of type `[T;N]`
  - `let arr = ['x'; 5];`
- **slices** - collection of _similar_ elements with length known at _runtime_
- `str` - string slice - text with length known at _runtime_
- `as` - numeric type conversions
- constants - ALL_CAP_SNAKE_CASE - **must always** have explicit types
- `::std::f64::INFINITY`
- `::std::f32::NAN`
- `bool` takes _an entire byte_ in memory
- `char` represented by single quotes `''`
  - unicode char `'\u{62}'`

## Functions

- use **tuple** to return multiple values from a function
- to return nothing: `return ();` - `()` is _empty tuple_
- to print `()`, use `{:?}`
-
