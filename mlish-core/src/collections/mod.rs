#[macro_export]
macro_rules! inline_mod {
    ($($name:ident), + $(,)?) => {
        $(
            mod $name;
            pub use $name::*;
        )+
    };
}
