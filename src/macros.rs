#[macro_export]
macro_rules! CONST {
    ($($name:ident : $type:ty = $value:expr; )*) => {
        $(
            const $name: $type = $value;
        )*
    };
}
