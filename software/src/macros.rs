macro_rules! cond_log {
    ($($arg:tt)*) => {
        #[cfg(feature = "logging")]
        $($arg)*
    };
}
