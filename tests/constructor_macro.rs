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
fn default_value_unit_struct() {
    struct Value;

    #[derive(Debug, PartialEq, ConstructorMacro)]
    struct DefaultValueUnitStruct {
        #[default = Value]
        field1: Value,
    }

    let thing = DefaultValueUnitStruct!();

    assert_eq!(thing.field1, Value);
}

#[test]
fn default_value_tuple_struct() {
    struct Value(usize);

    #[derive(Debug, PartialEq, ConstructorMacro)]
    struct DefaultValueTupleStruct {
        #[default = Value(0)]
        field1: Value,
    }

    let thing = DefaultValueTupleStruct!();

    assert_eq!(thing.field1, Value(0));
}

#[test]
fn default_value_struct() {
    struct Value{ value: usize };

    #[derive(Debug, PartialEq, ConstructorMacro)]
    struct DefaultValueStruct {
        #[default = Value { value: 0 }]
        field1: Value,
    }

    let thing = DefaultValueStruct!();

    assert_eq!(thing.field1, Value { value: 0 });
}

#[test]
fn default_value_c_enum() {
    enum Either {
        Left,
        Right,
    }
    #[derive(Debug, PartialEq, ConstructorMacro)]
    struct DefaultValueCEnum {
        #[default = Either::Left]
        field1: Either,
    }

    let thing = DefaultValueCEnum!();

    assert_eq!(thing.field1, Either::Left);
}

#[test]
fn default_value_enum_field() {
    enum Either {
        Left(usize),
        Right(usize),
    }
    #[derive(Debug, PartialEq, ConstructorMacro)]
    struct DefaultValueEnumField {
        #[default = Either::Left(0)]
        field1: Either,
    }

    let thing = DefaultValueEnumField!();

    assert_eq!(thing.field1, Either::Left(0));
}

#[test]
fn default_value_enum_named_field() {
    enum Either {
        Left { value: usize },
        Right { value: usize },
    }
    #[derive(Debug, PartialEq, ConstructorMacro)]
    struct DefaultValueEnumNamedField {
        #[default = Either::Left { value: 0 }]
        field1: Either,
    }

    let thing = DefaultValueEnumNamedField!();

    assert_eq!(thing.field1, Either::Left { value: 0 });
}

#[test]
fn unit_struct() {
    #[derive(ConstructorMacro)]
    struct EmptyThing;

    let _thing = EmptyThing!();
}

#[test]
fn module() {
    mod module {
        use super::*;
        #[derive(ConstructorMacro)]
        pub struct InnerThing;
    }

    use module::InnerThing;
    let _thing = InnerThing!();
}
