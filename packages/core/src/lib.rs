// TODO at some point write some amazing documentation for everything

use minifb::{Key, Window, WindowOptions};

pub struct App {
    pub window: Window,
    pub buffer: Vec<u32>,
    pub should_close: bool,
    pub input_change: bool,
    pub height: usize,
    pub width: usize,
    pub font_path: &'static [u8],
    pub next_button_position: Position,
    pub next_button_text: String,
}

impl App {
    pub fn new(width: usize, height: usize, title: &str) -> Self {
        #![cfg_attr(target_os = "windows", windows_subsystem = "windows")] // hiding cmd, windows //TODO better build system

        let (window, buffer) = defined_window(width, height, title);
        App {
            window,
            buffer,
            should_close: false,
            input_change: false,
            height: height,
            width: width,
            font_path: include_bytes!("../assets/fonts/FiraSans-Regular.ttf"),
            next_button_position: Position {
                x: 0.0,
                y: 0.0,
                scale: 0.0,
            },
            next_button_text: String::from(""),
        }
    }
}

pub fn defined_window(width: usize, height: usize, name: &str) -> (Window, Vec<u32>) {
    // Initialize the pixel buffer
    let buffer: Vec<u32> = vec![0; width * height];

    // Create a window
    let window = Window::new(
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

pub fn hex_color(name: &str) -> u32 {
    let new_color: u32 = match name {
        "Green" => 0xFF_00FF00,
        "Red" => 0xFF_FF0000,
        "Blue" => 0xFF_0000FF,
        "Yellow" => 0xFF_FFFF00,
        "Cyan" => 0xFF_00FFFF,
        "Magenta" => 0xFF_FF00FF,
        "White" => 0xFF_FFFFFF,
        "Black" => 0xFF_000000,
        "Gray" => 0xFF_808080,
        "Orange" => 0xFF_FFA500,
        "Purple" => 0xFF_800080,
        "Pink" => 0xFF_FFC0CB,
        "Brown" => 0xFF_A52A2A,
        "Light Gray" => 0xFF_D3D3D3,
        "Light Blue" => 0xFF_ADD8E6,
        "Dark Blue" => 0xFF_00008B,
        "Beige" => 0xFF_F5F5DC,
        "Teal" => 0xFF_008080,
        "Lavender" => 0xFF_E6E6FA,
        "Ivory" => 0xFF_FFFFF0,
        "Mint" => 0xFF_98FF98,
        "Coral" => 0xFF_FF7F50,
        "Navy" => 0xFF_000080,
        "Sky Blue" => 0xFF_87CEEB,
        "Sea Green" => 0xFF_2E8B57,
        "Forest Green" => 0xFF_228B22,
        "Dark Gray" => 0xFF_A9A9A9,
        "Slate Gray" => 0xFF_708090,
        "Charcoal" => 0xFF_36454F,
        "Jet Black" => 0xFF_343434,
        "Gunmetal" => 0xFF_2A3439,
        "Dark Slate Blue" => 0xFF_483D8B,
        "Midnight Blue" => 0xFF_191970,
        "Deep Navy" => 0xFF_1B1F3B,
        "Dark Olive Green" => 0xFF_556B2F,
        "Deep Forest Green" => 0xFF_1A2E1A,
        "Maroon" => 0xFF_800000,
        "Deep Burgundy" => 0xFF_4A0000,
        "Dark Chocolate" => 0xFF_3E2723,
        "Dark Copper" => 0xFF_4E3629,
        "Onyx" => 0xFF_353839,
        "Obsidian" => 0xFF_1C1C1C, // Great for dark mode
        _ => 0xFF_FFC0CB,          // defaults to pink if invaild color text
    };
    new_color
}

pub fn set_window_color(app: &mut App, color: &str) {
    //TODO non hex code based system
    let new_color = hex_color(color);

    for pixel in app.buffer.iter_mut() {
        *pixel = new_color;
    }
}

// https://docs.rs/minifb/latest/minifb/struct.Window.html#method.get_keys
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

/* How Use, TODO incapsulate
    keys_down(&app, |key| {
        match key {
            minifb::Key::W => println!("W key pressed"),
            minifb::Key::A => println!("A key pressed"),
            minifb::Key::S => println!("S key pressed"),
            minifb::Key::D => println!("D key pressed"),
            _ => println!("{:?} key pressed", key),
            }
        });
*/
pub fn keys_down<F>(app: &App, handle_key: F)
where
    F: Fn(&minifb::Key),
{
    app.window
        .get_keys_pressed(minifb::KeyRepeat::No)
        .iter()
        .for_each(|key| handle_key(key));
}

/*
//TODO window Scrolling
pub fn handle_mouse_scroll_event_asx(scroll: f64) {
    let scroll_str = scroll.to_string();

    if let Err(e) = std::fs::write("temp3.tmp", scroll_str.as_bytes()) {
        eprintln!("Failed to write to file: {}", e);
    }
}
*/

use rusttype::{point, Font, Scale};
const FONT_BYTES: &[u8] = include_bytes!("../assets/fonts/FiraSans-Regular.ttf"); // always having font loaded in package

#[derive(Clone)]
pub struct Position {
    pub x: f32,
    pub y: f32,
    pub scale: f32,
}

pub fn single_line_text(app: &mut App, position: Position, text: &str) {
    let font_data = FONT_BYTES;
    let font = Font::try_from_bytes(font_data).expect("Error loading font");

    // settings
    let scale1 = Scale::uniform(position.scale); // font size
    let start_point = point(position.x, position.y); // starting position of the text

    // rasterize the text
    let glyphs: Vec<_> = font.layout(text, scale1, start_point).collect();
    for glyph in glyphs {
        if let Some(bounding_box) = glyph.pixel_bounding_box() {
            glyph.draw(|x, y, v| {
                let px = (bounding_box.min.x + x as i32) as usize;
                let py = (bounding_box.min.y + y as i32) as usize;

                if px < app.width && py < app.height {
                    let color = (v * 255.0) as u32; // grayscale value
                    app.buffer[py * app.width + px] = (color << 16) | (color << 8) | color;
                }
            });
        }
    }
}

pub fn rasterize_text(
    font: &Font<'_>,
    line: &str,
    scale: Scale,
    start_point: rusttype::Point<f32>,
    app: &mut App,
) {
    let glyphs: Vec<_> = font.layout(line, scale, start_point).collect();
    for glyph in glyphs {
        if let Some(bounding_box) = glyph.pixel_bounding_box() {
            glyph.draw(|x, y, v| {
                let px = (bounding_box.min.x + x as i32) as usize;
                let py = (bounding_box.min.y + y as i32) as usize;

                if px < app.width && py < app.height {
                    let color = (v * 255.0) as u32; // grayscale value
                    app.buffer[py * app.width + px] = (color << 16) | (color << 8) | color;
                }
            });
        }
    }
}

pub fn multi_line_text(app: &mut App, position: Position, spacing: f32, text: Vec<&str>) {
    let font_data = FONT_BYTES;
    let font = Font::try_from_bytes(font_data).expect("Error loading font");

    // settings
    let scale = Scale::uniform(position.scale); // font size
    let mut iteration_position: rusttype::Point<f32>;

    iteration_position = point(position.x, position.y);

    for line in text.iter() {
        iteration_position = point(iteration_position.x, iteration_position.y + spacing);
        rasterize_text(&font, line, scale, iteration_position, app);
    }
}

pub fn editable_single_line(app: &mut App, position: Position, initial_text: &str) {
    let font_data = FONT_BYTES;
    let font = Font::try_from_bytes(font_data).expect("Error loading font");

    // settings
    let scale1 = Scale::uniform(position.scale); // font size
    let start_point = point(position.x, position.y); // starting position of the text

    // rasterize the text
    let glyphs: Vec<_> = font.layout(initial_text, scale1, start_point).collect();
    for glyph in glyphs {
        if let Some(bounding_box) = glyph.pixel_bounding_box() {
            glyph.draw(|x, y, v| {
                let px = (bounding_box.min.x + x as i32) as usize;
                let py = (bounding_box.min.y + y as i32) as usize;

                if px < app.width && py < app.height {
                    let color = (v * 255.0) as u32; // grayscale value
                    app.buffer[py * app.width + px] = (color << 16) | (color << 8) | color;
                }
            });
        }
    }
}

pub fn draw_rectangle(
    buffer: &mut Vec<u32>,
    width: usize,
    height: usize,
    x: f32,
    y: f32,
    rect_width: f32,
    rect_height: f32,
    color: u32,
) {
    let x_start = x as usize;
    let y_start = y as usize;
    let x_end = (x + rect_width) as usize;
    let y_end = (y + rect_height) as usize;

    for py in y_start..y_end {
        for px in x_start..x_end {
            if px < width && py < height {
                // draw outline only
                if py == y_start || py == y_end - 1 || px == x_start || px == x_end - 1 {
                    buffer[py * width + px] = color;
                }
            }
        }
    }
}

/* // can also each can be used for any position args
Position {
    x: 80.0,
    y: 80.0,
    scale: 30.0,
},
*/
#[macro_export]
macro_rules! position {
    ($x:expr, $y:expr, $scale:expr) => {
        Position {
            x: $x,
            y: $y,
            scale: $scale,
        }
    };
}

pub fn set_next_button(app: &mut App, position: Position) {
    app.next_button_position = position;
}

pub fn set_next_button_text(app: &mut App, text: &str) {
    app.next_button_text = text.to_string();
}

pub fn calculate_button_text_dimensions(font: &Font, text: &str, scale: Scale) -> (f32, f32) {
    let glyphs: Vec<_> = font.layout(text, scale, point(0.0, 0.0)).collect();
    let width = glyphs
        .iter()
        .filter_map(|g| g.pixel_bounding_box())
        .map(|bb| bb.max.x as f32 - bb.min.x as f32)
        .sum::<f32>();
    let height = scale.y; // esimated font, scale, height
    (width, height)
}

pub fn dev_mode() -> bool {
    #[cfg(debug_assertions)]
    {
        println!("(likely during development).");
        return true;
    }

    #[cfg(not(debug_assertions))]
    {
        println!("(compiled binary).");
        return false;
    }
}
