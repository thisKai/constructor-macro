# constructor-macro

## Cargo.toml
```toml
[dependencies]
constructor-macro = "0.2"
```

## Example

### Rust 2018
```rust
use constructor_macro::ConstructorMacro;

#[derive(ConstructorMacro)]
pub struct Thing {
    #[default(1)]
    pub field1: i32,
    pub field2: i32,
}

fn main() {
    let thing1 = Thing!();
    assert_eq!(thing1.field1, 1);
    assert_eq!(thing1.field2, 0);

    let thing2 = Thing! { field1: 2 };
    assert_eq!(thing2.field1, 2);
    assert_eq!(thing2.field2, 0);

    let thing3 = Thing! {
        field1: 0,
        field2: 100,
    };
    assert_eq!(thing3.field1, 0);
    assert_eq!(thing3.field2, 100);
}
```

### Rust 2015
```rust
#[macro_use]
extern crate constructor_macro;

...
```
