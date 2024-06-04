#[macro_export]
macro_rules! create_constants {
    ($($variant:ident)*) => {
        pub const APP_LIST: &[&str] = &[ $(stringify!($variant), )* ];
        pub const N_AUDIO_LEVELS: usize = APP_LIST.len();

        #[derive(Debug, Serialize, Deserialize)]
        pub enum AppLaunch {
            $(
                $variant,
            )*
        }
    };
}
