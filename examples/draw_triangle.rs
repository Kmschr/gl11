extern crate glfw;
use self::glfw::{Action, Context, Key};

extern crate gl11;

use std::sync::mpsc::Receiver;

const SCR_WIDTH: u32 = 1024;
const SCR_HEIGHT: u32 = 768;

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

    gl11::load_with(|symbol| window.get_proc_address(symbol) as *const _);

    unsafe {
        gl11::BlendFunc(gl11::SRC_ALPHA, gl11::ONE_MINUS_SRC_ALPHA);
        gl11::ClearColor(0.0, 0.0, 0.0, 0.0);
    }

    while !window.should_close() {
        unsafe {
            gl11::Clear(gl11::COLOR_BUFFER_BIT);

            gl11::Begin(gl11::TRIANGLES);
            gl11::Color3f(1.0, 0.0, 0.0);
            gl11::Vertex2f(-0.8, -0.8);
            gl11::Color3f(0.0, 1.0, 0.0);
            gl11::Vertex2f(0.8, -0.8);
            gl11::Color3f(0.0, 0.0, 1.0);
            gl11::Vertex2f(0.0, 0.9);
            gl11::End();
        }

        process_events(&mut window, &events);
        window.swap_buffers();
        glfw.poll_events();
    }
}

fn process_events(window: &mut glfw::Window, events: &Receiver<(f64, glfw::WindowEvent)>) {
    for (_, event) in glfw::flush_messages(events) {
        match event {
            glfw::WindowEvent::FramebufferSize(width, height) => unsafe {
                gl11::Viewport(0, 0, width, height)
            },
            glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                window.set_should_close(true)
            }
            _ => {}
        }
    }
}
