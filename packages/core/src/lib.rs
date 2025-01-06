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

// TODO consie tests maybe, or better documentation

// TODO at some point write some amazing documentation for everything

use minifb::{Key, Window, WindowOptions};

pub struct App {
    pub window: Window,
    pub buffer: Vec<u32>,
    pub should_close: bool,
    pub input_change: bool,
}

impl App {
    pub fn new(width: usize, height: usize, title: &str) -> Self {
        let (window, buffer) = defined_window(width, height, title);
        App {
            window,
            buffer,
            should_close: false,
            input_change: false,
        }
    }
}

pub fn defined_window( width: usize, height: usize, name: &str) -> (Window, Vec<u32>) {
    // Initialize the pixel buffer
    let mut buffer: Vec<u32> = vec![0; width * height];

    // Create a window
    let mut window = Window::new(
        name,
        width,
        height,
        //WindowOptions::default(),
        WindowOptions {
            resize: true,
            borderless: false,
            ..WindowOptions::default()
        },
    )
    .unwrap_or_else(|e| {
        panic!("Unable to open window: {}", e);
    });

    (window, buffer)
}

pub fn close(app: &mut App) {
    app.should_close = true;
}


pub fn input_pressed(app: &App, key: &str) -> bool {
    let minifb_key = match key {
        "esc" => Key::Escape,
        "1" => Key::Key1,
        "2" => Key::Key2,
        "3" => Key::Key3,
        "4" => Key::Key4,
        "5" => Key::Key5,
        "6" => Key::Key6,
        "7" => Key::Key7,
        "8" => Key::Key8,
        "9" => Key::Key9,
        "0" => Key::Key0,
        "a" => Key::A,
        "b" => Key::B,
        "c" => Key::C,
        "d" => Key::D,
        "e" => Key::E,
        "f" => Key::F,
        "g" => Key::G,
        "h" => Key::H,
        "i" => Key::I,
        "j" => Key::J,
        "k" => Key::K,
        "l" => Key::L,
        "m" => Key::M,
        "n" => Key::N,
        "o" => Key::O,
        "p" => Key::P,
        "q" => Key::Q,
        "r" => Key::R,
        "s" => Key::S,
        "t" => Key::T,
        "u" => Key::U,
        "v" => Key::V,
        "w" => Key::W,
        "x" => Key::X,
        "y" => Key::Y,
        "z" => Key::Z,
        "space" => Key::Space,
        "enter" => Key::Enter,
        "tab" => Key::Tab,
        "backspace" => Key::Backspace,
        "left" => Key::Left,
        "right" => Key::Right,
        "up" => Key::Up,
        "down" => Key::Down,
        "left_shift" => Key::LeftShift,
        "right_shift" => Key::RightShift,
        "left_ctrl" => Key::LeftCtrl,
        "right_ctrl" => Key::RightCtrl,
        "left_alt" => Key::LeftAlt,
        "right_alt" => Key::RightAlt,
        "semicolon" => Key::Semicolon,
        "apostrophe" => Key::Apostrophe,
        "comma" => Key::Comma,
        "period" => Key::Period,
        "slash" => Key::Slash,
        "backslash" => Key::Backslash,
        "left_bracket" => Key::LeftBracket,
        "right_bracket" => Key::RightBracket,
        "minus" => Key::Minus,
        "equals" => Key::Equal,
        "caps_lock" => Key::CapsLock,
        "scroll_lock" => Key::ScrollLock,
        "num_lock" => Key::NumLock,
        "pause" => Key::Pause,
        "insert" => Key::Insert,
        "home" => Key::Home,
        "page_up" => Key::PageUp,
        "delete" => Key::Delete,
        "end" => Key::End,
        "page_down" => Key::PageDown,
        "f1" => Key::F1,
        "f2" => Key::F2,
        "f3" => Key::F3,
        "f4" => Key::F4,
        "f5" => Key::F5,
        "f6" => Key::F6,
        "f7" => Key::F7,
        "f8" => Key::F8,
        "f9" => Key::F9,
        "f10" => Key::F10,
        "f11" => Key::F11,
        "f12" => Key::F12,
        _ => return false,
    };

    if app.window.is_key_down(minifb_key) {
        true
    } else {
        false
    }
}

/* 
//TODO
pub fn handle_mouse_button_event_asx(button: glfw::MouseButton) {
    let button_str = match button {
        glfw::MouseButton::Button1 => "mouse_button1",
        glfw::MouseButton::Button2 => "mouse_button2",
        glfw::MouseButton::Button3 => "mouse_button3",
        glfw::MouseButton::Button4 => "mouse_button4",
        glfw::MouseButton::Button5 => "mouse_button5",
        glfw::MouseButton::Button6 => "mouse_button6",
        glfw::MouseButton::Button7 => "mouse_button7",
        glfw::MouseButton::Button8 => "mouse_button8",
        _ => return,
    };

    if let Err(e) = std::fs::write("temp2.tmp", button_str.as_bytes()) {
        eprintln!("Failed to write to file: {}", e);
    }
}

//TODO
pub fn handle_mouse_scroll_event_asx(scroll: f64) {
    let scroll_str = scroll.to_string();

    if let Err(e) = std::fs::write("temp3.tmp", scroll_str.as_bytes()) {
        eprintln!("Failed to write to file: {}", e);
    }
}
*/

// window.set_background_color(255, 0, 0);