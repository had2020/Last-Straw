// By: Hadrian Lazic
// Licensed Under: MIT License and APACHE License
// Last Updated 2/1/2025
// LastStraw Idiomatic GUI framework in Rust
/*
 __                              __       ______    __
/  |                            /  |     /      \  /  |
$$ |        ______    _______  _$$ |_   /$$$$$$  |_$$ |_     ______   ______   __   __   __
$$ |       /      \  /       |/ $$   |  $$ \__$$// $$   |   /      \ /      \ /  | /  | /  |
$$ |       $$$$$$  |/$$$$$$$/ $$$$$$/   $$      \$$$$$$/   /$$$$$$  |$$$$$$  |$$ | $$ | $$ |
$$ |       /    $$ |$$      \   $$ | __  $$$$$$  | $$ | __ $$ |  $$/ /    $$ |$$ | $$ | $$ |
$$ |_____ /$$$$$$$ | $$$$$$  |  $$ |/  |/  \__$$ | $$ |/  |$$ |     /$$$$$$$ |$$ \_$$ \_$$ |
$$       |$$    $$ |/     $$/   $$  $$/ $$    $$/  $$  $$/ $$ |     $$    $$ |$$   $$   $$/
$$$$$$$$/  $$$$$$$/ $$$$$$$/     $$$$/   $$$$$$/    $$$$/  $$/       $$$$$$$/  $$$$$/$$$$/

*/
// Feel free to contribute!

#[warn(unused_variables)]
#[warn(unused_mut)]
// the warnings will be removed in the future
//#[warn(unused_imports)] // for the hashmap
use minifb::{Key, Window, WindowOptions};

/// debug log the version
pub fn log_version() {
    let version: &str = "2.2.1";
    println!("Version: ");
}

/// used to hold all the app infomation.
/// Their should always be one mutatable variable with the precise name "app".
/// This the App stuct to hold all of are current app window's
/// infomation, i.e, size height,
///  and some more that were needed from the minifb framework.
/// A variable must be set with the impl of new.
pub struct App {
    pub window: Window,
    pub buffer: Vec<u32>,

    pub should_close: bool,
    pub input_change: bool,

    // window dim
    pub height: usize,
    pub width: usize,

    // font path, TODO more fonts
    pub font_path: &'static [u8],

    // button
    pub next_button_position: Position,
    pub next_button_text: String,

    // text input
    pub current_text_edit_id: usize,
    pub selected_text_edit_id: usize,
    //pub input_text_storing: Vec<String>, // each index correlates to selected_text_edit_id assigned via calling sequence
    pub on_blinker: bool, // cycles on and off, on text element
    pub multi_line_storing: Vec<Vec<String>>, // each index matchs id, nut for each index of index, stands for a line of text
    pub already_set_initial_text: bool,
}

/// used to set a mutable variable with the name app to the App struct.
/// The variable created is used to edit the app window in real time, within the asx loop.
/// First argument is window height.
/// Second argument is window width.
/// Third argument is an &str of name of the program and window.
impl App {
    pub fn new(width: usize, height: usize, title: &str) -> Self {
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
            current_text_edit_id: 0,
            selected_text_edit_id: 0,
            multi_line_storing: vec![
                vec![String::new(), String::new()],
                vec![String::new(), String::new()],
            ],
            on_blinker: true,
            already_set_initial_text: false,
        }
    }
}

/// This fucntion defines a minfb window, it is used by App struct, to create a window, with settings
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
            resize: true, // this is important as it can mess with you application, making elements seam out of place.
            borderless: false,
            ..WindowOptions::default()
        },
    )
    .unwrap_or_else(|e| {
        panic!("Unable to open window: {}", e); // if you open an issue please include your operating system.
    });

    (window, buffer)
}

/// Function that you can use to close your app.
/// Same as app.should_close = true, but more smaller.
pub fn close(app: &mut App) {
    app.should_close = true;
}

/// Please, refer to this for the color map.
/// Colors are in hexademical.
/// For easy usage,
/// you can just pass in a &str to get the hexademical.
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

/// You should use this to set the background, of the window.
/// Tip make sure it as at the topmost of asx, so that it is the buttom layer.
/// You could also use it to clear over elements called below it.
pub fn set_window_color(app: &mut App, color: &str) {
    //TODO non hex code based system
    let new_color = hex_color(color);

    for pixel in app.buffer.iter_mut() {
        *pixel = new_color;
    }
}

//use std::{collections::HashMap, f64::RADIX};

/// Refer to this when you need a key, as it is more ideomatic.
/// Map of all the keys, in &str.
pub fn string_to_key_hash_map<'a>() -> std::collections::HashMap<&'a str, Key> {
    let key_mappings: std::collections::HashMap<&str, Key> = [
        ("esc", Key::Escape),
        ("1", Key::Key1),
        ("2", Key::Key2),
        ("3", Key::Key3),
        ("4", Key::Key4),
        ("5", Key::Key5),
        ("6", Key::Key6),
        ("7", Key::Key7),
        ("8", Key::Key8),
        ("9", Key::Key9),
        ("0", Key::Key0),
        ("a", Key::A),
        ("b", Key::B),
        ("c", Key::C),
        ("d", Key::D),
        ("e", Key::E),
        ("f", Key::F),
        ("g", Key::G),
        ("h", Key::H),
        ("i", Key::I),
        ("j", Key::J),
        ("k", Key::K),
        ("l", Key::L),
        ("m", Key::M),
        ("n", Key::N),
        ("o", Key::O),
        ("p", Key::P),
        ("q", Key::Q),
        ("r", Key::R),
        ("s", Key::S),
        ("t", Key::T),
        ("u", Key::U),
        ("v", Key::V),
        ("w", Key::W),
        ("x", Key::X),
        ("y", Key::Y),
        ("z", Key::Z),
        ("space", Key::Space),
        ("enter", Key::Enter),
        ("tab", Key::Tab),
        ("backspace", Key::Backspace),
        ("left", Key::Left),
        ("right", Key::Right),
        ("up", Key::Up),
        ("down", Key::Down),
        ("left_shift", Key::LeftShift),
        ("right_shift", Key::RightShift),
        ("left_ctrl", Key::LeftCtrl),
        ("right_ctrl", Key::RightCtrl),
        ("left_alt", Key::LeftAlt),
        ("right_alt", Key::RightAlt),
        (";", Key::Semicolon),
        ("'", Key::Apostrophe),
        (",", Key::Comma),
        (".", Key::Period),
        ("/", Key::Slash),
        ("backslash", Key::Backslash),
        ("left_bracket", Key::LeftBracket),
        ("right_bracket", Key::RightBracket),
        ("-", Key::Minus),
        ("+", Key::Equal),
        ("caps_lock", Key::CapsLock),
        ("scroll_lock", Key::ScrollLock),
        ("num_lock", Key::NumLock),
        ("pause", Key::Pause),
        ("insert", Key::Insert),
        ("home", Key::Home),
        ("page_up", Key::PageUp),
        ("delete", Key::Delete),
        ("end", Key::End),
        ("page_down", Key::PageDown),
        ("f1", Key::F1),
        ("f2", Key::F2),
        ("f3", Key::F3),
        ("f4", Key::F4),
        ("f5", Key::F5),
        ("f6", Key::F6),
        ("f7", Key::F7),
        ("f8", Key::F8),
        ("f9", Key::F9),
        ("f10", Key::F10),
        ("f11", Key::F11),
        ("f12", Key::F12),
    ]
    .iter()
    .cloned()
    .collect();
    key_mappings
}

/// used in input's to use the hash map acordingly, or flip.
pub fn key_to_string_hash_map<'a>() -> std::collections::HashMap<Key, &'a str> {
    let flipped_mappings: std::collections::HashMap<Key, &str> = string_to_key_hash_map()
        .into_iter()
        .map(|(key, value)| (value, key))
        .collect();

    flipped_mappings
}

/// Method for checking for if the user has pressed, a key.
/// Returns bool true, if the key is pressed.
pub fn input_pressed(app: &App, key: &str) -> bool {
    let key_mappings = string_to_key_hash_map();
    if let Some(&minifb_key) = key_mappings.get(key) {
        app.window.is_key_down(minifb_key)
    } else {
        false
    }
}

/// Used in interactable text element, to check for multiple keys pressed.
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

/// Simple struct for readability and ideomatic, on elements.
/// used for setting location of a element on the screen.
/// Made up of X: Height, Y: Weight, and Scale.
/// All are floats, as these are rough positions on screen
/// Methods requiring Postition can be used by writing out this struct, or using
/// the position!() macro.
#[derive(Clone)]
pub struct Position {
    pub x: f32,
    pub y: f32,
    pub scale: f32,
}

/// First argument is a declared App stuct.
/// Second argument is a stuct called position,
/// it is made up of X: Height, Y: Weight, and Scale, deciding were the ui is placed.
/// Third argument is a &str, with the message,
/// displayed on the string. This does not take user input,
/// but only displays some text.
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
                    let color = (v * 255.0) as u32; // TODO custom color
                    app.buffer[py * app.width + px] = (color << 16) | (color << 8) | color;
                }
            });
        }
    }
}

/// Method used for drawing text, with the font, in other Methods namely Interactable text.
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

// TODO custom color, TODO move blinker with arrow keys, and highlighting
/// Element used for drawing multiple lines of text.
/// First argument is a declared App stuct.
/// Second argument is a stuct called position, it is made up of X: Height, Y: Weight, and Scale, deciding were the ui is placed.
/// Third argument, are the lines of text, each index a line, stored in a Vec<&str>
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

/// Verison of multiline text expecting a string for a thrid argument, as teh message.
/// Namly used in Interactable text for less borrowing.
pub fn multi_line_text_owned(app: &mut App, position: Position, spacing: f32, text: Vec<String>) {
    let font_data = FONT_BYTES;
    let font = Font::try_from_bytes(font_data).expect("Error loading font");

    let scale = Scale::uniform(position.scale);
    let mut iteration_position: rusttype::Point<f32>;

    iteration_position = point(position.x, position.y);

    for line in text.iter() {
        iteration_position = point(iteration_position.x, iteration_position.y + spacing);
        rasterize_text(&font, line, scale, iteration_position, app);
    }
}

/// Simple method used namely in Interactable text, for drawing a box.
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

/// Used for Idiomatics, for less work when asigning a position arugment
/// Takes three floats. X: height, Y: width, and Scale: scale, all as floats.
/// Note it is less readable then just declaring a new Position struct.
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

/// Method used in layering the positioning of elements.
/// Impacted in asx
pub fn set_next_button(app: &mut App, position: Position) {
    app.next_button_position = position;
}

/// Sets the text for the next button.
/// The button most close when reading down.
pub fn set_next_button_text(app: &mut App, text: &str) {
    app.next_button_text = text.to_string();
}

/// Used in other methods, namely in Interactable text, and button for drawing a box.
pub fn calculate_button_text_dimensions(font: &Font, text: &str, scale: Scale) -> (f32, f32) {
    let glyphs: Vec<_> = font.layout(text, scale, point(0.0, 0.0)).collect();

    // total width based on glyph positions
    let width = glyphs
        .iter()
        .last()
        .and_then(|g| g.pixel_bounding_box().map(|bb| bb.max.x as f32))
        .unwrap_or(0.0)
        - glyphs
            .iter()
            .next()
            .and_then(|g| g.pixel_bounding_box().map(|bb| bb.min.x as f32))
            .unwrap_or(0.0);

    // height using font metrics
    let v_metrics = font.v_metrics(scale);
    let height = v_metrics.ascent - v_metrics.descent + v_metrics.line_gap;

    (width, height)
}

// TODO better method, and hot-reloading
/// Unfinished function, to be used in hot-reloading
pub fn dev_mode() -> bool {
    #[cfg(debug_assertions)]
    {
        //println!("(likely during development).");
        return true;
    }

    #[cfg(not(debug_assertions))]
    {
        //println!("(compiled binary).");
        return false;
    }
}

/// Used in other Methods/Elements for drawing boxes.
pub fn draw_box(
    app: &mut App,
    position: Position,
    box_width: usize,
    box_height: usize,
    color: &str,
) {
    let box_x: usize = position.x as usize;
    let box_y: usize = position.y as usize;
    for y in box_y..box_y + box_height {
        for x in box_x..box_x + box_width {
            if x < app.width && y < app.height {
                //bounds check
                app.buffer[y * app.width + x] = hex_color(color);
            }
        }
    }
}

use minifb::{CursorStyle, KeyRepeat};

//TODO caps
//TODO password protected function
// TODO highlight or no highlight
// TODO custom color, TODO move blinker with arrow keys, and highlighting

/// Used for getting multi or single line text input, from users.
/// First argument is the declared App stuct, from outside asx.
/// Second is a macro declaring a position struct, Hight, Width, Scale.
/// The text seen before any text is enter by user.
/// Color from color table, refered to at end of docs, or laststraw_core.
/// If the element would only accept single line, false, means single line only.
pub fn editable_lines(
    app: &mut App,
    position: Position,
    initial_text: &str,
    color: &str,
    single_line: bool,
) -> String {
    app.current_text_edit_id += 1;

    let mut letter_input_checked: bool = false;

    let mut button_pressed = false;
    let mut enter = false; // set to true, if enter key pressed
    let mut line_removed = false; // tracks when a line is removed, for no index errors

    let none_selected = app.selected_text_edit_id == 0;
    let selected = app.selected_text_edit_id == app.current_text_edit_id && !none_selected;
    let non_empty_position = position.x != 0.0 && position.y != 0.0 && position.scale != 0.0;

    let last_line = app.multi_line_storing[app.selected_text_edit_id].len() - 1;
    //println!("{},", last_line); // Debugging

    let mut line_text = app.multi_line_storing[app.selected_text_edit_id][last_line].clone();

    //println!("PREmls: {:?}", app.multi_line_storing); //[last_line]);

    if non_empty_position && !selected {
        let left_down = app.window.get_mouse_down(minifb::MouseButton::Left);

        let mouse_pos = app
            .window
            .get_mouse_pos(minifb::MouseMode::Clamp)
            .unwrap_or((0.0, 0.0));
        let (mouse_x, mouse_y) = (mouse_pos.0 as f32, mouse_pos.1 as f32);

        let mut text: &str;
        let text_value = line_text.clone();

        if line_text.len() > 0 {
            text = &text_value;
        } else {
            text = initial_text;
        }

        let mut position_iterator = Position {
            x: position.x,
            y: position.y,
            scale: position.scale,
        };

        //println!("app.multi_line_storing: {:?}", app.multi_line_storing); // for Debuging changes
        for line in 0..last_line {
            let font_data = &app.font_path;
            let font = rusttype::Font::try_from_bytes(font_data).expect("Error loading font");
            let scale = rusttype::Scale::uniform(position_iterator.scale); // font size

            let (text_width, text_height) = calculate_button_text_dimensions(&font, text, scale);

            let v_metrics = font.v_metrics(scale);
            let ascent = v_metrics.ascent;
            let descent = v_metrics.descent;

            let rect_y = position_iterator.y - ascent;
            let rect_height = text_height + descent.abs();

            let is_within_button = mouse_x >= position_iterator.x
                && mouse_x <= position_iterator.x + text_width
                && mouse_y >= rect_y
                && mouse_y <= rect_y + rect_height;

            if left_down && is_within_button {
                button_pressed = true;
            }

            let highlight_color = hex_color(color); // TODO custom box color and outline or solid boolean

            draw_rectangle(
                &mut app.buffer,
                app.width,
                app.height,
                position.x,
                rect_y,
                text_width,
                rect_height,
                highlight_color,
            );

            // rasterize and draw text
            let start_point = rusttype::point(position_iterator.x, position_iterator.y);

            let glyphs: Vec<_> = font.layout(text, scale, start_point).collect();
            for glyph in glyphs {
                if let Some(bounding_box) = glyph.pixel_bounding_box() {
                    glyph.draw(|x, y, v| {
                        let px = (bounding_box.min.x + x as i32) as usize;
                        let py = (bounding_box.min.y + y as i32) as usize;

                        if px < app.width && py < app.height {
                            let color = (v * 255.0) as u32; // TODO custom color
                            app.buffer[py * app.width + px] = (color << 16) | (color << 8) | color;
                        }
                    });
                }
            }

            position_iterator.y += 50.0; //* (position_iterator.scale / 10.0); TODO!
        }

        if button_pressed {
            app.selected_text_edit_id = app.current_text_edit_id;
        }
    } else {
        // selected
        app.window.set_cursor_style(CursorStyle::Ibeam);

        // blinker TODO move with letters
        /*
        if app.on_blinker == true {
            app.on_blinker = false;
            draw_box(
                app,
                Position {
                    x: position.clone().x - (1.0 * position.scale / 10.0),
                    y: position.clone().y - (5.0 * position.scale / 10.0),
                    scale: 0.0,
                },
                1 * position.scale as usize / 10,
                5 * position.scale as usize / 10,
                "White",
            );
        } else {
            app.on_blinker = true;
        }
        */

        let mut string_set_id_index: String = String::new();
        let key_mappings = key_to_string_hash_map();
        let mut backspace = false;

        for key in app.window.get_keys_pressed(KeyRepeat::Yes).iter() {
            let new_string: String = format!("{}", key_mappings.get(key).unwrap());
            letter_input_checked = true;
            let string_change = match new_string.as_str() {
                "space" => " ", // TODO expand system for caps and shift caps.
                _ => new_string.as_str(),
            };

            if string_change.len() == 1 {
                string_set_id_index.push_str(string_change);
            } else {
                if string_change == "backspace" {
                    backspace = true
                }
                if string_change == "delete" {
                    backspace = true
                }
                if string_change == "enter" {
                    enter = true
                }
                //println!("{}", string_change); // debugging missing keys
            }
        }

        // char adding
        if letter_input_checked && !backspace {
            let last_stored_text = &line_text;
            line_text = format!("{}{}", last_stored_text, string_set_id_index);
            letter_input_checked = false;

        // backspace, char deletion
        } else if letter_input_checked && backspace {
            let mut already_removed_line = false;
            let mut full_current_text: String = line_text.clone();

            if full_current_text.is_empty() {
                if !already_removed_line
                    && app.multi_line_storing[app.selected_text_edit_id].len() > 2
                {
                    app.multi_line_storing[app.selected_text_edit_id].pop();
                    already_removed_line = true;
                    line_removed = true;
                }
            } else {
                full_current_text.pop();
            }

            line_text = full_current_text;
            backspace = false;
            letter_input_checked = false;
        }

        // enter, new index line
        if enter && !single_line {
            app.multi_line_storing[app.selected_text_edit_id].push(String::new());
        }
    }

    if selected {
        let full_current_text: String = line_text.clone();
        //single_line_text(app, position, &full_current_text);
        multi_line_text_owned(
            app,
            Position {
                x: position.x,
                y: position.y - 100.0,
                scale: position.scale,
            },
            50.0,
            app.multi_line_storing[app.selected_text_edit_id].clone(),
        );

        // updating app's memory of the text
        if !enter && !line_removed {
            //println!("lt: {:?}", line_text); // Debugging
            app.multi_line_storing[app.selected_text_edit_id][last_line] = line_text;
            //println!("mls: {:?}", app.multi_line_storing[last_line]);
        } else if !enter && line_removed {
            app.multi_line_storing[app.selected_text_edit_id][last_line - 1] = line_text;
        }

        full_current_text
    } else {
        String::new() // empty if no input TODO replace with optional in a new macro
    }
}

/// Note: this is optional, and used to reduce CPU usage for much heavy apps, with many elements.
/// Tip: between 60-30, is okay.
/// First argument is the declared App stuct, from outside asx.
/// Second argument is the max frames per a second.
pub fn limit_fps(app: &mut App, fps: f32) {
    if fps > 1.0 {
        app.window
            .limit_update_rate(Some(std::time::Duration::from_secs_f32(1.0 / fps)));
    } else {
        eprintln!("limit_fps(), fps set too low.")
    }
}

/// Set the first line of the following interactable text element, to a &str
pub fn following_input_initial_line(app: &mut App, initial_line: &str) {
    if app.already_set_initial_text == false {
        app.multi_line_storing[app.current_text_edit_id + 1][1] = initial_line.to_string();
        app.already_set_initial_text = true;
    }
}

/// Sets the initial lines in the following interactable text element, to vec of &strs,
pub fn following_input_initial_lines(app: &mut App, initial_lines: Vec<&str>) {
    if app.already_set_initial_text == false {
        let mut line_iteration: usize = 0;
        for line in initial_lines {
            line_iteration += 1;
            app.multi_line_storing[app.current_text_edit_id + 1][line_iteration] = line.to_string();
        }
        app.already_set_initial_text = true;
    }
}
