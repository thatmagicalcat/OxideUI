use glfw::*;
use glow::HasContext;

const WIDTH: u32 = 800;
const HEIGHT: u32 = 800;

fn main() {
    let mut glfw = glfw::init(glfw::fail_on_errors).unwrap();

    glfw.window_hint(WindowHint::ContextVersionMinor(3));
    glfw.window_hint(WindowHint::ContextVersionMajor(3));
    glfw.window_hint(WindowHint::OpenGlProfile(OpenGlProfileHint::Core));

    let (mut window, events) = glfw
        .create_window(WIDTH, HEIGHT, "OxideUI", WindowMode::Windowed)
        .unwrap();

    window.make_current();

    let gl =
        unsafe { glow::Context::from_loader_function(|s| window.get_proc_address(s) as *const _) };

    println!("OpenGL version: {}", unsafe {
        gl.get_parameter_string(glow::VERSION)
    });

    while !window.should_close() {
        glfw.poll_events();
        glfw::flush_messages(&events).for_each(|(_, _event)| {});

        unsafe {
            gl.clear_color(0.2, 0.3, 0.3, 1.0);
            gl.clear(glow::COLOR_BUFFER_BIT);
        }

        window.swap_buffers();
    }
}
