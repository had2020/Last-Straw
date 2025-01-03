extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Block};

#[proc_macro]
pub fn asx(input: TokenStream) -> TokenStream {
    let block = parse_macro_input!(input as Block);

    let expanded = quote! {
        {
            let mut _generated_block_wrapper = || {
                
                let mut err_object = error_init(); 

                while !window.should_close() { 
                    window.swap_buffers();
                    
                    // input handling
                    err_object.poll_events();

                    //std::fs::remove_file("temp.tmp").unwrap(); // remove temp file

                    // input key
                    for (_, event) in glfw::flush_messages(&events) {

                        match event {
                            glfw::WindowEvent::Key(key, _, Action::Press, _) => {
                                laststraw::handle_key_event_asx(key);
                            },
                            _ => {},
                        }
                    }
                    /* _ */

                    #block
                }
                if window.should_close() {
                    std::fs::remove_file("temp.tmp").unwrap();
                }
            };
            _generated_block_wrapper();
        }
    };

    TokenStream::from(expanded)
}