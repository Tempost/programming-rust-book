
# NOTES:
## Panic - Something that should never happen
    - Out of bounds array acces
    - Division by Zero
    - .expect() on Result that is actually an Err
    - Assertion failure (assert!)

To view a stack trace pass RUST_BACKTRACE=1 when calling cargo run or built program
Panic will also safetly drop all owned/heap memory, use of panic! is NEVER unsafe rust

std::panic::catch_unwind() can be used to catch the panic and recover from it
resuming normal program operation while still handling the panic saftely. But not every panic
is "catchable" in this way

## Result - Function fails (or succeeds) and returns data or error wrapped in the Result<>
    - Indicates possible failure
    - Requires .unwrap() or error handling in user land

## Common type alias pattern:

```rust
pub type Result<T> = result::Result<T, Error>;
```
Used for cases were in an API you will use a result with the same error type many times
can be used to save time with typing or reduce errors from copy/paste, yank/paste

Most Rust errors impl the std::error::Error trait, allowing the user to print errors messages
using println!() To get Debug level info use {:?}

## Useful function for getting ALL error information when printing errors
```rust
use std::error::Error;
use std::io::{stderr, Write};
/// Dump an error message to `stderr`.
///
/// If another error happens while building the error message or
/// writing to `stderr`, it is ignored.
fn print_error(mut err: &dyn Error) {
    let _ = writeln!(stderr(), "error: {}", err);
    while let Some(source) = err.source() {
        let _ = writeln!(stderr(), "caused by: {}", source);
        err = source;
    }
}
```


