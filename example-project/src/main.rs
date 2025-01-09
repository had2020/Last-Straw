use laststraw::*;

fn main() {
    let mut app = App::new(500, 500, "test");

    asx!({
        set_window_color(&mut app, "Obsidian"); // is layored so this is back

        if input_pressed(&app, "esc") {
            app.should_close = true;
        }

        // TODO find out how to pass extra vars in a macro to replace this clutter
        if input_pressed(&app, "space") {
            single_line_text(
                &mut app,
                Position {
                    x: 80.0,
                    y: 80.0,
                    scale: 30.0,
                },
                "You pressed space",
            );
        }

        let lines: Vec<&str> = vec!["Apple fruit", "Banana", "Cherry pie", "Oreos"];
        multi_line_text(
            &mut app,
            Position {
                x: 100.0,
                y: 100.0,
                scale: 50.0,
            },
            50.0,
            lines,
        );

        // TODO functionize

        use rusttype::{point, Font, Scale};

        let position = Position {
            x: 100.0,
            y: 200.0,
            scale: 20.0,
        };

        let left_down = app.window.get_mouse_down(minifb::MouseButton::Left);

        let mouse_pos = app
            .window
            .get_mouse_pos(minifb::MouseMode::Clamp)
            .unwrap_or((0.0, 0.0));
        let (mouse_x, mouse_y) = (mouse_pos.0 as f32, mouse_pos.1 as f32);

        fn calculate_text_dimensions(font: &Font, text: &str, scale: Scale) -> (f32, f32) {
            let glyphs: Vec<_> = font.layout(text, scale, point(0.0, 0.0)).collect();
            let width = glyphs
                .iter()
                .filter_map(|g| g.pixel_bounding_box())
                .map(|bb| bb.max.x as f32 - bb.min.x as f32)
                .sum::<f32>();
            let height = scale.y; // Approximation: font scale height
            (width, height)
        }

        fn draw_rectangle(
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
                        // Draw the rectangle's edges (outline only)
                        if py == y_start || py == y_end - 1 || px == x_start || px == x_end - 1 {
                            buffer[py * width + px] = color;
                        }
                    }
                }
            }
        }

        // Load font and settings
        let font_data = app.font_path;
        let font = Font::try_from_bytes(font_data).expect("Error loading font");
        let scale = Scale::uniform(position.scale); // Font size
        let text = "text";

        // Calculate text dimensions
        let (text_width, text_height) = calculate_text_dimensions(&font, text, scale);

        // Adjust button logic
        let is_within_button = mouse_x >= position.x
            && mouse_x <= position.x + text_width
            && mouse_y >= position.y
            && mouse_y <= position.y + text_height;

        if left_down && is_within_button {
            single_line_text(
                &mut app,
                Position {
                    x: 80.0,
                    y: 80.0,
                    scale: 30.0,
                },
                "hello world!",
            );
        }

        // Highlight box logic
        let highlight_color = 0xFF0000; // Red color for the box
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

        // Rasterize and draw the text
        let start_point = point(position.x, position.y);
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

        set_button_position!(true);
        button!({
            println!("ds");
        })
    });
}
