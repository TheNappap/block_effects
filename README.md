# block_effects
Rust proc macro for chaining block effects such as unsafe, async, if, while, etc.

## Example

``` rust
fn multiple_blocks() {
    block!{ 
        for i in 0..3 for j in 0..3 {
            //...
        }
        while *cond* {
            //...
            break;
        }
        if *cond* match 0 {
            0 => assert!(true),
            _ => ()
        } else {
            //...
        }
    }
}
```
