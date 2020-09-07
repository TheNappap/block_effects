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
            break;
        }
        if cond match 0 unsafe  {
            0 => assert!(true),
            _ => ()
        } else {
            //...
        }
    }
}
```
