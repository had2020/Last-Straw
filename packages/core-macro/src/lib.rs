extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Block};

//use glfw::{fail_on_errors, Action, Context, Glfw, GlfwReceiver, MouseButton, PWindow, WindowEvent, Key}; // new argument for type of input maybe a way to have multiple inputs

#[proc_macro]
pub fn asx(input: TokenStream) -> TokenStream {
    let block = parse_macro_input!(input as Block);

    let expanded = quote! {
        fn _generated_block_wrapper() {

            let (mut window, events) = defined_window(true, 500, 500, "test")
                .expect("Failed to Define window!");
            
            let mut err_object = error_init(); 

            while !window.should_close() { 
                window.swap_buffers();
                
                //input_handling(window, events, err_object); // failing
                // input handling
                err_object.poll_events();

                for (_, event) in glfw::flush_messages(&events) {
                    //println!("{:?}", event); // debug
                    match event { // asx! macro will be used to replace this
                        glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => { 
                            window.set_should_close(true)
                        },
                        _ => {},
                    }
                }
                // end of input handling
                #block
            }
        }
        _generated_block_wrapper();
    };

    TokenStream::from(expanded)
}