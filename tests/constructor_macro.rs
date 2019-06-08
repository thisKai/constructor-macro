#![feature(proc_macro_hygiene)]

use constructor_macro::ConstructorMacro;

#[test]
fn construct_a_struct() {
    #[derive(Debug, Default, PartialEq, ConstructorMacro)]
    struct Thing {
        field1: usize,
        field2: usize,
    }

    let expected = Thing {
        field1: 1,
        field2: 0,
    };

    let thing = Thing! {
        field1: 1
    };
    assert_eq!(thing, expected);

    let thing_with_comma = Thing! {
        field1: 1,
    };
    assert_eq!(thing_with_comma, expected);
}
