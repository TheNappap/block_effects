# block_effects
Rust proc macro for chaining block effects such as unsafe, async, if, while, etc.

## Example

``` rust
fn multiple_blocks() {
    block!{ 
        for i in 0..3 for j in 0..3 {
            assert!(i < 3);
            assert!(j < 3);
            did_for = true;
        }
        while true || false {
            did_while = true;
            break;
        }
        if true match 0 {
            0 => did_if = true,
            _ => ()
        } else {
            assert!(false);
        }
    }
}
```
