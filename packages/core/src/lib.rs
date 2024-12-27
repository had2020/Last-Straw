//!  comments item containing the comment

// cargo test
// doc gen cargo doc --open

/*  TODO add tests like this!
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
 */

use glfw::{fail_on_errors, Action, Context, Glfw, GlfwReceiver, MouseButton, PWindow, WindowEvent, Key};

pub fn defined_window( width: u32, height: u32, name: &str) -> Option<(glfw::PWindow, glfw::GlfwReceiver<(f64, glfw::WindowEvent)>)> {
    let mut glfw = glfw::init(fail_on_errors).expect("Failed to initialize GLFW");

    let (mut window, events) = glfw.create_window(width, height, name, glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window, with defined_window method.");

    window.make_current();
    window.set_key_polling(true);

    Some((window, events)) 
}

pub fn error_init() -> Glfw {
    use glfw::fail_on_errors;
    glfw::init(fail_on_errors!()).unwrap()
}

pub fn testwindow(pwindow: PWindow, events: GlfwReceiver<(f64, WindowEvent)>, aflow: Glfw) {
    let mut window = pwindow;
    let mut flow = aflow;
    while !window.should_close() {
        // buffering frames
        window.swap_buffers();

        // event handling input 
        flow.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            println!("{:?}", event);
            match event {
                glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => { // input esc
                    window.set_should_close(true)
                },
                _ => {},
            }
        }
    }
}

pub fn app_running(pwindow: PWindow) -> bool{
    if !pwindow.should_close() {
        true
    } else {
        false
    }
}

//extern crate proc_macro;
//use proc_macro::TokenStream;

/* 
pub fn apploop(input: TokenStream) -> TokenStream {
    //input as a block of code
    let block: Block = parse_macro_input!(input as Block);

    //while loop wrapping code
    let expanded = quote! {
        while true {
            #block
        }
    };

    TokenStream::from(expanded)
}
*/