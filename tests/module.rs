use constructor_macro::ConstructorMacro;

#[test]
fn module() {
    mod module {
        use super::*;
        #[derive(ConstructorMacro)]
        pub struct Inner;
    }

    use module::Inner;
    let _thing = Inner!();
}
