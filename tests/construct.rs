use constructor_macro::ConstructorMacro;

#[test]
fn unit_struct() {
    #[derive(ConstructorMacro)]
    struct UnitStruct;

    let _thing = UnitStruct!();
}

#[test]
fn struct_with_named_fields() {
    #[derive(Debug, PartialEq, ConstructorMacro)]
    struct StructWithNamedFields {
        field1: usize,
        field2: usize,
    }

    let expected = StructWithNamedFields {
        field1: 1,
        field2: 0,
    };

    let thing = StructWithNamedFields! {
        field1: 1
    };
    assert_eq!(thing, expected);

    let thing_with_comma = StructWithNamedFields! {
        field1: 1,
    };
    assert_eq!(thing_with_comma, expected);
}
