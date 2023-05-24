extern crate glfw;
use self::glfw::{Action, Context, Key};

use std::sync::mpsc::Receiver;

const SCR_WIDTH: u32 = 800;
const SCR_HEIGHT: u32 = 600;

pub fn main() {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    glfw.window_hint(glfw::WindowHint::ContextVersion(1, 1));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(
        glfw::OpenGlProfileHint::Any,
    ));

    let (mut window, events) = glfw
        .create_window(SCR_WIDTH, SCR_HEIGHT, "Game", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window");

    window.make_current();
    window.set_key_polling(true);
    window.set_framebuffer_size_polling(true);

    let gl11 = gl11::Gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

    unsafe {
        gl11.Enable(gl11::TEXTURE_2D);
    }

    while !window.should_close() {
        process_events(&mut window, &events, &gl11);
        window.swap_buffers();
        glfw.poll_events();
    }
}

fn process_events(
    window: &mut glfw::Window,
    events: &Receiver<(f64, glfw::WindowEvent)>,
    gl11: &gl11::Gl,
) {
    for (_, event) in glfw::flush_messages(events) {
        match event {
            glfw::WindowEvent::FramebufferSize(width, height) => unsafe {
                gl11.Viewport(0, 0, width, height)
            },
            glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                window.set_should_close(true)
            }
            _ => {}
        }
    }
}
