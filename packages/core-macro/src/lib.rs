extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Block};

// TODO update change on file save

#[proc_macro]
pub fn asx(input: TokenStream) -> TokenStream {
    // todo wait until user input change to keep running
    let block = parse_macro_input!(input as Block);

    let expanded = quote! {
        {
            let is_dev_mode: bool = dev_mode();
            if is_dev_mode {
                //std::process::exit(1);
            }

            let mut last_window_size = app.window.get_size();

            let mut _generated_block_wrapper = || {

                while app.window.is_open() && !app.should_close {

                    app.current_text_edit_id = 0;

                    let current_window_size = app.window.get_size(); //TODO better resize
                    if current_window_size != last_window_size {
                        let (new_width, new_height) = current_window_size;
                        app.buffer.resize(new_width * new_height, 0);
                        app.width = new_width;
                        app.height = new_height;
                        last_window_size = current_window_size;
                    }

                    #block
                    // TODO add optional pause if no input event

                    app.window.update_with_buffer(&app.buffer, app.width, app.height).unwrap();
                }
            };
            _generated_block_wrapper();
        }
    };

    TokenStream::from(expanded)
}

// TODO under text button, and more colors, also outline and none outline
#[proc_macro]
pub fn button(input: TokenStream) -> TokenStream {
    let block = parse_macro_input!(input as Block);

    let expanded = quote! {
        {
            let mut _generated_block_wrapper = || {

                let position = app.next_button_position.clone();
                let mut button_pressed = false;

                if position.x != 0.0 && position.y != 0.0 && position.scale != 0.0 {

                    let left_down = app.window.get_mouse_down(minifb::MouseButton::Left);

                    let mouse_pos = app
                        .window
                        .get_mouse_pos(minifb::MouseMode::Clamp)
                        .unwrap_or((0.0, 0.0));
                    let (mouse_x, mouse_y) = (mouse_pos.0 as f32, mouse_pos.1 as f32);

                    let font_data = &app.font_path;
                    let font = rusttype::Font::try_from_bytes(font_data).expect("Error loading font");
                    let scale = rusttype::Scale::uniform(position.scale); // font size
                    let text_value = &app.next_button_text;
                    let text = text_value;

                    // dimensions
                    let (text_width, text_height) = calculate_button_text_dimensions(&font, text, scale);

                    // check if mouse within
                    let is_within_button = mouse_x >= position.x
                        && mouse_x <= position.x + text_width
                        && mouse_y >= position.y
                        && mouse_y <= position.y + text_height;

                    if left_down && is_within_button {
                        button_pressed = true
                    }

                    // outline box logic
                    let highlight_color = 0xFF0000; // TODO custom box color and outline or draw propertys
                    draw_rectangle(
                        &mut app.buffer,
                        app.width,
                        app.height,
                        position.x,
                        position.y,
                        text_width,
                        text_height,
                        highlight_color,
                    );

                    // rasterize and draw text
                    let start_point = rusttype::point(position.x, position.y);
                    let glyphs: Vec<_> = font.layout(text, scale, start_point).collect();
                    for glyph in glyphs {
                        if let Some(bounding_box) = glyph.pixel_bounding_box() {
                            glyph.draw(|x, y, v| {
                                let px = (bounding_box.min.x + x as i32) as usize;
                                let py = (bounding_box.min.y + y as i32) as usize;

                                if px < app.width && py < app.height {
                                    let color = (v * 255.0) as u32;
                                    app.buffer[py * app.width + px] = (color << 16) | (color << 8) | color;
                                }
                            });
                        }
                    }

                    if button_pressed {
                        button_pressed = false;
                        #block
                    }
                }
            };
            _generated_block_wrapper();
        }
    };

    TokenStream::from(expanded)
}
