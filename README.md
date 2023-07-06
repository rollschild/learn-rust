# Learn Rust (yet again)

## Design

- Rust provides **cache-friendly data structures** _by default_
  - usually it's **arrays** that hold data within Rust programs, _NOT_ nested tree structures created by pointers
  - **data-oriented programming**
- Methods are _always_ dispatched statically, unless explicitly requested to do so dynamically
- Utilities written in Rust are compiled as _static binaries_ by default
- **macros** return **code** rather than value
- Rust does _NOT_ have **constructors**
  the convention of `new` method implemented by types is _not_ part of the Rust lang

## Ops

- To compile a release build: `cargo run --release`
  - release build is faster at runtime but slower to compile
- append `-q` to `cargo` to run in quiet mode

## Basic Types

- default integer is `i32`
- default float is `f64`
- `usize` and `isize` - pointer sized integers
  - assume CPU's native width
  - 4 bytes on 32-bit arch
  - 8 bytes on 64-bit arch
- `f32` and `f64` - floating point
- `0b` - binary
  - `{:0b}`
- `0x` - hex
  - `{:0x}`
- `0o` - octal
  - `{:0o}`
- Conversions between types are _always_ **explicit**
  - use `as`
  - or, `use std::convert::TryInto`
    ```rust
    // try_into() returns an i32 wrapped in `Result`
    let num_ = num.try_into().unwrap();
    ```
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
- string formatting uses `{}` as formatter
- `as` - numeric type conversions
- constants - ALL_CAP_SNAKE_CASE - **must always** have explicit types
- `::std::f64::INFINITY`
- `::std::f32::NAN`
- `bool` takes _an entire byte_ in memory
- `char` represented by single quotes `''`
  - unicode char `'\u{62}'`
- `()` is a **unit** - returning nothing
  - the `main()` function by default returns `()`
- `main()` function can also return a `Result`

### Floating-point Numbers

- Implemented in base 2, but often used in calculations in base 10
- In Rust, `f32` and `f64` _only_ implement `std::cmp::PartialEq`
  - other numeric types _also_ implement `std::cmp::Eq`
- Guidelines:
  - **Avoid testing floating-point numbers for equality**
  - Be wary when results may be mathematically undefined
- `to_bits()`
- Safer to test within an acceptable margin, \***\*epsilon\*\***
- `f32::EPSILON` and `f64::EPSILON`
  - `assert!(abs_diff <= f32::EPSILON);`
- `NAN` - Not A Number
  - _NEVER_ equal to each other
- use `is_nan()` and `is_finite()`
  - they crash
-

## Functions

- use **tuple** to return multiple values from a function
- to return nothing: `return ();` - `()` is _empty tuple_
- to print `()`, use `{:?}`

## Flow Control

- `for item in container` - `container` becomes _unavailable_ after the `for` loop ends
- Three `for` loops:
  - `for item in collection` - Ownership
  - `for item in &collection` - Read-only
  - `for item in &mut collection` - Read-write
- `n..m` - _exclusive_ range
- `n..=m` - _inclusive_ range
- Try _not_ to use index:
  - performance: `collection[index]` incurs runtime costs for _bounds checking_
  - safety
- `loop` - to loop _forever_
  - until a `break` or termination
- often for long-running servers
- **loop label**
  - prefixed with `'`:
  ```rust
  'outer: for x in 0.. {
    for y in 0.. {
      for z in 0.. {
        if x + y + z > 1000 {
          break 'outer;
        }
      }
    }
  }
  ```
- Rust has no _truth_ or _falsey_ concepts
- `break` returns a value - allows infinite loops to return values
  - `break 123;`
- `match`
  - safer?
  - does _NOT_ fall through - returns _immediately_ when a match is found

## References

-
