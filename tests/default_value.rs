use constructor_macro::ConstructorMacro;

#[test]
fn values() {
    #[derive(Debug, PartialEq, ConstructorMacro)]
    struct Values {
        field1: usize,
        field2: usize,
    }

    let thing = Values!();

    assert_eq!(thing.field1, 0);
    assert_eq!(thing.field2, 0);
}

#[test]
fn integers() {
    #[derive(Debug, PartialEq, ConstructorMacro)]
    struct Integers {
        #[default = 100]
        field1: usize,
        #[default = 5]
        field2: usize,
    }

    let thing = Integers!();

    assert_eq!(thing.field1, 100);
    assert_eq!(thing.field2, 5);
}

#[test]
fn unit_struct() {
    struct Value;

    #[derive(Debug, PartialEq, ConstructorMacro)]
    struct UnitStruct {
        #[default = Value]
        field1: Value,
    }

    let thing = UnitStruct!();

    assert_eq!(thing.field1, Value);
}

#[test]
fn tuple_struct() {
    struct Value(usize);

    #[derive(Debug, PartialEq, ConstructorMacro)]
    struct TupleStruct {
        #[default = Value(0)]
        field1: Value,
    }

    let thing = TupleStruct!();

    assert_eq!(thing.field1, Value(0));
}

#[test]
fn r#struct() {
    struct Value { value: usize };

    #[derive(Debug, PartialEq, ConstructorMacro)]
    struct Struct {
        #[default = Value { value: 0 }]
        field1: Value,
    }

    let thing = Struct!();

    assert_eq!(thing.field1, Value { value: 0 });
}

#[test]
fn c_enum() {
    enum Either {
        Left,
        Right,
    }
    #[derive(Debug, PartialEq, ConstructorMacro)]
    struct CEnum {
        #[default = Either::Left]
        field1: Either,
    }

    let thing = CEnum!();

    assert_eq!(thing.field1, Either::Left);
}

#[test]
fn enum_tuple_variant() {
    enum Either {
        Left(usize),
        Right(usize),
    }
    #[derive(Debug, PartialEq, ConstructorMacro)]
    struct EnumTupleVariant {
        #[default = Either::Left(0)]
        field1: Either,
    }

    let thing = EnumTupleVariant!();

    assert_eq!(thing.field1, Either::Left(0));
}

#[test]
fn enum_struct_variant() {
    enum Either {
        Left { value: usize },
        Right { value: usize },
    }
    #[derive(Debug, PartialEq, ConstructorMacro)]
    struct EnumStructVariant {
        #[default = Either::Left { value: 0 }]
        field1: Either,
    }

    let thing = EnumStructVariant!();

    assert_eq!(thing.field1, Either::Left { value: 0 });
}
