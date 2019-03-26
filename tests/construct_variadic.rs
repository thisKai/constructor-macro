#![feature(proc_macro_hygiene)]

use constructor_macro::construct_variadic;

#[test]
fn construct_a_struct() {
    #[derive(Debug, Default, PartialEq)]
    struct Thing {
        field1: usize,
        field2: usize,
    }
    let thing = construct_variadic! {
        Thing;
        field1: 1,
    };
    
    assert_eq!(thing, Thing { field1: 1, field2: 0 });
}
