extern crate proc_macro;
use laststraw_core::Position;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Block, Ident, ItemStruct};

// TODO update change on file save

#[proc_macro]
pub fn asx(input: TokenStream) -> TokenStream {
    // todo wait until user input change to keep running
    let block = parse_macro_input!(input as Block);

    let expanded = quote! {
        {
            let mut _generated_block_wrapper = || {

                while app.window.is_open() && !app.should_close {

                    #block
                    // TODO add optional pause if no input event

                    app.window.update_with_buffer(&app.buffer, app.width, app.height).unwrap();
                }
            };
            _generated_block_wrapper();
        }
    };

    TokenStream::from(expanded)
}

use rusttype::{point, Font, Scale};

#[proc_macro]
pub fn button(input: TokenStream) -> TokenStream {
    let block = parse_macro_input!(input as Block);

    let expanded = quote! {
        {
            let mut _generated_block_wrapper = || {
                if set_button_position1 = true {
                    #block
                }
                //#block
            };
            _generated_block_wrapper();
        }
    };

    TokenStream::from(expanded)
}
