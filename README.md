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

### `String` vs. `str`

- `str` - string slice - text with length known at _runtime_
  - cannot be expanded or shrank once created
  - UTF-8 guaranteed
  - `&str` - contains reference to `str` data and a length
  - attempting to assign a variable to `str` will _FAIL_
  - Rust compiler wants to create fixed-sized variables within a function's stack frame
  - `str` values can be of arbitrary length - can only be stored as local variables by reference
- `String` uses dynamic memory allocation
- Creating `&str` avoids a memory allocation
- `String` is an **owned** type
  - read-write
- `&str` is a **borrowed** type
  - read-only
- string literals are `&str`
  - full type: `&'static str`
- `char` is _ALWAYS_ **4** bytes
  - interanally encoded in UCS-4/UTF-32
  - _ALWAYS_ fixed-length - easier for the compiler
- `[u8]` - slice of raw bytes
- `Vec<u8>` - vector of raw bytes
- `String` <-> `Vec<u8>`
- `str` <-> `[u8]`
- string formatting uses `{}` as formatter
- `std::ffi::OSString` - platform-native string
  - not guaranteed to be UTF-8
  - does _NOT_ contain the zero byte `0x00`
- `std::path::Path` - filesys
- a backslash `\` escapes the newline character in the string literal
- `String` is guaranteed to be UTF-8
  - however reading a text file into `String` may cause errors if there are invalid bytes
  - Better - read in data as `[u8]` (slice of `u8` values) then decode those bytes

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

### Arrays vs. Vectors vs. Slices

- array - fixed size and _extremely_ light weight
  - although possible to replace items within an array
  - an array's type: `[T; n]`
- vector - growable but incurs small runtime penalty
- In practice, most interactions with arrays occurs via _another type_, **slice**, `[T]`
  - slice itself is interacted by reference `&[T]`
- slice - dynamically sized array-like objects
  - size _not_ known at compile time
  - _NOT_ resizable
  - _dynamic typing_
  - `[T; n]` vs. `[T]`
  - create a slice from an array - cheap!
  - can be used as a **view** on arrays (and other slices)
  - size of slice itself is _FIXED_ in memory: two `usize` components
    - pointer
    - length
  - `&[T]`
-

## Functions

- use **tuple** to return multiple values from a function
- to return nothing: `return ();` - `()` is _empty tuple_
- to print `()`, use `{:?}`

### Advanced Functions

#### Explicit Lifetime Annotations

- `'a`, `'b`, etc
  - lifetime variables
- `i: &'a i32` - binds lifetime variable `'a` to the lifetime of `i`
  - `i` is a reference to an `i32` with lifetime `a`
- All values bound to a given lifetime must live as long as the last access to any value bound to that lifetime

#### Generics

- **Traits**
  - `fn add<T: std::ops::Add<Output = T>>(i: T, j: T) -> T {}`
  - _all_ Rust operations are defined _within_ **traits**
- _ALL_ Rust operations are systactic sugar for trait's methods
- During compilation, `a + b` is converted to `a.add(b)`
-

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

## Read from CLI

- Use `clap`
- example usage:
  - `cargo run -- <arg>`
  - `./target/debug/<executable> --help`

## Read from Files

- `BufReader`
  - provides buffered I/O
  - can reduce system calls to OS if the hard disk is congested

## Compound Types

- `#![allow(unused_variables)]`
- `#[allow(dead_code)]`
- a return type of `!` indicates this function _NEVER_ returns
  - when function is guaranteed to crash
  - `panic!()`
  - when there is an infinite loop
- `()` - unit type
  - zero-length tuple
  - the function returns no value
  - expressions ended with `;` returns `()`
- `#[derive(Debug)]`
  - allows for printing for compound types
  - works with `{:?}`
  - the `Debug` trait is implemented for that type
- `String::from()` generates **owned** strings from string literals, which are slices
- accessing fields _by reference_ prevents the **use after move** issues
  - **borrowing**
- the `newtype` pattern
  - `struct SomeNewType(String);`
- `Vec<T>::reserve(&mut self, addtional: uszie)` - reserves `additional` _more_ elements
- `String::from_utf8_lossy(&buffer)` - converts `Vec<u8>` to `String`
  - non-UTF8 bytes are replaced with

### `impl`

- every `struct` can be instantiated through a literal syntax (with `::new()`)
- `struct` fields are _private_ by default, but can be accessed within the module that defines the `struct`

### Errors

- In C: `errno` of `-1`
  - commonly accessed/modified by sys calls
- In Rust: `static mut ERROR: i32 = 0;`
  - `static` - static lifetime valid for the life of the program
- Accessing and modifying `static mut` variables requires the use of an unsafe block

  ```rust
  unsafe {

  }
  ```

- By convention. global variables use `ALL_CAPS`
- `const` included for values that _never_ change
- `let` vs. `const`
  - data behind `let` _can_ change
  - `let` relates more to **aliasing** than **immutability**
  - **aliasing**: having _multiple_ references to the same location in memory at the same time
    - **borrows** (read-only references) to variables declared with `let` _can_ alias the same data
    - **mutable borrows** (read-write references) are _guaranteed to never_ alias data

#### `Result`

- `Result` has two states,
  - `Ok`
  - `Err`
- requires an extra `unwrap()`
  - will crash if unwrapping `Err`
- Calling `.unwrap()` on a `Result` is poor style!

### `enum`

- `.splitn(X)` - splits to at most `X` parts
- `enum` supports `impl`

### `trait`

- Allowing multiple types to implement a certain `trait` enables code reuse and allows the Rust compiler to perform **zero-cost abstraction**
  - by not adding extra data around values within `strcut`s
- `println!`, `print!`, `write!`, `writeln!`, `format!`
  - rely on `Display` and `Debug` traits
  - `{}`

### Privacy

- a `pub struct`'s fields remain _private_
- an `pub enum`'s variants are _public_
- `pub struct`'s methods should also be marked `pub`

### Documentation

- `///` or `//!`
- Use `cargo doc --open` to generate doc for the project _and_ open it in web browser
- Use `cargo doc --no-deps` to not include dependencies

## Lifetimes, Ownership, and Borrowing

- **move** - movement of **ownership**
- _Every_ value in Rust is **owned**
- Types implementing `Copy` trait are duplicated at times that would otherwise be illegal
- primitive types possess **copy semantics**
- all other types have **move semantics**

### Owner

- Owner cleans up when its values' lifetimes end
- `Drop`
  - to implement custom destructor
  - `drop(&mut self)`
- Values _cannot_ outlive their owner

### Best Practices

- Use references where full ownership is not required
- Duplicate the value
- Refactor the code to reduce number of long-lived objects
- Wrap your data in a type designed to assist with movement issues

#### Duplicate the value

- Two modes of duplication: **clone** vs. **copy**, by different traits
  - `std::clone::Clone`
  - `std::marker::Copy`
  - copy is _implicit_ - whenever ownership would otherwise be moved to an inner scope
    - always fast and cheap
    - always identical - copies are bit-for-bit duplicates
  - clone is _explicit_ - `.clone()`
    - may be _slow_ and _expensive_
- Why not always `Copy`?
  - not for non-negligible performance impact
  - not working perfectly on references
  - some types overload the `Clone` trait
- If implementing `Copy` manually, you would _also_ need to implement `Clone`

#### Wrap data within specialty types

- Will incur runtime costs
- `std::rc::Rc`
  - `Rc<T>`
    - **a reference-counted value of type `T`**
    - provides **shared ownership** of `T`
    - prevents `T` from being removed from memory until every owner is removed
    - implements `Clone`
    - does _NOT_ allow mutation!
    - to allow mutation, use `Rc<RefCell<T>>`
      - `let mut mutable_base = <base>.borrow_mut()` - allowing mutation
    - could be an alternative if implementing `Clone` is _prohibitively_ expensive
    - _NOT_ thread safe!!!
    - In multithreaded code, replace with `Arc<T>` and `Arc<Mutex<T>>`

## Data in Depth

### Bit Patterns and Types

- Text files are just binary files that happen to follow a _consistent_ mapping between bit strings and characters
  - **encoding**

```rust
let a: f32 = 42.42;
let frankentype: u32 = unsafe {
  std::mem::transmute(a)
};
```

- `{:032b}` - directive for the `println!()` macro
  - left-pad with 32 zeros
  - right-hand `b` invokes the `std::fmt::Binary` trait
- `{}` invokes `std::fmt::Display` trait
- `{:?}` invokes `std::fmt::Debug` trait
- `std::mem::transmute()` - naively interpret an `f32` as `u32` without affecting any of the underlying bits

### Life of an Integer

```rust
#[allow(arithmetic_overflow)]

fn main() {
  let (a, b) = (200, 200);
  let c: u8 = a + b;
  println!("200 + 200 = {}", c);
}
```

- code below compiles if specifying the `-O` flag: `rustc -O`
  - but gives the _wrong_ answer, 144

#### Endianness

- how bytes layout in CPU
- **Big Endian** - most significant on the left
- **Little Endian** - most significant on the right
- Integers are almost certainly stored in **little endian**

### Decimal Numbers

- Each floating-point number is laid out in memory as **scientific notation**
- Consists of:
  - sign bit
  - exponent
  - mantissa
  - radix, which is `2`
- $(-1)^{signbit} \times mantissa \times Radix^{exponent - bias}$
- allows for both $0$ and $-0$
- To isolate the sign bit, `>> 31`
- To isolate the exponent:
  - `>> 23`
  - AND mask `& 0xff` to exclude the sign bit
  - Decode - subtract `127`
- To isolate mantissa,
  - `& 0x7fffff`
  - calculate weight

### Fixed-Point Number Formats

- Useful for representing fractions
- An option for performing calc on CPUs w/o a floating point unit (FPU)
  - microcontrollers
- Loses accuracy, saves _significant_ space
- \***\*The Q Format\*\*** - fixed-point number format using a single byte
- Q7
  - 7 bits available for representing number and 1 bit for sign
  - `i8`
- **tuple struct** - struct created from unnamed fields
  - `struct Q(i8);`
- `PartialEq` - can be compared using `==`
- `Eq` - any possible values of the type can be compared against any other possible values of the same type
- `impl From<T> for U`
  - `std::convert::From`
  - `std::convert::TryFrom`

### Generating Random Probabilities from Random Bytes

- Division is a _slow_ operation

### Implementing a CPU

- **CHIP-8**

#### CPU RIA/1 - The Adder

- An **operation** (**op**)
  - procedures supported natively by the system
  - **implemented in hardware**
  - **intrinsic operation**
- **Register**
  - containers of data that can be directly accessed by CPU
  - for CHIP-8, each register is `u8`
- **opcode** - a number that maps to an operation
  - on CHIP-8, opcodes include both:
    - operation
    - operands' registers
- Steps to perform _addition_:
  1. Initialize CPU
  2. Load `u8` values into **registers**
  3. Load the addition opcode into `current_operation`
  4. Perform the addition
- Emulator:
  1. Reads the opcode
  2. Decodes instruction
  3. Matches decoded instruction to known opcodes
  4. Dispatches execution of the operation to a specific function
- How to interpret CHIP-8 codes
  - `u16` values made up of 4 **nibbles**
  - **nibble** - half a byte
  - Each opcode (`u16`) is made up of two bytes:
    - **high byte**
    - **low byte**
  - Each byte is made up of two nibbles
    - **high nibble**
    - **low nibble**

```
                 high byte              low byte
              \/          \/         \/          \/
0  x          7           3          E           E
              ^           ^
        high nibble   low nibble
```

#### CPU RIA/2 - The Multiplier

- can execute several instructions in sequence
- 4K memory
- opcode of `0x0000` indicates stopping the CPU
- Limitations:
  - `position_in_memory` is more formally referred to as **program counter**
  - CHIP-8 specifications reserves the first 512 bytes (`0x100`) for the system
- Use the last register as a **carry flag**
  - an operation has overflowed the `u8` register size

#### CPU RIA/3 - The Caller

- Adds the ability to call functions
- _NO_ programming language support
  - _any_ programs still need to be written in binary
- Support for a **stack** - a specialized memory to store the following addresses:
  - the **CALL** opcode - `0x2nnn`
    - `nnn` is the memory address
    - sets `position_in_memory` to `nnn`, the address of the function
  - the **RETURN** opcode - `0x00EE`
    - sets `position_in_memory` to the memory address of the previous CALL opcode
- Function: a sequence of bytes that can be executed by a CPU
- `copy_from_slice(&some_slice)`
- Calling a function:
  1. Store the current memory location on the stack
  2. Increment the stack pointer
  3. Set the current memory location to the intended memory address
- Returning from a function:
  1. Decrement the stack pointer
  2. Retrieve the calling memory address from the stack
  3. Set the current memory location to the intended memory address
- `cargo build --release` _disables_ runtime checks
  - integer overflows/underflows could happen!
