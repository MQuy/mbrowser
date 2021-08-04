pub mod url;

#[macro_export]
macro_rules! not_reached {
    () => (panic!("not be reached"));
    ($($arg:tt)+) => (panic!("not be reached: {}", $crate::format_args!($($arg)+)));
}

#[macro_export]
macro_rules! not_supported {
    () => (panic!("not yet supported"));
    ($($arg:tt)+) => (panic!("not yet supported: {}", $crate::format_args!($($arg)+)));
}
