extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Block};

#[proc_macro]
pub fn asx(input: TokenStream) -> TokenStream { // todo wait until user input change to keep running
    let block = parse_macro_input!(input as Block);

    let expanded = quote! {
        {
            let mut _generated_block_wrapper = || {

                let mut should_close:bool = false;
                let mut input_change:bool = false;

                while window.is_open() && !should_close {

                    window.get_keys().iter().for_each(|key| {
                        input_change = true;
                    });

                    if input_change {
                        input_change = false;
                        // outisde vars also work
                        #block
                    }

                    window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
                }
            };
            _generated_block_wrapper();
        }
    };

    TokenStream::from(expanded)
}