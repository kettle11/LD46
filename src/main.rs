use glow::*;
use kettlewin::*;

mod gl;
mod image;
mod lines;
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

use lines::*;
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
    let mut quad = Mesh::new(&gl);
    quad.update(
        &gl,
        &[
            Vector3::new(0., 0., 0.),
            Vector3::new(1., 0., 0.),
            Vector3::new(1., 1., 0.),
            Vector3::new(0., 1., 0.),
        ],
        &[[0, 1, 2], [0, 2, 3]],
    );
    let mut line_mesh = Mesh::new(&gl);

    let mut line_points = Vec::new();

    let mut screen_width = 0;
    let mut screen_height = 0;
    let mut last_position: Option<Vector2> = None;
    let mut mouse_down = false;
    let mut mouse_position = Vector2::new(0., 0.);

    let mut circle = Mesh::new(&gl);
    lines::update_mesh_with_circle(&gl, &mut circle, Vector3::ZERO, 0.2, 30);

    let mut projection = mat4_orthographic(-1.0, 1.0, -1.0, 1.0, 0.0, 1.0);
    let mut camera_view = Matrix4x4::IDENTITY;

    loop {
        let event = events.next_event().await;
        match event {
            Event::WindowCloseRequested { .. } => app.quit(),
            Event::MouseMoved { x, y, .. } => {
                // When the mouse check if the mouse drawing should be updated.
                mouse_position = Vector2::new(x, y);
                if mouse_down {
                    let mouse_pos = screen_to_gl(
                        mouse_position.x,
                        mouse_position.y,
                        screen_width,
                        screen_height,
                    );
                    if let Some(last_position_inner) = last_position {
                        if (last_position_inner - mouse_pos).length() > 0.0002 {
                            line_points.push(Vector3::new(
                                last_position_inner.x,
                                last_position_inner.y,
                                0.0,
                            ));
                            line_points.push(Vector3::new(mouse_pos.x, mouse_pos.y, 0.0));
                            lines::update_mesh_with_line(
                                &gl,
                                &mut line_mesh,
                                &line_points,
                                0.01,
                                Vector3::FORWARD,
                            );
                            last_position = Some(mouse_pos);
                        }
                    } else {
                        // Add a point
                        last_position = Some(mouse_pos);
                    }
                }
            }
            Event::MouseButtonDown {
                button: MouseButton::Left,
                x,
                y,
                ..
            } => {
                mouse_down = true;
                log!("Mouse pressed");
            }
            Event::MouseButtonUp {
                button: MouseButton::Left,
                x,
                y,
                ..
            } => {
                mouse_down = false;
                last_position = None;
            }
            Event::WindowResized { width, height, .. } => unsafe {
                screen_width = width;
                screen_height = height;
                gl.viewport(0, 0, width as i32, height as i32);
            },
            Event::Draw { .. } => {
                // Clear the screen.
                unsafe {
                    gl.clear_color(0.3765, 0.3137, 0.8627, 1.0);
                    gl.clear(COLOR_BUFFER_BIT | DEPTH_BUFFER_BIT);
                }

                shader_program.use_program(&gl);

                // Bind the camera's view and projection
                shader_program.set_matrix(&gl, "u_view", &camera_view);
                shader_program.set_matrix(&gl, "u_projection", &projection);

                // First render the line
                shader_program.set_matrix(&gl, "u_model", &Matrix4x4::IDENTITY);
                line_mesh.draw(&gl);

                // Then render the circle
                shader_program.set_matrix(&gl, "u_model", &Matrix4x4::IDENTITY);
                circle.draw(&gl);

                // Finally display what we've drawn.
                // Since we're using web this happens automatically, but on desktop this call is required.
                gl_context.swap_buffers();
                window.request_redraw();
            }
            _ => {}
        }
    }
}

fn screen_to_gl(x: f32, y: f32, window_width: u32, window_height: u32) -> Vector2 {
    Vector2::new(
        x / (window_width as f32),
        1.0 - (y / (window_height as f32)),
    ) * 2.0
        - Vector2::new(1.0, 1.0)
}
