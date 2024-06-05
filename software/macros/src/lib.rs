// extern crate proc_macro2;
#![feature(extend_one)]

extern crate proc_macro;

use proc_macro2::Literal;
use proc_macro2::{Group, TokenStream, TokenTree};
use quote::{format_ident, quote};
use syn::parse_macro_input;

#[proc_macro]
pub fn create_enum(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as TokenStream);

    let mut tokenstream = TokenStream::new();
    let mut commands: Vec<TokenStream> = Vec::new();

    let mut iterator = input.into_iter();

    let n_slots = match iterator.next().unwrap() {
        TokenTree::Literal(val) => val.to_string().parse::<usize>().unwrap(),
        _ => panic!("Expected a literal"),
    };

    let _ = iterator.next().unwrap();

    iterator.for_each(|token| match token.clone() {
        TokenTree::Punct(punct) => {
            if punct.as_char() == ',' {
                commands.push(tokenstream.clone());
                tokenstream = TokenStream::new();
            } else {
                tokenstream.extend_one(token);
            }
        }
        _ => tokenstream.extend_one(token),
    });

    for _ in 0..(n_slots - commands.len()) {
        commands.push(tokenstream.clone());
    }

    let enum_variants = (0..n_slots)
        .map(|i| format_ident!("Slot{}", i))
        .collect::<Vec<_>>();

    let expanded = quote! {

            const N_SLOTS: usize = #n_slots;

            #[cfg_attr(feature = "std", derive(thiserror::Error))]
            #[cfg_attr(not(feature = "std"), derive(thiserror_no_std::Error))]
            #[derive(Debug)]
            pub enum AppMsgError {
                Launch,
                Exit,
                NoMatchingCommand
            }

            impl std::fmt::Display for AppMsgError {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    write!(f, "{:?}", self)
                }
            }

            #[derive(Serialize, Deserialize, Debug)]
            pub enum AppMsg {
                #(
                    #enum_variants
                ),*
            }


            #[cfg(feature = "std")]
            impl AppMsg {
                #[allow(unreachable_code)]
                pub fn launch(&self) -> Result<(), AppMsgError> {
                    use std::process::Command;

                    match self {
                        #(
                            Self::#enum_variants => {
                                #commands;
                                Ok(())
                            }
                        ),*
                    }
                }

            }
        };

    expanded.into()
}
