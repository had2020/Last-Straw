# Laststraw

Simple GUI framework with only Minifb and Freetype-sys

# How to run example project
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

# Why
I took lots of inspiration from Rust Frameworks like Dioxus and Tauri Frameworks.
The main problem for my with these frameworks, is that desktop apps, acted like
mini web browsers. This meant I had to tailer my code to not interact with the hardware, like a website, with static files. I just needed a framework that had buttons and multiline text to enter and display. Just something to make a basic text editer. I like the features of Iced, but I wanted a more light CPU, based framework, that is cross compatible. 

# How it works
- App stuct 
firstly, I use the App stuct to hold all of are current app window's infomation, I.e, size height, and some more that were needed from the minifb framework.
```
```
