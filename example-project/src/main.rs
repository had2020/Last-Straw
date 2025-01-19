use laststraw::*;

fn main() {
    let mut app = App::new(500, 500, "test");

    asx!({
        set_window_color(&mut app, "Obsidian"); // is layored so this is back

        if input_pressed(&app, "esc") {
            app.should_close = true;
        }

        if input_pressed(&app, "space") {
            single_line_text(&mut app, position!(20.0, 20.0, 40.0), "You pressed space");
        }

        let lines: Vec<&str> = vec!["Apple fruit", "Banana", "Cherry pie", "Oreos"];
        multi_line_text(&mut app, position!(100.0, 100.0, 50.0), 50.0, lines);

        set_next_button(&mut app, position!(30.0, 30.0, 30.0)); // maybe wrap as struct
        set_next_button_text(&mut app, "helq");
        button!({
            single_line_text(
                &mut app,
                position!(20.0, 20.0, 40.0),
                "You clicked the button",
            );
        });

        let texty = editable_single_line(&mut app, position!(100.0, 50.0, 50.0), "sfsds", "Blue");
        single_line_text(&mut app, position!(20.0, 20.0, 40.0), &texty); // you can acess the value later, will be empty, never recived input

        limit_fps(&mut app, 60.0);
    });

    println!("app closed after window code.");
}
