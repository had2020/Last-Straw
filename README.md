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

#  Button Element
1. First argument is a declared App stuct.
2. Second is the key see input table at the buttom of the docs or laststraw_core.
3. Any code inside the if state behaves as expected, as the function returns true or false based off the if the key is pressed.

``` rust
  if input_pressed(&app, "space") {
      single_line_text(&mut app, position!(20.0, 20.0, 40.0), "You pressed space");
  }
```

# 

## Why
I took lots of inspiration from Rust Frameworks like Dioxus and Tauri Frameworks.
The main problem for my with these frameworks, is that desktop apps, acted like mini web browsers. 

This meant I had to tailer my code to not interact with the hardware, like a website, with static files. I just needed a framework that had buttons and multiline text to enter and display. Just something to make a basic text editer. 

I like the features of Iced, but I wanted a more light CPU, based framework, that is cross compatible. I also wanted easy to read variable names, as once someone learns them, they can just macrofi them.

## Key Table

## Color Table

## Example Project code, with all elements for a basic appilcation
