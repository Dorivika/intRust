#[macro_export]
macro_rules! CONST {
    ($($name:ident : $type:ty = $value:expr; )*) => {
        $(
            pub const $name: $type = $value;
        )*
    };
}
