extern crate proc_macro;
use laststraw_core::Position;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Block, Ident, ItemStruct};

// TODO update change on file save

#[proc_macro]
pub fn asx(input: TokenStream) -> TokenStream {
    // todo wait until user input change to keep running
    let block = parse_macro_input!(input as Block);

    let expanded = quote! {
        {
            let mut _generated_block_wrapper = || {

                while app.window.is_open() && !app.should_close {

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

use rusttype::{point, Font, Scale};

#[proc_macro]
pub fn button_proc(input: TokenStream) -> TokenStream {
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
                    let font = Font::try_from_bytes(font_data).expect("Error loading font");
                    let scale = Scale::uniform(position.scale); // font size
                    let text_value = &app.next_button_text;
                    let text = "{text_value}";

                    if button_pressed {
                        #block
                    }
                }
            };
            _generated_block_wrapper();
        }
    };

    TokenStream::from(expanded)
}
