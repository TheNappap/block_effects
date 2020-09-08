# block_effects
##### Rust proc macro for chaining block effects.
With `block_effects`, it is possible to chain different block effects with the use of the `block!` macro.

This idea was proposed by [mcy](https://internals.rust-lang.org/t/eliminating-seemingly-unnecessary-braces/12971).

The `block!` macro is used to mark blocks with chained block effects.

The following block effects can be chained:
`unsafe`, `async`, `if`, `match`, `loop`, `while` and `for`.
Additionally `if let` and `while let` are also possible.


## Goal
The main purpose of this crate is to minimize indentation for nested blocks.
Unfortunately proc macros also add indentation. 
So to fully benefit from this one should not indent the `block!` macro or even put the macro braces on the same line as the block.

## Example
``` rust
use block_effects::block;

block!{
if let Some(_) = Some(0) if let Some(_) = Some(0) {
    assert!(true); 
}
}

let _future = block!{ unsafe async for i in 0..3 match i { 
    0..=2 => assert!(true),
    _ => assert!(false)
} };
```
