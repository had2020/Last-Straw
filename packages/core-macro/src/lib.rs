extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Block};

#[proc_macro]
pub fn asx(input: TokenStream) -> TokenStream {
    let block = parse_macro_input!(input as Block);

    let expanded = quote! {
        {
            let _generated_block_wrapper = || {
                let (mut window, events) = defined_window(true, 500, 500, "test")
                    .expect("Failed to Define window!");
                
                let mut err_object = error_init(); 

                while !window.should_close() { 
                    window.swap_buffers();
                    
                    // input handling
                    err_object.poll_events();

                    for (_, event) in glfw::flush_messages(&events) {
                        match event {
                            glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => { 
                                window.set_should_close(true)
                            },
                            _ => {},
                        }
                    }
                    /* _ */
                    #block
                }
            };
            _generated_block_wrapper();
        }
    };

    TokenStream::from(expanded)
}