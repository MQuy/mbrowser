#[macro_export]
macro_rules! not_supported {
    () => (panic!("not yet supported"));
    ($($arg:tt)+) => (panic!("not yet supported: {}", $crate::format_args!($($arg)+)));
}
