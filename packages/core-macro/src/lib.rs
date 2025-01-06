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

                while app.window.is_open() && !app.should_close {
                    /* 
                    app.window.get_keys().iter().for_each(|key| {
                        app.input_change = true;
                    });

                    
                    if app.input_change {
                        app.input_change = false;
                        // outisde vars also work
                        #block
                    }
                    */
                    #block

                    app.window.update_with_buffer(&app.buffer, WIDTH, HEIGHT).unwrap();
                }
            };
            _generated_block_wrapper();
        }
    };

    TokenStream::from(expanded)
}