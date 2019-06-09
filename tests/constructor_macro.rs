use constructor_macro::ConstructorMacro;

#[test]
fn construct_a_struct() {
    #[derive(Debug, PartialEq, ConstructorMacro)]
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

#[test]
fn default_value() {
    #[derive(Debug, PartialEq, ConstructorMacro)]
    struct Thing1 {
        field1: usize,
        field2: usize,
    }

    let thing = Thing1!();

    assert_eq!(thing.field1, 0);
    assert_eq!(thing.field2, 0);
}

#[test]
fn default_value_attribute() {
    #[derive(Debug, PartialEq, ConstructorMacro)]
    struct Thing2 {
        #[default = 100]
        field1: usize,
        #[default = 5]
        field2: usize,
    }

    let thing = Thing2!();

    assert_eq!(thing.field1, 100);
    assert_eq!(thing.field2, 5);
}

#[test]
fn unit_struct() {
    #[derive(ConstructorMacro)]
    struct EmptyThing;

    let _thing = EmptyThing!();
}
