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

/* 
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
*/

pub fn input_pressed(key: &str) -> bool {
    let input = std::fs::read("temp.tmp").unwrap_or_else(|_| {
        std::fs::File::create("temp.tmp").unwrap();
        Vec::new()
    });
    let input_str = String::from_utf8(input).unwrap_or_default();

    if let Err(e) = std::fs::write("temp.tmp", "") { // clear file to prevent too many loops of the same key
        eprintln!("Failed to write to file: {}", e);
    }

    input_str == key
}

// all allowed keys in asx are listed here for reabability
pub fn handle_key_event_asx(key: glfw::Key) {
    let key_str = match key {
        glfw::Key::Escape => "esc",
        glfw::Key::Q => "Q",
        glfw::Key::W => "W",
        glfw::Key::E => "E",
        glfw::Key::R => "R",
        glfw::Key::T => "T",
        glfw::Key::Y => "Y",
        glfw::Key::U => "U",
        glfw::Key::I => "I",
        glfw::Key::O => "O",
        glfw::Key::P => "P",
        glfw::Key::A => "A",
        glfw::Key::S => "S",
        glfw::Key::D => "D",
        glfw::Key::F => "F",
        glfw::Key::G => "G",
        glfw::Key::H => "H",
        glfw::Key::J => "J",
        glfw::Key::K => "K",
        glfw::Key::L => "L",
        glfw::Key::Z => "Z",
        glfw::Key::X => "X",
        glfw::Key::C => "C",
        glfw::Key::V => "V",
        glfw::Key::B => "B",
        glfw::Key::N => "N",
        glfw::Key::M => "M",
        glfw::Key::Space => "space",
        glfw::Key::Enter => "enter",
        glfw::Key::Tab => "tab",
        glfw::Key::Backspace => "backspace",
        glfw::Key::Left => "left_arrow",
        glfw::Key::Right => "right_arrow",
        glfw::Key::Up => "up_arrow",
        glfw::Key::Down => "down_arrow",
        glfw::Key::LeftShift => "left_shift",
        glfw::Key::RightShift => "right_shift",
        glfw::Key::LeftControl => "left_ctrl",
        glfw::Key::RightControl => "right_ctrl",
        glfw::Key::LeftAlt => "left_alt",
        glfw::Key::RightAlt => "right_alt",
        glfw::Key::F1 => "F1",
        glfw::Key::F2 => "F2",
        glfw::Key::F3 => "F3",
        glfw::Key::F4 => "F4",
        glfw::Key::F5 => "F5",
        glfw::Key::F6 => "F6",
        glfw::Key::F7 => "F7",
        glfw::Key::F8 => "F8",
        glfw::Key::F9 => "F9",
        glfw::Key::F10 => "F10",
        glfw::Key::F11 => "F11",
        glfw::Key::F12 => "F12",
        glfw::Key::Num0 => "0",
        glfw::Key::Num1 => "1",
        glfw::Key::Num2 => "2",
        glfw::Key::Num3 => "3",
        glfw::Key::Num4 => "4",
        glfw::Key::Num5 => "5",
        glfw::Key::Num6 => "6",
        glfw::Key::Num7 => "7",
        glfw::Key::Num8 => "8",
        glfw::Key::Num9 => "9",
        glfw::Key::Period => ".",
        glfw::Key::Comma => ",",
        glfw::Key::Semicolon => ";",
        glfw::Key::Apostrophe => "'",
        glfw::Key::Slash => "/",
        glfw::Key::Backslash => "\\",
        glfw::Key::LeftBracket => "[",
        glfw::Key::RightBracket => "]",
        glfw::Key::Minus => "-",
        glfw::Key::Equal => "=",
        _ => return,
    };

    if let Err(e) = std::fs::write("temp.tmp", key_str.as_bytes()) {
        eprintln!("Failed to write to file: {}", e);
    }
}