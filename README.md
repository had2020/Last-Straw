# Laststraw

![MSRV](https://img.shields.io/badge/Rust%20MSRV-1.78.0-brightgreen)
[![crates.io](https://img.shields.io/crates/v/laststraw.svg)](https://crates.io/crates/laststraw/0.1.0)
[![docs.rs](https://docs.rs/laststraw/badge.svg)](https://docs.rs/laststraw)
[![Downloads](https://img.shields.io/crates/d/laststraw.svg)](https://crates.io/crates/laststraw)



Ideomatic Simple GUI framework with only Minifb and Freetype-sys

# See Github for Up To Date Docs
Enjoyed the project? Leaving a Star means the world ⭐️
- https://github.com/had2020/Last-Straw

## How to run example project
firstly this libary depends on the developer installing freetype2

Installing C library freetype in package manager for 🦀 Rust.
- $ macos 🍎
  ```
  brew install freetype
  ```
- $ Linux 🐧
  ```
  sudo apt-get install libfreetype6-dev
  ```
- $ Windows 🪟

  📝 Note: you will likely need to use GNUWin32 to compile, or someother package manager like Choco!
  Read: https://github.com/PistonDevelopers/freetype-sys?tab=readme-ov-file for more

  Or with the Choco package manager.
  ``` bash
  choco install freetype
  ```

Installing the Crate.

``` bash
cargo add laststraw
```

``` rust
use laststraw::*;

fn main() {
    let mut app = App::new(500, 500, "test"); // app variable, must be only a single one named app, which is mutable.

    asx!({ // runs every frame while, app.should_close == false.
        single_line_text(&mut app, position!(20.0, 20.0, 40.0), "Hello"); // shows Hello on the screen

        set_next_button(&mut app, position!(30.0, 30.0, 30.0)); // this code is layer dependent
        set_next_button_text(&mut app, "helq");
        button!({
            single_line_text(
                &mut app,
                position!(20.0, 20.0, 40.0),
                "You clicked the button",
            );
            println!("Button press logged!");
        });
    });

    println!("app closed after window code."); // last line of code, like in a normal rust program, EVEN if the user exited with exit button.
    // FAILS only if force exit by task manager alike program.
}

```
Make sure to add dependencies in Cargo.toml
``` bash
cargo add freetype-rs@0.37.0
cargo add minifb@0.27.0
cargo add rusttype@0.9.3
cargo add quote@1.0.38
cargo add syn@2.0.92 --features full
```

## How it works
# Main Parts
- App | stuct

firstly, I use the App stuct to hold all of are current app window's infomation, i.e, size height, and some more that were needed from the minifb framework.
A variable must be set with the name app and the impl of new. Just like the code seen below.
``` rust
let mut app = App::new(500, 500, "test");
```
1. The First argument is window height.
2. The second argument is window width.
3. The last argument is an &str of name of the program and window.

- asx! | proc macro

asx is similar in dioxus and react. Every frame it will print "app_loop". Code here is looped, here is where you can add UI elements like single_line_text();
``` rust
asx!({ println!("app_loop") })
```
Any element must be declared inside the asx!

# How is GUI layered
Gui elements are layered as they are declared, the lowest is at the top.
For example multi_line_text(), would be furtherest to the back.
``` rust
asx!({
  set_window_color(&mut app, "Obsidian"); // undermost layer

  let lines: Vec<&str> = vec!["Apple fruit", "Banana", "Cherry pie", "Oreos"];
  multi_line_text(&mut app, position!(100.0, 100.0, 50.0), 50.0, lines); // topmost layer
})
```

# Position Struct
used for setting location of a element on the screen.
Made up of X: Height, Y: Weight, and Scale.

You can make one the position macro as seen below, or just write out the stuct for readablity.
``` rust
position!(20.0, 20.0, 40.0)
```
OR
``` rust
Position { x: 10.0, y: 10.0, scale: 50.0, }
```
for readablity

# Text Elements
if app variables are updated, then we need to own a mut borrow, &mut app.
Other's only borrow ownership like checking for input_pressed().

- single_line_text

1. First argument is a declared App stuct.
2. The second argument is a stuct called position, it is made up of X: Height, Y: Weight, and Scale, deciding were the ui is placed.
3. The third argument is a &str, with the message, displayed on the string.
   This does not take user input, but only displays some text.
``` rust
single_line_text(&mut app, position!(20.0, 20.0, 40.0), "You pressed space");
```

- multi_line_text

1. First argument is a declared App stuct.
2. The second argument is a stuct called position, it is made up of X: Height, Y: Weight, and Scale, deciding were the ui is placed.
3. The third argument, are the lines of text, each index a line, stored in a Vec<&str>

``` rust
let lines: Vec<&str> = vec!["Apple fruit", "Banana", "Cherry pie", "Oreos"]; // every new line is a index
multi_line_text(&mut app, position!(100.0, 100.0, 50.0), 50.0, lines); // lines needs index in a Vec<&str>
```

# Button Element
Proc Macro which only runs the code inside one clicked.
Similar to asx!
``` rust
button!({
  println!("button is pressed");
});
```

# Interactable text
1. First argument is the declared App stuct, from outside asx.
2. Second is a macro declaring a position struct, Hight, Width, Scale.
3. The text seen before any text is enter by user.
4. Color from color table, refered to at end of docs, or laststraw_core.
5. If the element would only accept single line, false, means single line only.

``` rust
let texty: String = editable_lines(
    &mut app, // mut app stuct variable created by new impl on App.
    position!(100.0, 50.0, 50.0), // position struct created by macro.
    "enter:", // preluding text.
    "Blue", // color of the boarder.
    false, // if you wish for only single line input.
);
```

# Setting the initial text in following/next Interactable text
- for setting multi_lines
``` rust
let lines = vec!["one", "two", "three"]; // example &str list, each line is an index
following_input_initial_lines(&mut app, lines);
```
- for setting a single line
``` rust
following_input_initial_text(&mut app, "just a line here"); // you can use both for just a line, but this is easier
```

# Changing initial Interactable text
place before a Interactable text element to set initial text,
which stays after clicking the element.
``` rust
pub fn set_next_input_init_text(app: &mut App, init_text: &str) {
    if app.already_set_initial_text == true {
        app.multi_line_storing[app.current_text_edit_id][1] = init_text.to_string();
    }
}
```

#  Input checking
1. First argument is a declared App stuct.
2. Second is the key see input table at the end of the docs or laststraw_core.
3. Any code inside the if state behaves as expected, as the function returns true or false based off the if the key is pressed.

``` rust
  if input_pressed(&app, "space") {
      single_line_text(&mut app, position!(20.0, 20.0, 40.0), "Hello");
  }
```

# Changing the Window color
TIP make sure this is the futherest back, because it writes over the whole screen buffer.
Sets the color of the background.
``` rust
set_window_color(&mut app, "Obsidian");
```

# Limit fps to reduce CPU usage if needed
1. First argument is the declared App stuct, from outside asx.
2. Second argument is the max frames per a second.

Note: this is optional, and used to reduce CPU usage for much heavy apps, with many elements.
``` rust
limit_fps(&mut app, 60.0); // a nice 60 frames per a second, 30-60 is a good number.
```

# Closing app
- This framework uses a lot of abstraction from minifb and freetype.
- So to simply close the app, set app.should_close to true.
- This is becasue, asx is a avarge while loop like seen in many window frameworks like minifb: in this project, and Glfw.
```
  app.should_close = true; // closes app at line.
```

# Structuring, Base Plate
This is a example of a base plate for a app, in this framework.
``` rust
use laststraw::*;

fn main() {
    let mut app = App::new(500, 500, "test"); // app variable, must be only a single one named app, which is mutable.

    asx!({ // runs every frame while, app.should_close == false.
        single_line_text(&mut app, position!(20.0, 20.0, 40.0), "Hello"); // shows Hello on the screen

        if input_pressed(&app, "esc") { // if the esc key is pressed the inside code runs.
            app.should_close = true; // stops the asx loop to end.
        }
    });

    println!("app closed after window code."); // last line of code, like in a normal rust program, EVEN if the user exited with exit button.
    // FAILS only if force exit by task manager alike program.
}

```

## 🔑 Key Table
| 🔣 Symbol       | 🖮 Key Mapping        |
|-----------------|-----------------------|
| ⎋ esc           | `Key::Escape`         |
| 1️⃣ 1            | `Key::Key1`           |
| 2️⃣ 2            | `Key::Key2`           |
| 3️⃣ 3            | `Key::Key3`           |
| 4️⃣ 4            | `Key::Key4`           |
| 5️⃣ 5            | `Key::Key5`           |
| 6️⃣ 6            | `Key::Key6`           |
| 7️⃣ 7            | `Key::Key7`           |
| 8️⃣ 8            | `Key::Key8`           |
| 9️⃣ 9            | `Key::Key9`           |
| 0️⃣ 0            | `Key::Key0`           |
| 🅰️ a             | `Key::A`              |
| 🅱️ b             | `Key::B`              |
| 🇨 c             | `Key::C`              |
| 🇩 d             | `Key::D`              |
| 🇪 e             | `Key::E`              |
| 🇫 f             | `Key::F`              |
| 🇬 g             | `Key::G`              |
| 🇭 h             | `Key::H`              |
| 🇮 i             | `Key::I`              |
| 🇯 j             | `Key::J`              |
| 🇰 k             | `Key::K`              |
| 🇱 l             | `Key::L`              |
| 🇲 m             | `Key::M`              |
| 🇳 n             | `Key::N`              |
| 🇴 o             | `Key::O`              |
| 🅿️ p             | `Key::P`              |
| 🇶 q             | `Key::Q`              |
| 🇷 r             | `Key::R`              |
| 🇸 s             | `Key::S`              |
| 🇹 t             | `Key::T`              |
| 🇺 u             | `Key::U`              |
| 🇻 v             | `Key::V`              |
| 🇼 w             | `Key::W`              |
| ❎ x             | `Key::X`              |
| 🇾 y             | `Key::Y`              |
| 🇿 z             | `Key::Z`              |
| ␣ space         | `Key::Space`          |
| ↵ enter         | `Key::Enter`          |
| ⇥ tab           | `Key::Tab`            |
| ⌫ backspace     | `Key::Backspace`      |
| ◀️ left           | `Key::Left`           |
| ▶️ right          | `Key::Right`          |
| 🔼 up            | `Key::Up`             |
| 🔽 down          | `Key::Down`           |
| ⇧ left_shift     | `Key::LeftShift`      |
| ⇧ right_shift    | `Key::RightShift`     |
| ⌃ left_ctrl      | `Key::LeftCtrl`       |
| ⌃ right_ctrl     | `Key::RightCtrl`      |
| ⎇ left_alt       | `Key::LeftAlt`        |
| ⎇ right_alt      | `Key::RightAlt`       |
| 🔢 ;             | `Key::Semicolon`      |
| 🖋️ '              | `Key::Apostrophe`     |
| 🥒 ,              | `Key::Comma`          |
| .               | `Key::Period`         |
| ➗ /              | `Key::Slash`          |
| ⬛ backslash     | `Key::Backslash`      |
| 🗂️ left_bracket  | `Key::LeftBracket`    |
| 📂 right_bracket | `Key::RightBracket`   |
| ➖ -              | `Key::Minus`          |
| ➕ +              | `Key::Equal`          |
| 🔒 caps_lock     | `Key::CapsLock`       |
| 🛑 scroll_lock   | `Key::ScrollLock`     |
| 🔢 num_lock      | `Key::NumLock`        |
| ⏸️ pause         | `Key::Pause`          |
| 🖋️ insert        | `Key::Insert`         |
| 🏠 home          | `Key::Home`           |
| 📄 page_up       | `Key::PageUp`         |
| 🗑️ delete         | `Key::Delete`         |
| 🏁 end           | `Key::End`            |
| 📉 page_down     | `Key::PageDown`       |
| 🔈 f1            | `Key::F1`             |
| 🎛️ f2            | `Key::F2`             |
| 🔢 f3            | `Key::F3`             |
| 🔢 f4            | `Key::F4`             |
| 🔢 f5            | `Key::F5`             |
| 🔢 f6            | `Key::F6`             |
| 🔢 f7            | `Key::F7`             |
| 🔢 f8            | `Key::F8`             |
| 🔢 f9            | `Key::F9`             |
| 🔢 f10           | `Key::F10`            |
| 🔢 f11           | `Key::F11`            |
| 🔢 f12           | `Key::F12`            |


## 🎨 **Color Table**

| **Color Name**        | **Hex Code**      |
|-----------------------|--------------------|
| Green 🟢               | `0xFF00FF00`       |
| Red 🔴                 | `0xFFFF0000`       |
| Blue 🔵                | `0xFF0000FF`       |
| Yellow 🟡              | `0xFFFFFF00`       |
| Cyan 🟦                | `0xFF00FFFF`       |
| Magenta 💖            | `0xFFFF00FF`       |
| White ⚪               | `0xFFFFFFFF`       |
| Black ⚫               | `0xFF000000`       |
| Gray 🩶                | `0xFF808080`       |
| Orange 🟠              | `0xFFFFA500`       |
| Purple 🟣              | `0xFF800080`       |
| Pink 🌸                | `0xFFFFC0CB`       |
| Brown 🟤               | `0xFFA52A2A`       |
| Light Gray 🪶          | `0xFFD3D3D3`       |
| Light Blue 🌊          | `0xFFADD8E6`       |
| Dark Blue 🏴‍☠️         | `0xFF00008B`       |
| Beige 🤎               | `0xFFF5F5DC`       |
| Teal 🐠                | `0xFF008080`       |
| Lavender 🌾            | `0xFFE6E6FA`       |
| Ivory 🏛️              | `0xFFFFFFF0`       |
| Mint 🌿                | `0xFF98FF98`       |
| Coral 🐚               | `0xFFFF7F50`       |
| Navy ⚓                | `0xFF000080`       |
| Sky Blue ☁️           | `0xFF87CEEB`       |
| Sea Green 🌊           | `0xFF2E8B57`       |
| Forest Green 🌲        | `0xFF228B22`       |
| Dark Gray 🖤           | `0xFFA9A9A9`       |
| Slate Gray 🏔️         | `0xFF708090`       |
| Charcoal 🏭            | `0xFF36454F`       |
| Jet Black 🕶️          | `0xFF343434`       |
| Gunmetal 🔫            | `0xFF2A3439`       |
| Dark Slate Blue 🌌     | `0xFF483D8B`       |
| Midnight Blue 🌃       | `0xFF191970`       |
| Deep Navy 🛳️          | `0xFF1B1F3B`       |
| Dark Olive Green 🌿    | `0xFF556B2F`       |
| Deep Forest Green 🌳   | `0xFF1A2E1A`       |
| Maroon 🍷              | `0xFF800000`       |
| Deep Burgundy 🍇       | `0xFF4A0000`       |
| Dark Chocolate 🍫      | `0xFF3E2723`       |
| Dark Copper 🏺         | `0xFF4E3629`       |
| Onyx 💎               | `0xFF353839`       |
| Obsidian 🌋           | `0xFF1C1C1C`       |
| **Default (Invalid)**  | `0xFFFFC0CB`       |




## Example project, Element showcase
This app code showcases all the elements in use, you can refer to it when using this framework.
``` rust
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

        set_next_button(&mut app, position!(30.0, 30.0, 30.0));
        set_next_button_text(&mut app, "helq");
        button!({
            single_line_text(
                &mut app,
                position!(20.0, 20.0, 40.0),
                "You clicked the button",
            );
        });

        let texty = editable_lines(
            &mut app,
            position!(100.0, 50.0, 50.0),
            "enter:",
            "Blue",
            false,
        );
        single_line_text(&mut app, position!(20.0, 20.0, 40.0), &texty); // you can acess the value later, will be empty, never recived input

        limit_fps(&mut app, 60.0);
    });

    println!("app closed after window code.");
}
```

## Why / Project statement 📝
I took lots of inspiration from Rust Frameworks like Dioxus and Tauri Frameworks.
The main problem for my with these frameworks, is that desktop apps, acted like mini web browsers.

This meant I had to tailer my code to not interact with the hardware, like a website, with static files. I just needed a framework that had buttons and multiline text to enter and display. Just something to make a basic text editer.

I like the features of Iced, but I wanted a more light CPU, based framework. That could also be cross compatible. I also wanted easy to read variable names, as once someone learns them, they can just macrofi them.

I also wished it to be easyer for people new to Rust to be able to make Apps.
As I belive people like projects they can see rendered on their screen more.

Hopfully, this project makes some dev happy to keep their Rust app low level.

## Contributing 💪
Since, this is my first published crate framework, I expect their might be bugs.
Feel free to open an issue or anything, if you belive you can do the code better.
I would love to learn any new ideas, to make this framework better!
