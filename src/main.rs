use glow::*;
use kettlewin::*;

mod gl;
mod image;
mod log;
mod mesh;
mod shader;
use gl::*;
use image::*;
use log::*;
use mesh::*;
use shader::*;
#[allow(dead_code)]
mod zmath;

use zmath::*;

fn main() {
    console_error_panic_hook::set_once();

    let (app, mut event_loop) = initialize();
    event_loop.run_async(app, run);
}

async fn run(app: Application, mut events: Events) {
    let window = app.new_window().build().unwrap();
    let (mut gl_context, gl) = gl::setup(&window);
    //  let beach_image = load_image(&gl, "beach.jpg").await.unwrap();

    let vert = include_str!("shaders/vert.vs");
    let frag = include_str!("shaders/frag.fs");

    let shader_program = ShaderProgram::new(&gl, vert, frag);
    let quad = Mesh::new(
        &gl,
        &[
            ([0., 0., 0.], [0., 0.]),
            ([1., 0., 0.], [0., 0.]),
            ([1., 1., 0.], [0., 0.]),
            ([0., 1., 0.], [0., 0.]),
        ],
        &[[0, 1, 2], [0, 2, 3]],
    );

    loop {
        let event = events.next_event().await;
        match event {
            Event::WindowCloseRequested { .. } => app.quit(),
            Event::MouseButtonDown {
                button: MouseButton::Left,
                ..
            } => {
                log!("Mouse pressed");
            }
            Event::WindowResized { width, height, .. } => unsafe {
                gl.viewport(0, 0, width as i32, height as i32);
            },
            Event::Draw { .. } => {
                // Clear the screen.
                unsafe {
                    gl.clear_color(0.3765, 0.3137, 0.8627, 1.0);
                    gl.clear(COLOR_BUFFER_BIT | DEPTH_BUFFER_BIT);
                }
                shader_program.use_program(&gl);
                quad.draw(&gl);
                // Finally display what we've drawn.
                gl_context.swap_buffers();
                window.request_redraw();
            }
            _ => {}
        }
    }
}
