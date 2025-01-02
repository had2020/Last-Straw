extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Block};
use std::fs::File;
use shmem::SharedMem;

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

                    // input key
                    for (_, event) in glfw::flush_messages(&events) {
                        match event {
                            glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => { 
                                
                                if let Err(e) = std::fs::write("foo.txt", b"Hello, world!") {
                                    eprintln!("Failed to write to file: {}", e); // replace with Shared Memory Using shmem crate
                                    break;
                                }

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