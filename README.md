# Laststraw

MSRV Verison: Rust 1.78.0 

Ideomatic Simple GUI framework with only Minifb and Freetype-sys

## How to run example project
firstly this libary depends on the developer installing freetype2 

Installing SYSTEM freetype package for rust
- macos
  ``` 
  brew install freetype
  ```
- Linux
  ```
  sudo apt-get install libfreetype6-dev
  ```
- Windows
  Note you will likely need to use GNUWin32 to compile!
  Read: https://github.com/PistonDevelopers/freetype-sys?tab=readme-ov-file for more

Running the example 
  Cloning repo
```
git clone https://github.com/had2020/Last-Straw.git
```
Then run the example project, INSIDE the example-project folder
```
cargo run
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

- ASX! | proc macro

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


## Why
I took lots of inspiration from Rust Frameworks like Dioxus and Tauri Frameworks.
The main problem for my with these frameworks, is that desktop apps, acted like mini web browsers. 

This meant I had to tailer my code to not interact with the hardware, like a website, with static files. I just needed a framework that had buttons and multiline text to enter and display. Just something to make a basic text editer. 

I like the features of Iced, but I wanted a more light CPU, based framework, that is cross compatible. I also wanted easy to read variable names, as once someone learns them, they can just macrofi them.

## Key Table

## Color Table

## Example Project code, with all elements for a basic appilcation
