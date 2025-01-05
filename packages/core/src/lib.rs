//!  comments item containing the comment

// cargo test
// doc gen cargo doc --open

/*  TODO add tests like this!
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
 */

// TODO consie tests maybe, or better documentation

// TODO at some point write some amazing documentation for everything

use minifb::{Key, Window, WindowOptions};

pub fn defined_window( resizeable: bool, width: usize, height: usize, name: &str) -> (Window, Vec<u32>) {
    // Initialize the pixel buffer
    let mut buffer: Vec<u32> = vec![0; width * height];

    // Create a window
    let mut window = Window::new(
        name,
        width,
        height,
        //WindowOptions::default(),
        WindowOptions {
            resize: resizeable,
            ..WindowOptions::default()
        },
    )
    .unwrap_or_else(|e| {
        panic!("Unable to open window: {}", e);
    });

    (window, buffer)
}

pub fn error_init() -> Glfw {
    use glfw::fail_on_errors;
    glfw::init(fail_on_errors!()).unwrap()
}

pub fn app_running(pwindow: PWindow) -> bool{ //TODO possiablely replaced by core macro
    if !pwindow.should_close() {
        true
    } else {
        false
    }
}

// for asx! macro remove it is never used due to borrow checker issues
pub fn input_handling(pwindow: PWindow, events: GlfwReceiver<(f64, WindowEvent)>, flow: Glfw) {
    let mut window = pwindow;
    let mut err_object = flow;
    err_object.poll_events();
    for (_, event) in glfw::flush_messages(&events) {
        println!("{:?}", event);
        match event { // asx! macro will be used to replace this
            glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => { // input esc // should be seperate function later
                window.set_should_close(true)
            },
            _ => {},
        }
    }
}

pub fn set_background_color(color: &str) {
    let (r, g, b, a) = match color {
        "red" => (1.0, 0.0, 0.0, 1.0),
        "green" => (0.0, 1.0, 0.0, 1.0),
        "blue" => (0.0, 0.0, 1.0, 1.0),
        "white" => (1.0, 1.0, 1.0, 1.0),
        "black" => (0.0, 0.0, 0.0, 1.0),
        "yellow" => (1.0, 1.0, 0.0, 1.0),
        "cyan" => (0.0, 1.0, 1.0, 1.0),
        "magenta" => (1.0, 0.0, 1.0, 1.0),
        "orange" => (1.0, 0.5, 0.0, 1.0),
        "purple" => (0.5, 0.0, 0.5, 1.0),
        "pink" => (1.0, 0.75, 0.8, 1.0),
        "brown" => (0.6, 0.3, 0.0, 1.0),
        "gray" => (0.5, 0.5, 0.5, 1.0),
        "light_gray" => (0.75, 0.75, 0.75, 1.0),
        "dark_gray" => (0.25, 0.25, 0.25, 1.0),
        "light_blue" => (0.68, 0.85, 0.9, 1.0),
        "dark_blue" => (0.0, 0.0, 0.55, 1.0),
        "light_green" => (0.56, 0.93, 0.56, 1.0),
        "dark_green" => (0.0, 0.39, 0.0, 1.0),
        _ => (0.0, 0.0, 0.0, 1.0), // default to black if color is not recognized
    };

    unsafe {
        gl::ClearColor(r, g, b, a);
    }
    clear_color_buffer();
}

pub fn clear_color_buffer() {
    unsafe {
        gl::Clear(gl::COLOR_BUFFER_BIT);
    }
}

pub fn set_custom_background_color(r: f32, g: f32, b: f32, a: f32) {
    unsafe {
        gl::ClearColor(r, g, b, a);
    }
    clear_color_buffer();
}

pub fn input_pressed(key: &str) -> bool {
    let input = std::fs::read("temp.tmp").unwrap_or_else(|_| {
        std::fs::File::create("temp.tmp").unwrap();
        Vec::new()
    });
    let input_str = String::from_utf8(input).unwrap_or_default();

    if let Err(e) = std::fs::write("temp.tmp", "") { // clear file to prevent too many loops of the same key
        eprintln!("Failed to write to file: {}", e);
    }

    input_str == key
}

// all allowed keys in asx are listed here for reabability
pub fn handle_key_event_asx(key: glfw::Key) {
    let key_str = match key {
        glfw::Key::Escape => "esc",
        glfw::Key::Q => "Q",
        glfw::Key::W => "W",
        glfw::Key::E => "E",
        glfw::Key::R => "R",
        glfw::Key::T => "T",
        glfw::Key::Y => "Y",
        glfw::Key::U => "U",
        glfw::Key::I => "I",
        glfw::Key::O => "O",
        glfw::Key::P => "P",
        glfw::Key::A => "A",
        glfw::Key::S => "S",
        glfw::Key::D => "D",
        glfw::Key::F => "F",
        glfw::Key::G => "G",
        glfw::Key::H => "H",
        glfw::Key::J => "J",
        glfw::Key::K => "K",
        glfw::Key::L => "L",
        glfw::Key::Z => "Z",
        glfw::Key::X => "X",
        glfw::Key::C => "C",
        glfw::Key::V => "V",
        glfw::Key::B => "B",
        glfw::Key::N => "N",
        glfw::Key::M => "M",
        glfw::Key::Space => "space",
        glfw::Key::Enter => "enter",
        glfw::Key::Tab => "tab",
        glfw::Key::Backspace => "backspace",
        glfw::Key::Left => "left_arrow",
        glfw::Key::Right => "right_arrow",
        glfw::Key::Up => "up_arrow",
        glfw::Key::Down => "down_arrow",
        glfw::Key::LeftShift => "left_shift",
        glfw::Key::RightShift => "right_shift",
        glfw::Key::LeftControl => "left_ctrl",
        glfw::Key::RightControl => "right_ctrl",
        glfw::Key::LeftAlt => "left_alt",
        glfw::Key::RightAlt => "right_alt",
        glfw::Key::F1 => "F1",
        glfw::Key::F2 => "F2",
        glfw::Key::F3 => "F3",
        glfw::Key::F4 => "F4",
        glfw::Key::F5 => "F5",
        glfw::Key::F6 => "F6",
        glfw::Key::F7 => "F7",
        glfw::Key::F8 => "F8",
        glfw::Key::F9 => "F9",
        glfw::Key::F10 => "F10",
        glfw::Key::F11 => "F11",
        glfw::Key::F12 => "F12",
        glfw::Key::Num0 => "0",
        glfw::Key::Num1 => "1",
        glfw::Key::Num2 => "2",
        glfw::Key::Num3 => "3",
        glfw::Key::Num4 => "4",
        glfw::Key::Num5 => "5",
        glfw::Key::Num6 => "6",
        glfw::Key::Num7 => "7",
        glfw::Key::Num8 => "8",
        glfw::Key::Num9 => "9",
        glfw::Key::Period => ".",
        glfw::Key::Comma => ",",
        glfw::Key::Semicolon => ";",
        glfw::Key::Apostrophe => "'",
        glfw::Key::Slash => "/",
        glfw::Key::Backslash => "\\",
        glfw::Key::LeftBracket => "[",
        glfw::Key::RightBracket => "]",
        glfw::Key::Minus => "-",
        glfw::Key::Equal => "=",
        _ => return,
    };

    if let Err(e) = std::fs::write("temp.tmp", key_str.as_bytes()) {
        eprintln!("Failed to write to file: {}", e);
    }
}

//TODO
pub fn handle_mouse_button_event_asx(button: glfw::MouseButton) {
    let button_str = match button {
        glfw::MouseButton::Button1 => "mouse_button1",
        glfw::MouseButton::Button2 => "mouse_button2",
        glfw::MouseButton::Button3 => "mouse_button3",
        glfw::MouseButton::Button4 => "mouse_button4",
        glfw::MouseButton::Button5 => "mouse_button5",
        glfw::MouseButton::Button6 => "mouse_button6",
        glfw::MouseButton::Button7 => "mouse_button7",
        glfw::MouseButton::Button8 => "mouse_button8",
        _ => return,
    };

    if let Err(e) = std::fs::write("temp.tmp2", button_str.as_bytes()) {
        eprintln!("Failed to write to file: {}", e);
    }
}

//TODO
pub fn handle_mouse_scroll_event_asx(scroll: f64) {
    let scroll_str = scroll.to_string();

    if let Err(e) = std::fs::write("temp.tmp3", scroll_str.as_bytes()) {
        eprintln!("Failed to write to file: {}", e);
    }
}

/* 
// character rendering 
const FONT_BYTES: &[u8] = include_bytes!("../assets/fonts/FiraSans-Regular.ttf"); // always having font loaded in package

extern crate freetype as ft;
use std::collections::VecDeque;

use gl::types::*;
use std::ptr;
use std::mem;
use std::os::raw::c_void;

#[derive(Debug)]
enum CurvePoint {
    Line(f64, f64),
    Bezier2((f64, f64), (f64, f64)),
    Bezier3((f64, f64), (f64, f64), (f64, f64)),
}

pub fn create_vao(all_vertices: &[Vec<CurvePoint>]) -> GLuint {
    // Flatten the vertex data
    let mut flat_vertices = Vec::new();
    for contour in all_vertices {
        for vertex in contour {
            match vertex {
                CurvePoint::Line(x, y) => flat_vertices.extend_from_slice(&[*x, *y]),
                CurvePoint::Bezier2(p1, p2) => {
                    flat_vertices.extend_from_slice(&[p1.0, p1.1, p2.0, p2.1])
                }
                CurvePoint::Bezier3(p1, p2, p3) => {
                    flat_vertices.extend_from_slice(&[p1.0, p1.1, p2.0, p2.1, p3.0, p3.1])
                }
            }
        }
    }

    let mut vao: GLuint = 0;
    let mut vbo: GLuint = 0;

    unsafe {
        // Generate and bind the VAO
        gl::GenVertexArrays(1, &mut vao);
        gl::BindVertexArray(vao);

        // Generate and bind the VBO
        gl::GenBuffers(1, &mut vbo);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);

        // Upload the vertex data to the GPU
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (flat_vertices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
            flat_vertices.as_ptr() as *const c_void,
            gl::STATIC_DRAW,
        );

        // Define the vertex attributes (e.g., 2D positions)
        let stride = 2 * mem::size_of::<GLfloat>() as GLsizei;
        gl::VertexAttribPointer(
            0,                  // Attribute index
            2,                  // Number of components per vertex
            gl::FLOAT,          // Data type
            gl::FALSE,          // Normalized?
            stride,             // Stride
            ptr::null(),        // Offset
        );
        gl::EnableVertexAttribArray(0);

        // Unbind the VAO and VBO for safety
        //gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        //gl::BindVertexArray(0);
    }

    println!("All vertices collected:");
    for (i, contour) in all_vertices.iter().enumerate() {
        println!("Contour {}: {:?}", i, contour);
    }

    vao
}


pub fn draw_curve(curve: ft::outline::Curve, vertices: &mut Vec<CurvePoint>) {
    match curve {
        ft::outline::Curve::Line(pt) => {
            vertices.push(CurvePoint::Line(pt.x as f64, -pt.y as f64));
        }
        ft::outline::Curve::Bezier2(pt1, pt2) => {
            vertices.push(CurvePoint::Bezier2(
                (pt1.x as f64, -pt1.y as f64),
                (pt2.x as f64, -pt2.y as f64),
            ));
        }
        ft::outline::Curve::Bezier3(pt1, pt2, pt3) => {
            vertices.push(CurvePoint::Bezier3(
                (pt1.x as f64, -pt1.y as f64),
                (pt2.x as f64, -pt2.y as f64),
                (pt3.x as f64, -pt3.y as f64),
            ));
        }
    }
}

pub fn render(vao: GLuint, count: i32) {
    unsafe {
        gl::BindVertexArray(vao);
        gl::DrawArrays(gl::LINE_STRIP, 0, count);
        gl::BindVertexArray(0);
    }
}

pub fn compile_shader(src: &str, ty: GLenum) -> GLuint {
    let shader = unsafe { gl::CreateShader(ty) };
    unsafe {
        gl::ShaderSource(
            shader,
            1,
            &(src.as_ptr() as *const i8),
            &(src.len() as i32),
        );
        gl::CompileShader(shader);
    }

    // Check for compilation errors
    let mut success = gl::FALSE as GLint;
    unsafe { gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success) };
    if success != gl::TRUE as GLint {
        let mut len = 0;
        unsafe { gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut len) };
        let mut buffer = Vec::with_capacity(len as usize);
        unsafe {
        buffer.set_len((len as usize) - 1); // Skip null terminator
        }
        unsafe {
            gl::GetShaderInfoLog(
                shader,
                len,
                ptr::null_mut(),
                buffer.as_mut_ptr() as *mut i8,
            )
        };
        panic!(
            "Shader compilation failed: {}",
            std::str::from_utf8(&buffer).unwrap()
        );
    }
    shader
}

pub fn link_program(vertex_shader: GLuint, fragment_shader: GLuint) -> GLuint {
    let program = unsafe { gl::CreateProgram() };
    unsafe {
        gl::AttachShader(program, vertex_shader);
        gl::AttachShader(program, fragment_shader);
        gl::LinkProgram(program);
    }

    // Check for linking errors
    let mut success = gl::FALSE as GLint;
    unsafe { gl::GetProgramiv(program, gl::LINK_STATUS, &mut success) };
    if success != gl::TRUE as GLint {
        let mut len = 0;
        unsafe { gl::GetProgramiv(program, gl::INFO_LOG_LENGTH, &mut len) };
        let mut buffer = Vec::with_capacity(len as usize);
        unsafe {
        buffer.set_len((len as usize) - 1); // Skip null terminator
        }
        unsafe {
            gl::GetProgramInfoLog(
                program,
                len,
                ptr::null_mut(),
                buffer.as_mut_ptr() as *mut i8,
            )
        };
        panic!(
            "Program linking failed: {}",
            std::str::from_utf8(&buffer).unwrap()
        );
    }
    program
}

pub fn text(character: char) {

    let font = "../assets/fonts/FiraSans-Regular.ttf";
    let library = ft::Library::init().unwrap();
    //let face = library.new_face(font, 0).unwrap();
    let face = library // should load path from emmbeded bytes
        .new_memory_face(FONT_BYTES.to_vec(), 0)
        .expect("Failed to load font");

    face.set_char_size(40 * 64, 0, 50, 0).unwrap();
    face.load_char(character as usize, ft::face::LoadFlag::NO_SCALE)
        .unwrap();

    let glyph = face.glyph();
    let outline = glyph.outline().unwrap();

    // store all collected vertices
    let mut all_vertices: VecDeque<Vec<CurvePoint>> = VecDeque::new();

    for contour in outline.contours_iter() {
        let mut contour_vertices: Vec<CurvePoint> = Vec::new();
        let start = contour.start();
        contour_vertices.push(CurvePoint::Line(start.x as f64, -start.y as f64));
        for curve in contour {
            draw_curve(curve, &mut contour_vertices);
        }
        all_vertices.push_back(contour_vertices);
    }

    // debug collected vertices
    /* 
    for (i, contour) in all_vertices.iter().enumerate() {
        println!("Contour {}:", i);
        for vertex in contour {
            match vertex {
                CurvePoint::Line(x, y) => println!("Line to: ({}, {})", x, y),
                CurvePoint::Bezier2(p1, p2) => println!(
                    "Quadratic Bezier to: Control ({}, {}), End ({}, {})",
                    p1.0, p1.1, p2.0, p2.1
                ),
                CurvePoint::Bezier3(p1, p2, p3) => println!(
                    "Cubic Bezier to: Control1 ({}, {}), Control2 ({}, {}), End ({}, {})",
                    p1.0, p1.1, p2.0, p2.1, p3.0, p3.1
                ),
            }
        }
    }
    */

    let version = unsafe {
        std::ffi::CStr::from_ptr(gl::GetString(gl::VERSION) as *const i8)
            .to_str()
            .unwrap()
    };
    
    // Extract the major and minor version numbers
    let mut major_version = 0;
    let mut minor_version = 0;
    
    if let Some(version_match) = version.split('.').collect::<Vec<&str>>().get(0..2) {
        major_version = version_match[0].parse().unwrap_or(0);
        minor_version = version_match[1].parse().unwrap_or(0);
    }
    
    // Declare shader sources
    let (vertex_shader_src, fragment_shader_src);
    
    if major_version > 3 || (major_version == 3 && minor_version >= 3) {
        // GLSL 330 core shaders
        vertex_shader_src = r#"
        #version 330 core
        layout(location = 0) in vec2 aPos;
        void main() {
            gl_Position = vec4(aPos, 0.0, 1.0);
        }
        "#;
    
        fragment_shader_src = r#"
        #version 330 core
        out vec4 FragColor;
        void main() {
            FragColor = vec4(1.0, 1.0, 1.0, 1.0); // White
        }
        "#;
    } else if major_version == 2 || (major_version == 1 && minor_version >= 2) {
        // GLSL 120 shaders
        vertex_shader_src = r#"
        #version 120
        attribute vec2 aPos;
        void main() {
            gl_Position = vec4(aPos, 0.0, 1.0);
        }
        "#;
    
        fragment_shader_src = r#"
        #version 120
        void main() {
            gl_FragColor = vec4(1.0, 1.0, 1.0, 1.0);
        }
        "#;
    } else {
        panic!("Unsupported OpenGL version: {}.{}", major_version, minor_version);
    }
    

    let vertex_shader = compile_shader(vertex_shader_src, gl::VERTEX_SHADER);
    let fragment_shader = compile_shader(fragment_shader_src, gl::FRAGMENT_SHADER);
    let shader_program = link_program(vertex_shader, fragment_shader);

    let error_code = unsafe { gl::GetError() };
    if error_code != gl::NO_ERROR {
        eprintln!("OpenGL Error: {}", error_code);
    }

    // Use the shader program
    unsafe {
        gl::UseProgram(shader_program);
    }

    let vertex_count: usize = all_vertices.iter().map(|contour| contour.len()).sum();
    let all_vertices_vec: Vec<_> = all_vertices.into_iter().collect();
    let vao = create_vao(&all_vertices_vec);
        
    //render(vao, 2);   
    //let vertex_count: usize = all_vertices.iter().map(|contour| contour.len()).sum();
    render(vao, vertex_count as i32);

}

*/