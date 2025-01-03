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

                    // input key
                    for (_, event) in glfw::flush_messages(&events) {

                        if let Err(e) = std::fs::write("inputs.txt", format!("{:?}\n", event)) {
                            eprintln!("Failed to write to file: {}", e); // possible replace with shmem
                            break;
                        }

                        match event {
                            glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => { 
                                
                                /* //example process to write to file
                                if let Err(e) = std::fs::write("foo.txt", b"bar!") {
                                    eprintln!("Failed to write to file: {}", e); 
                                    break;
                                }
                                */

                                std::fs::remove_file("inputs.txt").unwrap(); // clear input cache file
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