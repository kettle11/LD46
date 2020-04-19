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

use wasm_bindgen::{prelude::*, JsCast};
use wasm_bindgen_futures::JsFuture;
#[wasm_bindgen(module = "/src/helpers.js")]
extern "C" {
    fn download(path: &str, text: &str);
}
fn main() {
    #[cfg(target_arch = "wasm32")]
    console_error_panic_hook::set_once();

    let (app, mut event_loop) = initialize();
    event_loop.run_async(app, run);
}

#[derive(Debug, Clone, Copy)]
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
}

#[derive(Debug)]
struct Collectible {
    position: Vector3,
    radius: f32,
    color: Color,
    collected: bool,
}

struct Camera {
    projection: Matrix4x4,
    view: Matrix4x4,
    inverse_projection: Matrix4x4,
    inverse_view: Matrix4x4,
}

struct Level {
    line_color: Color,
    collectibles: Vec<Collectible>,
}

impl Level {
    pub fn reset(&mut self) {
        self.collectibles.clear();
    }
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
                if velocity_along_collision < 0.0 {
                    self.velocity -= normal_of_collision * velocity_along_collision * 1.4;
                }
                self.position += normal_of_collision * 0.0001;
            }
        }
        self.position += self.velocity;
    }

    fn check_for_collectibles(&mut self, collectibles: &mut [Collectible]) {
        for collectible in collectibles {
            if !collectible.collected
                && (self.position - collectible.position).length()
                    < self.radius + collectible.radius
            {
                log!("COLLECT!");
                collectible.collected = true;
            }
        }
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

struct MouseState {
    position: Vector2,
    frame: u32,
    mouse_up: bool,
    collectible_place: bool,
}

struct MousePlayback {
    state: Vec<MouseState>,
    current_frame: u32,
    current_state: usize,
    current_frame_recording: u32,
    playing: bool,
}

impl MousePlayback {
    pub fn new() -> Self {
        Self {
            state: Vec::new(),
            current_state: 0,
            current_frame_recording: 0,
            playing: false,
            current_frame: 0,
        }
    }

    pub fn increment_frame(&mut self) {
        self.current_frame_recording += 1;
    }

    pub fn record_collectible(&mut self, position: Vector2) {
        self.state.push(MouseState {
            position,
            frame: self.current_frame_recording,
            mouse_up: false,
            collectible_place: true,
        });
    }

    pub fn record_mouse(&mut self, position: Vector2) {
        self.state.push(MouseState {
            position,
            frame: self.current_frame_recording,
            mouse_up: false,
            collectible_place: false,
        });
    }

    pub fn record_mouse_up(&mut self) {
        self.state.push(MouseState {
            position: Vector2::new(0., 0.),
            frame: self.current_frame_recording,
            mouse_up: true,
            collectible_place: false,
        });
    }

    pub fn reset_playback(&mut self) {
        self.current_frame = 0;
        self.current_state = 0;
    }

    pub fn save(&self) {
        let mut string = String::new();
        for s in &self.state {
            if s.mouse_up {
                string += "a"; // a is mouseup
                string += " ";
            } else if s.collectible_place {
                string += "b"; // b is collectible place
                string += " ";
                string += &s.position.x.to_string();
                string += " ";
                string += &s.position.y.to_string();
                string += " ";
            } else {
                string += &s.position.x.to_string();
                string += " ";
                string += &s.position.y.to_string();
                string += " ";
            }
            string += &s.frame.to_string();
            string += " ";
        }
        download("level.txt", &string);
    }

    pub fn load(&mut self, s: &str) {
        self.state.clear();
        let mut s = s.split(" ").peekable();

        while let Some(_) = s.peek() {
            let first = s.next().unwrap();

            match first {
                "a" => {
                    let frame = s.next().unwrap().parse().unwrap();

                    // Mouse up
                    self.state.push(MouseState {
                        position: Vector2::new(0.0, 0.0),
                        frame,
                        mouse_up: true,
                        collectible_place: false,
                    })
                }
                "b" => {
                    // Collectible
                    let x = s.next().unwrap().parse().unwrap();
                    let y = s.next().unwrap().parse().unwrap();
                    let frame = s.next().unwrap().parse().unwrap();

                    self.state.push(MouseState {
                        position: Vector2::new(x, y),
                        frame,
                        mouse_up: false,
                        collectible_place: true,
                    })
                }
                "" => {}
                _ => {
                    let x = first.parse().unwrap();
                    let y = s.next().unwrap().parse().unwrap();
                    let frame = s.next().unwrap().parse().unwrap();
                    self.state.push(MouseState {
                        position: Vector2::new(x, y),
                        frame,
                        mouse_up: false,
                        collectible_place: false,
                    })
                }
            }
        }
    }

    pub fn playback(&mut self, frames: u32, lines: &mut Lines, level: &mut Level) {
        if self.current_state < self.state.len() {
            if self.current_state == 0 {
                lines.end_segment();
                self.current_frame = self.state[self.current_state].frame; // Skip beginning delay.
            }

            self.current_frame += frames;
            while self.current_state < self.state.len()
                && self.state[self.current_state].frame < self.current_frame
            {
                let current_state = &self.state[self.current_state];
                // Play next action
                let position = current_state.position;
                if current_state.collectible_place {
                    level.collectibles.push(Collectible {
                        position: Vector3::new(position.x, position.y, 0.0),
                        color: Color::new(1.0, 1.0, 1.0, 1.0),
                        radius: 0.015,
                        collected: false,
                    })
                } else if current_state.mouse_up {
                    lines.end_segment();
                } else {
                    lines.add_segment(Vector3::new(position.x, position.y, 0.0));
                }
                self.current_state += 1;
            }
        }
    }
}

struct Lines {
    last_position: Option<Vector3>,
    line_points: Vec<Vector3>,
    needs_update: bool,
    mesh: Mesh,
}

impl Lines {
    pub fn new(gl: &GL) -> Self {
        Self {
            needs_update: false,
            last_position: None,
            line_points: Vec::new(),
            mesh: Mesh::new(&gl),
        }
    }
    pub fn end_segment(&mut self) {
        self.last_position = None;
    }

    pub fn add_segment(&mut self, position: Vector3) {
        self.needs_update = true;
        if let Some(last_position_inner) = self.last_position {
            if (last_position_inner - position).length() > 0.01 {
                self.line_points.push(Vector3::new(
                    last_position_inner.x,
                    last_position_inner.y,
                    0.0,
                ));
                self.line_points
                    .push(Vector3::new(position.x, position.y, 0.0));

                self.last_position = Some(position);
            }
        } else {
            // Add a point
            self.last_position = Some(position);
        }
    }

    pub fn clear(&mut self) {
        self.needs_update = true;
        self.line_points.clear();
    }

    pub fn update_mesh(&mut self, gl: &GL) {
        if self.needs_update {
            self.needs_update = false;
            lines::update_mesh_with_line(
                &gl,
                &mut self.mesh,
                &self.line_points,
                LINE_RADIUS,
                Vector3::FORWARD,
            );
        }
    }
}
async fn run(app: Application, mut events: Events) {
    let window = app.new_window().build().unwrap();
    let (mut gl_context, gl) = gl::setup(&window);
    //  let beach_image = load_image(&gl, "beach.jpg").await.unwrap();

    let vert = include_str!("shaders/vert.vs");
    let frag = include_str!("shaders/frag.fs");

    let shader_program = ShaderProgram::new(&gl, vert, frag);

    let mut screen_width = 0;
    let mut screen_height = 0;
    let mut mouse_down = false;

    let mut mouse_position = Vector2::new(0., 0.);

    let start_position = Vector3::new(0.0, 1.6, 0.0);
    let mut ball = Ball {
        position: start_position,
        velocity: Vector3::ZERO,
        radius: 0.06,
        color: Color::new(1.0, 1.0, 1.0, 1.0),
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

    let mut level = Level {
        line_color: Color::new(138.0 / 255.0, 132. / 255.0, 170. / 255.0, 1.0),
        collectibles: Vec::new(),
    };

    let mut lines = Lines::new(&gl);

    let mut mouse_playback = MousePlayback::new();
    let mut recording = true;

    let level_string = include_str!("levels/test.txt");
    mouse_playback.load(&level_string);
    mouse_playback.playing = true;
    loop {
        let event = events.next_event().await;
        match event {
            Event::WindowCloseRequested { .. } => app.quit(),
            Event::MouseMoved { x, y, .. } => {
                // When the mouse check if the mouse drawing should be updated.
                mouse_position = Vector2::new(x, y);

                if mouse_down {
                    let mouse_position = screen_to_world(
                        mouse_position.x,
                        mouse_position.y,
                        &camera,
                        screen_width,
                        screen_height,
                    );
                    if recording {
                        mouse_playback
                            .record_mouse(Vector2::new(mouse_position.x, mouse_position.y));
                    }
                    lines.add_segment(mouse_position);
                }
            }
            Event::MouseButtonDown {
                button: MouseButton::Right,
                x,
                y,
                ..
            } => {
                let mouse_pos = screen_to_world(x, y, &camera, screen_width, screen_height);
                mouse_playback.record_collectible(Vector2::new(mouse_pos.x, mouse_pos.y));

                level.collectibles.push(Collectible {
                    position: mouse_pos,
                    color: white,
                    radius: 0.015,
                    collected: false,
                });
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
                lines.end_segment();
                if recording {
                    mouse_playback.record_mouse_up();
                }
            }
            Event::KeyDown { key: Key::P, .. } => {
                lines.clear();
                mouse_playback.reset_playback();
                mouse_playback.playing = true;
                level.reset();
            }
            Event::KeyDown { key: Key::S, .. } => {
                mouse_playback.save();
            }
            Event::KeyDown { key: Key::E, .. } => {
                lines.clear();
                lines.update_mesh(&gl);
            }
            Event::KeyDown { key: Key::R, .. } => {
                ball.position = start_position;
                ball.velocity = Vector3::ZERO;
                lines.clear();
                mouse_playback.reset_playback();
                mouse_playback.playing = true;
                level.reset();
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
                mouse_playback.increment_frame();
                if mouse_playback.playing {
                    mouse_playback.playback(2, &mut lines, &mut level);
                }
                // First update physics
                ball.ball_physics(&lines.line_points);
                ball.check_for_collectibles(&mut level.collectibles);

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
                shader_program.set_color(&gl, "u_color", &level.line_color);

                // Only updates if necessary
                lines.update_mesh(&gl);
                lines.mesh.draw(&gl);

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

                // Draw collectibles
                for collectible in &level.collectibles {
                    if !collectible.collected {
                        shader_program.set_matrix(
                            &gl,
                            "u_model",
                            &mat4_from_trs(
                                collectible.position,
                                Quaternion::IDENTITY,
                                Vector3::new_uniform(collectible.radius),
                            ),
                        );
                        shader_program.set_color(&gl, "u_color", &ball.color);

                        circle.draw(&gl);
                    }
                }

                // Finally display what we've drawn.
                // Since we're using web this happens automatically, but on desktop this call is required.
                gl_context.swap_buffers();
                window.request_redraw();
            }
            _ => {}
        }
    }
}

fn screen_to_world(
    x: f32,
    y: f32,
    camera: &Camera,
    window_width: u32,
    window_height: u32,
) -> Vector3 {
    let v = Vector3::new(
        x / (window_width as f32),
        1.0 - (y / (window_height as f32)),
        0.0,
    ) * 2.0
        - Vector3::new(1.0, 1.0, 0.0);

    let mut p = mat4_transform_point(
        &camera.inverse_view,
        mat4_transform_point(&camera.inverse_projection, v),
    );
    p.z = 0.0;
    p
}
