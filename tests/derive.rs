#![feature(proc_macro_hygiene)]

use constructor_macro::VariadicConstructor;

#[test]
fn construct_a_struct() {
    #[derive(Default, VariadicConstructor)]
    struct Thing {
        field1: usize,
        field2: usize,
    }

    let thing = Thing! {
        field1: 1
    };
    let thing_with_comma = Thing! {
        field1: 1,
    };
}
