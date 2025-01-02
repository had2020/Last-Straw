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

// TODO at some point write some amazing documentation for everything

pub fn defined_window( resizeable: bool, width: u32, height: u32, name: &str) -> Option<(glfw::PWindow, glfw::GlfwReceiver<(f64, glfw::WindowEvent)>)> {
    let mut glfw = glfw::init(fail_on_errors).expect("Failed to initialize GLFW");

    let (mut window, events) = glfw.create_window(width, height, name, glfw::WindowMode::Windowed).unwrap();
        //.expect("Failed to create GLFW window, with defined_window method.");

    window.make_current();
    window.set_key_polling(true);

    // inti settings for window
    glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));
    glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));
    if resizeable {
        glfw.window_hint(glfw::WindowHint::Resizable(true));
    }

    // return
    Some((window, events)) // TODO use put in () for asx marco
}

pub fn error_init() -> Glfw {
    use glfw::fail_on_errors;
    glfw::init(fail_on_errors!()).unwrap()
}

pub fn testwindow(pwindow: PWindow, events: GlfwReceiver<(f64, WindowEvent)>, aflow: Glfw) { // TODO replaced with core macro, does whole thing, for testing, will halt asx! macro
    let mut window = pwindow;
    let mut flow = aflow; // flow for making defined run something else in asx! macro
    // /\ redefinitions 
    while !window.should_close() { // move inside core macro some how
        // buffering frames
        window.swap_buffers();

        // event handling input 
        flow.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            println!("{:?}", event);
            match event { // asx! macro will be used to replace this
                glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => { // input esc // should be seperate function later
                    window.set_should_close(true)
                },
                _ => {},
            }
        }
    }
}

pub fn app_running(pwindow: PWindow) -> bool{ //TODO possiablely replaced by core macro
    if !pwindow.should_close() {
        true
    } else {
        false
    }
}

// for asx! macro remove it is never used due to borrow checker issues
pub fn input_handling(pwindow: PWindow, events: GlfwReceiver<(f64, WindowEvent)>, flow: Glfw) {
    let mut window = pwindow;
    let mut err_object = flow;
    err_object.poll_events();
    for (_, event) in glfw::flush_messages(&events) {
        println!("{:?}", event);
        match event { // asx! macro will be used to replace this
            glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => { // input esc // should be seperate function later
                window.set_should_close(true)
            },
            _ => {},
        }
    }
}

/* not needed
pub fn setvalue(value: &mut bool) {
    *value = true;
}
*/

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

// post marco working TODO clear out old code

/* 
pub fn input_key(key_type: Key, action_type: Action, window: PWindow, events: GlfwReceiver<(f64, WindowEvent)>) {
    for (_, event) in glfw::flush_messages(&events) { // This works TODO put in function for easy use with a match for input sting to input type
        match event {
            glfw::WindowEvent::Key(Key::Up, _, action_type, _) => { 
                window.set_should_close(true);
                println!("Up key pressed!");
                
            },
            _ => {},
        }
    }
}
*/

// TODO first ginal implementation

pub fn input_key<F>(key_type: Key, action_type: Action, window: &PWindow, events: &GlfwReceiver<(f64, WindowEvent)>, callback: F)
where
    F: Fn(),
{
    for (_, event) in glfw::flush_messages(&events) { // This works TODO put in function for easy use with a match for input sting to input type
        match event {
            glfw::WindowEvent::Key(key_type, _, action_type, _) => { 
                
            },
            _ => {},
        }
    }
    // call the closure
    callback();
    
}



pub fn inputmatch<F>(events: GlfwReceiver<(f64, WindowEvent)>, pattern: F)
where
    F: Fn(glfw::WindowEvent),
{
    for (_, event) in glfw::flush_messages(&events) {
        pattern(event);
    }
}


pub fn onupinputevent<F>(callback: F) -> impl Fn(glfw::WindowEvent)
where
    F: Fn(),
{
    let pattern = move |event: glfw::WindowEvent| {
        match event {
            glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                callback(); // run the passed function
            },
            _ => {},
        }
    };
    pattern
}