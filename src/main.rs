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
    #[cfg(target_arch = "wasm32")]
    console_error_panic_hook::set_once();

    let (app, mut event_loop) = initialize();
    event_loop.run_async(app, run);
}

struct Color {
    r: f32,
    g: f32,
    b: f32,
    a: f32,
}

impl Color {
    pub fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }
}
struct Ball {
    position: Vector3,
    velocity: Vector3,
    radius: f32,
    color: Color,
    intersection_position: Vector3,
}

struct Camera {
    projection: Matrix4x4,
    view: Matrix4x4,
    inverse_projection: Matrix4x4,
    inverse_view: Matrix4x4,
}

impl Camera {
    pub fn new(projection: Matrix4x4, view: Matrix4x4) -> Self {
        Self {
            projection,
            inverse_projection: projection.inverse(),
            view,
            inverse_view: view.inverse(),
        }
    }

    pub fn set_projection(&mut self, projection: Matrix4x4) {
        self.projection = projection;
        self.inverse_projection = self.projection.inverse();
    }

    pub fn set_view(&mut self, view: Matrix4x4) {
        self.view = view;
        self.inverse_view = self.view.inverse();
    }
}

const LINE_RADIUS: f32 = 0.01;
impl Ball {
    // Every two Vector3s in points is a line segment
    fn ball_physics(&mut self, points: &[Vector3]) {
        let len = points.len();
        self.color = Color::new(1.0, 1.0, 1.0, 1.0);
        self.velocity += Vector3::DOWN * 0.0001;
        for i in (1..len).step_by(2) {
            let (distance, p) = point_with_line_segment(self.position, points[i - 1], points[i]);

            if distance
                < (self.radius + LINE_RADIUS - 0.001/* Allow ball to sink slightly into surface*/)
            {
                let normal_of_collision = (self.position - p).normal();
                let velocity_along_collision = Vector3::dot(normal_of_collision, self.velocity);
                self.velocity -= normal_of_collision * velocity_along_collision;
                self.position += normal_of_collision * 0.0001;
            }
        }
        self.position += self.velocity;
    }
}

// Returns magnitude of distance and the point
fn point_with_line_segment(p: Vector3, a: Vector3, b: Vector3) -> (f32, Vector3) {
    let ba = b - a;
    let pa = p - a;
    let h = (Vector3::dot(ba, pa) / Vector3::dot(ba, ba))
        .max(0.0)
        .min(1.0);
    let position = a + (ba * h);
    ((p - position).length(), position)
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

    // line_points.push(Vector3::new(0.0, -0.3, 0.0));
    // line_points.push(Vector3::new(0.5, -0.3, 0.0));

    lines::update_mesh_with_line(
        &gl,
        &mut line_mesh,
        &line_points,
        LINE_RADIUS,
        Vector3::FORWARD,
    );
    let mut screen_width = 0;
    let mut screen_height = 0;
    let mut last_position: Option<Vector3> = None;
    let mut mouse_down = false;
    let mut mouse_position = Vector2::new(0., 0.);

    let start_position = Vector3::new(0.0, 1.6, 0.0);
    let mut ball = Ball {
        position: start_position,
        velocity: Vector3::ZERO,
        radius: 0.06,
        color: Color::new(1.0, 1.0, 1.0, 1.0),
        intersection_position: Vector3::ZERO,
    };

    let mut circle = Mesh::new(&gl);
    lines::update_mesh_with_circle(&gl, &mut circle, Vector3::ZERO, 1.0, 30);

    let mut camera = Camera::new(
        mat4_orthographic(-1.0, 1.0, -1.0, 1.0, 0.0, 1.0),
        Matrix4x4::IDENTITY,
    );

    // Camera shifts everything down and to the left to make 0,0 bottom left.
    // 2.0, 2.0 is upper right
    camera.set_view(mat4_from_trs(
        Vector3::new(-1.0, -1.0, 0.0),
        Quaternion::IDENTITY,
        Vector3::new_uniform(1.0),
    ));

    let white = Color::new(1.0, 1.0, 1.0, 1.0);

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
                        &camera,
                        screen_width,
                        screen_height,
                    );
                    if let Some(last_position_inner) = last_position {
                        if (last_position_inner - mouse_pos).length() > 0.01 {
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
                                LINE_RADIUS,
                                Vector3::FORWARD,
                            );
                            last_position = Some(mouse_pos);
                        }
                    } else {
                        // Add a point
                        last_position = Some(mouse_pos);
                    }
                }

                // Debug test of line intersection
                let mouse_pos = screen_to_gl(
                    mouse_position.x,
                    mouse_position.y,
                    &camera,
                    screen_width,
                    screen_height,
                );
                //ball.position = Vector3::new(mouse_pos.x, mouse_pos.y, 0.0);
            }
            Event::MouseButtonDown {
                button: MouseButton::Left,
                x,
                y,
                ..
            } => {
                mouse_down = true;
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
            Event::KeyDown { key: Key::E, .. } => {
                line_points.clear();
                lines::update_mesh_with_line(
                    &gl,
                    &mut line_mesh,
                    &line_points,
                    LINE_RADIUS,
                    Vector3::FORWARD,
                );
            }
            Event::KeyDown { key: Key::R, .. } => {
                ball.position = start_position;
                ball.velocity = Vector3::ZERO;
            }
            Event::WindowResized { width, height, .. } => unsafe {
                screen_width = width;
                screen_height = height;
                gl.viewport(0, 0, width as i32, height as i32);
                let aspect_ratio = width as f32 / height as f32;
                camera.set_projection(mat4_orthographic(
                    -aspect_ratio,
                    aspect_ratio,
                    -1.0,
                    1.0,
                    0.0,
                    1.0,
                ));
            },
            Event::Draw { .. } => {
                // First update physics
                ball.ball_physics(&line_points);

                // Clear the screen.
                unsafe {
                    gl.clear_color(19.0 / 255.0, 12.0 / 255.0, 61.0 / 255.0, 1.0);
                    gl.clear(COLOR_BUFFER_BIT | DEPTH_BUFFER_BIT);
                }

                shader_program.use_program(&gl);

                // Bind the camera's view and projection
                shader_program.set_matrix(&gl, "u_view", &camera.view);
                shader_program.set_matrix(&gl, "u_projection", &camera.projection);

                // First render the line
                shader_program.set_matrix(&gl, "u_model", &Matrix4x4::IDENTITY);
                shader_program.set_color(&gl, "u_color", &white);

                line_mesh.draw(&gl);

                // Then render the circle
                shader_program.set_matrix(
                    &gl,
                    "u_model",
                    &mat4_from_trs(
                        ball.position,
                        Quaternion::IDENTITY,
                        Vector3::new_uniform(ball.radius),
                    ),
                );
                shader_program.set_color(&gl, "u_color", &ball.color);

                circle.draw(&gl);

                // Draw debug circle
                shader_program.set_matrix(
                    &gl,
                    "u_model",
                    &mat4_from_trs(
                        ball.intersection_position,
                        Quaternion::IDENTITY,
                        Vector3::new_uniform(ball.radius * 0.5),
                    ),
                );
                shader_program.set_color(&gl, "u_color", &ball.color);

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

fn screen_to_gl(x: f32, y: f32, camera: &Camera, window_width: u32, window_height: u32) -> Vector3 {
    let v = Vector3::new(
        x / (window_width as f32),
        1.0 - (y / (window_height as f32)),
        0.0,
    ) * 2.0
        - Vector3::new(1.0, 1.0, 0.0);

    mat4_transform_point(
        &camera.inverse_view,
        mat4_transform_point(&camera.inverse_projection, v),
    )
}
