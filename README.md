# block_effects
Rust proc macro for chaining block effects such as unsafe, async, if, match, while, for and loop

## Example

``` rust
fn multiple_blocks() {
    block!{ 
        for i in 0..3 for j in 0..3 {
            //...
        }
        async while cond {
            //...
        }
        if cond unsafe match 0 {
            0 => //...,
            _ => //...
        } else {
            //...
        }
    }
}
```
