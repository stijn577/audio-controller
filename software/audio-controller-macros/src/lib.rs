// extern crate proc_macro2;
// #![feature(extend_one)]

// extern crate proc_macro;

use quote::{format_ident, quote};
use syn::LitStr;

#[proc_macro]
pub fn create_bitmaps(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let path = syn::parse::<LitStr>(input).unwrap();

    let files: std::fs::ReadDir =
        std::fs::read_dir(path.value()).expect("Failed to read directory");

    let mut bmp_paths = files
        .map(|entry| {
            entry
                .unwrap()
                .path()
                .into_os_string()
                .into_string()
                .unwrap()
        })
        .collect::<Vec<_>>();
    bmp_paths.sort();

    println!("Defined constants for bitmaps:");
    let bmp_names = bmp_paths
        .iter()
        .map(|s| {
            let temp = s
                .strip_suffix(".bmp")
                .unwrap()
                .split('\\')
                .last()
                .unwrap()
                .to_uppercase();
            println!("{}", temp);
            format_ident!("{}", temp)
        })
        .collect::<Vec<_>>();

    let expanded = quote! {
        #(
            pub const #bmp_names: ImageRaw<'static, Rgb888> = ImageRaw::new(include_bytes!(concat!("../", #bmp_paths)), 64);
        ),*
    };

    expanded.into()
}

// use proc_macro::TokenStream;
// use quote::quote;
// use syn::{parse_macro_input, Data, DeriveInput, Fields, Ident, Index};

// #[proc_macro]
// pub fn messaging(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
//     let input = parse_macro_input!(input as TokenStream);
//
//     // Extract number of hardware and software slots from literals
//     let slots = match input.into_iter().next().unwrap() {
//         TokenTree::Literal(val) => val.to_string().parse::<usize>().unwrap(),
//         _ => panic!("Expected a literal"),
//     };
//
//     let slots = (0..slots)
//         .map(|i| format_ident!("Slot{}", i))
//         .collect::<Vec<_>>();
//
//     let expanded = quote! {
//         #[cfg_attr(feature = "std", derive(thiserror::Error))]
//         #[cfg_attr(not(feature = "std"), derive(thiserror_no_std::Error))]
//         #[derive(Serialize, Deserialize, Debug)]
//         pub enum ButtonError {
//             Default,
//             NoMatchingCommand,
//         }
//
//         #[cfg(feature="std")]
//         impl std::fmt::Display for ButtonError {
//             fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//                 write!(f, "{:?}", self)
//             }
//         }
//
//         #[derive(Serialize, Deserialize, Debug)]
//         pub enum Message {
//             #(
//                 #slots
//             ),*
//             ,AudioLevels(AudioLevels),
//         }
//
//
//     };
//
//     expanded.into()
// }
