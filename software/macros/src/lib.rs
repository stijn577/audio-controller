// extern crate proc_macro2;
#![feature(extend_one)]

extern crate proc_macro;

// use proc_macro2::{TokenStream, TokenTree};
// use quote::{format_ident, quote};
// use syn::parse_macro_input;

// A procedural macro that generates a custom enum `Button` and associated functionality.
//
// # Parameters
//
// * `input`: A TokenStream representing the input to the macro.
//
// # Returns
//
// A TokenStream representing the expanded code.
//
// # Example
//
// ```rust
// use buttons::buttons;
//
// const SHELL: &str = "powershell";
// const SHELL_EXEC: &str = "-Command";
//
// buttons!(
//     // define the number of slots on the hardware device => becomes const N_SLOTS: usize = ...;
//     5,
//     // define comands one by one, if more than number of slots are defined, macro panicks
//     // discord command
//     Command::new(SHELL)
//         .arg(SHELL_EXEC)
//         .args([
//             "C:/Users/Stijn_Admin/AppData/Local/Discord/Update.exe",
//             "--processStart",
//             "Discord.exe"
//         ])
//         .status()
//         .expect("Failed to launch discord!");
//     Ok(()),
//     // spotify command
//     Command::new(SHELL)
//         .arg(SHELL_EXEC)
//         .arg("spotify.exe")
//         .status()
//         .expect("Failed to launch spotify!");
//     Ok(()),
//     // firefox command
//     Command::new(SHELL)
//         .arg(SHELL_EXEC)
//         .arg("firefox.exe")
//         .status()
//         .expect("Failed to launch firefox!");
//     Ok(()),
//     // empty Command
//     Ok(()),
//     // if there are more slots than commands, there is a Error returned when the launch function is called
//     Err(ButtonError::NoMatchingCommand)
// );
// ```
//
// This will generate a custom enum `Button` with 5 variants: `Slot0`, `Slot1`, `Slot2`, `Slot3` and `Slot4`.
// The `launch` method will execute the corresponding command when called on a `Button` variant and return Ok(()).
// If there is less commands defined than the first argument for `buttons!` declares, a `ButtonError` is returned when `.launch()` is called.
// If there is more the macro will `panic!` at compile time.
//

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
