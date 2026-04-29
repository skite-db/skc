#[macro_export]
macro_rules! context {
    ($($arg:tt)*) => {
        || format!($($arg)*)
    };
}