use glow::*;
use kettlewin::*;

mod editor;
mod gl;
mod image;
mod line_manager;
mod lines;
mod log;
mod mesh;
mod mouse_playback;
mod shader;

use editor::*;
use gl::*;
use image::*;
use line_manager::*;
use log::*;
use mesh::*;
use mouse_playback::*;
use shader::*;
#[allow(dead_code)]
mod zmath;

mod audio;
use audio::*;

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
pub struct Color {
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
#[derive(Debug)]
struct Collectible {
    position: Vector3,
    radius: f32,
    color: Color,
    alpha: f32,
    collected: bool,
}

struct Camera {
    projection: Matrix4x4,
    view: Matrix4x4,
    inverse_projection: Matrix4x4,
    inverse_view: Matrix4x4,
}

pub struct Level {
    start_position: Vector3,
    line_color: Color,
    user_line_color: Color,
    collected: u32,
    collectibles: Vec<Collectible>,
    complete: bool,
    setup: bool,
}

impl Level {
    pub fn new(start_position: Vector3, line_color: Color, user_line_color: Color) -> Self {
        Self {
            start_position,
            line_color,
            collected: 0,
            collectibles: Vec::new(),
            user_line_color,
            complete: false,
            setup: false,
        }
    }

    pub fn reset(&mut self) {
        self.collected = 0;
        for collectible in &mut self.collectibles {
            collectible.collected = false;
        }
        self.complete = false;
    }

    pub fn collect(&mut self, amount: u32) {
        self.collected += amount;
        if self.collected >= self.collectibles.len() as u32 {
            self.complete = true;
            log!("Finished level!");
        }

        log!("LEN: {:?}", self.collectibles.len());
    }

    pub fn clear(&mut self) {
        self.complete = false;
        self.collectibles.clear();
        self.reset();
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

struct Ball {
    position: Vector3,
    velocity: Vector3,
    radius: f32,
    color: Color,
    alpha: f32,
    moving: bool,
    grounded: i32,
}

impl Ball {
    fn check_lines(&mut self, points: &[Vector3]) {
        let len = points.len();

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

                self.grounded = 10;
                self.position += normal_of_collision * 0.0001;
            }
        }
    }
    // Every two Vector3s in points is a line segment
    fn ball_physics(&mut self, points: &[Vector3], user_lines: &[Vector3]) {
        self.grounded -= 1;
        self.velocity += Vector3::DOWN * 0.0001;

        self.check_lines(points);
        self.check_lines(user_lines);
        self.position += self.velocity;
    }

    fn check_for_collectibles(&mut self, level: &mut Level) -> (bool, f32) {
        let mut collected_count = 0;
        let mut height = 0.0;
        for collectible in &mut level.collectibles {
            if !collectible.collected
                && (self.position - collectible.position).length()
                    < self.radius + collectible.radius
            {
                log!("COLLECT!");
                collectible.alpha = 0.1;
                collectible.collected = true;
                collected_count += 1;
                height = collectible.position.y;
            }
        }

        if collected_count > 0 {
            level.collect(collected_count);
            (true, height)
        } else {
            (false, height)
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

async fn run(app: Application, mut events: Events) {
    let window = app.new_window().build().unwrap();
    let (mut gl_context, gl) = gl::setup(&window);
    //  let beach_image = load_image(&gl, "beach.jpg").await.unwrap();

    audio::setup();
    let vert = include_str!("shaders/vert.vs");
    let frag = include_str!("shaders/frag.fs");

    let shader_program = ShaderProgram::new(&gl, vert, frag);

    let mut screen_width = 0;
    let mut screen_height = 0;
    let mut mouse_down = false;
    let mut right_mouse_down = false;
    let mut mouse_position = Vector2::new(0., 0.);

    let mut mouse_playback = MousePlayback::new();
    mouse_playback.playing = true;

    let mut level = Level::new(
        Vector3::ZERO,
        Color::new(138.0 / 255.0, 132. / 255.0, 170. / 255.0, 1.0),
        Color::new(88.0 / 255.0, 65. / 255.0, 226. / 255.0, 1.0),
    );

    let mut lines = Lines::new(&gl);
    let mut user_lines = Lines::new(&gl);

    let level_string = [
        include_str!("levels/level0.txt"),            // Titlescreen
        include_str!("levels/level0a.txt"),           // Tutorial 1
        include_str!("levels/level0b.txt"),           // Tutorial 2
        include_str!("levels/level1.txt"),            // Remember starry nights
        include_str!("levels/level2.txt"),            // City
        include_str!("levels/level2b.txt"),           // The cool air
        include_str!("levels/level3.txt"),            // Big dipper
        include_str!("levels/breeze.txt"),            // Summer breeze
        include_str!("levels/icecream.txt"),          // icecream
        include_str!("levels/dew.txt"),               // Morning dew
        include_str!("levels/leaves.txt"),            // Leaves rustling
        include_str!("levels/cool_s.txt"),            // Cool s
        include_str!("levels/distant_mountains.txt"), // distant mountains
        include_str!("levels/love.txt"),              // Love
        include_str!("levels/mountain_forest.txt"),
        include_str!("levels/music.txt"),      // Music
        include_str!("levels/snowflakes.txt"), // Snowflakes
        include_str!("levels/squiggles.txt"),  // Squiggles (sort of like phone wire)
        include_str!("levels/hear.txt"),       // "if you hear this"
        include_str!("levels/remember.txt"),   // "I hope you remember"
        include_str!("levels/fin.txt"),        // fin
    ];

    let mut current_level = 0;

    let mut ball = Ball {
        position: level.start_position,
        velocity: Vector3::ZERO,
        radius: 0.06,
        color: Color::new(1.0, 1.0, 1.0, 1.0),
        alpha: 1.0,
        moving: false,
        grounded: 0,
    };

    let wind_sound = audio::load_audio("wind.wav").await.unwrap();
    let ball_sound = audio::load_audio("ball_roll.wav").await.unwrap();

    // Plays forever
    ball_sound.play_ball_audio();

    load_level(
        &mut ball,
        &level_string,
        current_level,
        &mut level,
        &mut mouse_playback,
        &mut lines,
        &mut user_lines,
        &wind_sound,
    );
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

    unsafe {
        gl.enable(BLEND);
        gl.blend_func(SRC_ALPHA, ONE_MINUS_SRC_ALPHA);
    }

    let mut fade_out = false;
    let mut fade_in = false;
    let mut level_alpha = 1.0;

    let bell_sound = audio::load_audio("bell1.wav").await.unwrap();

    // hardcoded to not do regular transitions.
    let mut prevent_transition = false;
    // Actually enables edit capabilties
    let mut editor = Editor::new();
    loop {
        let event = events.next_event().await;
        /*
        match event {
            Event::KeyDown { key: Key::E, .. } => {
                editor.active = !editor.active;
                if editor.active {
                    // To avoid accidentally losing work
                    prevent_transition = true;
                }
                log!("EDITOR ACTIVE: {:?}", editor.active);
            }
            Event::KeyDown {
                key: Key::Digit1, ..
            } => {
                prevent_transition = !prevent_transition;
                log!("PREVENT TRANSITION: {:?}", prevent_transition);
            }
            _ => {}
        }*/
        if editor.active {
            editor.update(
                event.clone(),
                &mut mouse_playback,
                &mut level,
                &mut lines,
                &mut user_lines,
                &camera,
                screen_width,
                screen_height,
            );
        }
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
                    if !editor.active {
                        user_lines.add_segment(mouse_position)
                    }
                }
            }
            Event::MouseButtonDown {
                button: MouseButton::Right,
                x,
                y,
                ..
            } => {
                audio::setup(); // Setup if not already setup
                let mouse_pos = screen_to_world(x, y, &camera, screen_width, screen_height);
                right_mouse_down = true;
            }
            Event::MouseButtonUp {
                button: MouseButton::Right,
                x,
                y,
                ..
            } => {
                right_mouse_down = false;
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
                user_lines.end_segment();
            }
            /*
            Event::KeyDown { key: Key::N, .. } => {
                level.complete = true;
            }*/
            Event::KeyDown {
                key: Key::Space, ..
            } => {
                reset(&mut ball, &mut level);
                ball.moving = true;
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
                // Check if the ball placeholder is clicked
                if level.setup && mouse_down && !editor.active {
                    let mouse_pos = screen_to_world(
                        mouse_position.x,
                        mouse_position.y,
                        &camera,
                        screen_width,
                        screen_height,
                    );
                    if (mouse_pos - level.start_position).length() < ball.radius {
                        reset(&mut ball, &mut level);
                        ball.moving = true;
                    }
                }

                // Check if the ball is out of the screen bounds
                if ball.position.x < -1.0 || ball.position.x > 3.0 || ball.position.y < 0.0 {
                    reset(&mut ball, &mut level);
                }

                // Check if the ball should be spawned
                if mouse_playback.complete && !level.setup {
                    ball.position = level.start_position;
                    ball.velocity = Vector3::ZERO;
                    level.setup = true;
                }

                // Eraser
                if right_mouse_down {
                    let mouse_position = screen_to_world(
                        mouse_position.x,
                        mouse_position.y,
                        &camera,
                        screen_width,
                        screen_height,
                    );

                    user_lines.erase(mouse_position, 0.06);
                }
                mouse_playback.increment_frame();
                if mouse_playback.playing {
                    mouse_playback.playback(8, &mut lines, &mut level);
                }
                // First update physics
                if level.setup && ball.moving {
                    ball.ball_physics(&lines.line_points, &user_lines.line_points);
                    let (hit_collectible, height) = ball.check_for_collectibles(&mut level);
                    if hit_collectible {
                        bell_sound.play(
                            1.2 + (height as f64 / 2.0) * 2.0 + audio::random() * 0.2,
                            2.0,
                        );
                    }
                }

                // Update ball roll audio
                let ball_roll_audio = ball.velocity.length() as f64 / 0.02;

                audio::ball_audio(ball_roll_audio * 3.5 * level_alpha, 0.2 + ball_roll_audio);

                // Clear the screen.
                unsafe {
                    gl.clear_color(19.0 / 255.0, 12.0 / 255.0, 61.0 / 255.0, 1.0);
                    gl.clear(COLOR_BUFFER_BIT | DEPTH_BUFFER_BIT);
                }

                shader_program.use_program(&gl);

                // Set the total level fade
                shader_program.set_float(&gl, "u_fade", level_alpha as f32);

                // Bind the camera's view and projection
                shader_program.set_matrix(&gl, "u_view", &camera.view);
                shader_program.set_matrix(&gl, "u_projection", &camera.projection);

                // First render the level lines
                shader_program.set_matrix(&gl, "u_model", &Matrix4x4::IDENTITY);
                shader_program.set_color(&gl, "u_color", &level.line_color);

                // Only updates if necessary
                lines.update_mesh(&gl);
                lines.mesh.draw(&gl);

                shader_program.set_color(&gl, "u_color", &level.user_line_color);

                // Only updates if necessary
                user_lines.update_mesh(&gl);
                user_lines.mesh.draw(&gl);

                // Render the circle placeholder
                shader_program.set_matrix(
                    &gl,
                    "u_model",
                    &mat4_from_trs(
                        level.start_position,
                        Quaternion::IDENTITY,
                        Vector3::new_uniform(ball.radius),
                    ),
                );
                shader_program.set_color(
                    &gl,
                    "u_color",
                    &Color::new(ball.color.r, ball.color.g, ball.color.b, 0.1),
                );
                circle.draw(&gl);

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
                ball.color = Color::new(1.0, 1.0, 1.0, ball.alpha);
                if ball.alpha < 1.0 {
                    ball.alpha += 0.015;
                } else {
                    ball.alpha = 1.0;
                }

                shader_program.set_color(&gl, "u_color", &ball.color);

                circle.draw(&gl);
                // Draw collectibles
                for collectible in &mut level.collectibles {
                    let color = Color::new(
                        collectible.color.r,
                        collectible.color.g,
                        collectible.color.b,
                        collectible.alpha,
                    );
                    shader_program.set_matrix(
                        &gl,
                        "u_model",
                        &mat4_from_trs(
                            collectible.position,
                            Quaternion::IDENTITY,
                            Vector3::new_uniform(collectible.radius),
                        ),
                    );
                    if !collectible.collected {
                        if collectible.alpha < 1.0 {
                            collectible.alpha += 0.04;
                        } else {
                            collectible.alpha = 1.0;
                        }
                        shader_program.set_color(&gl, "u_color", &color);
                    } else {
                        shader_program.set_color(
                            &gl,
                            "u_color",
                            &Color::new(color.r, color.g, color.b, 0.1),
                        );
                    }
                    circle.draw(&gl);
                }

                // Manage fade out
                if fade_out && level_alpha < 0.0 {
                    reset_ball(&mut ball, &mut level);
                    fade_in = true;
                    fade_out = false;
                    level_alpha = 0.0;
                    // This is where the actual level transition happen
                    if current_level + 1 < level_string.len() as u32 {
                        current_level += 1;
                        load_level(
                            &mut ball,
                            &level_string,
                            current_level,
                            &mut level,
                            &mut mouse_playback,
                            &mut lines,
                            &mut user_lines,
                            &wind_sound,
                        );
                    } else {
                        load_level(
                            &mut ball,
                            &level_string,
                            current_level,
                            &mut level,
                            &mut mouse_playback,
                            &mut lines,
                            &mut user_lines,
                            &wind_sound,
                        );
                    }
                }
                if fade_out {
                    level_alpha -= 0.02;
                }

                if fade_in {
                    level_alpha += 0.02;
                    if level_alpha > 1.0 {
                        fade_in = false;
                        level_alpha = 1.0;
                    }
                }

                // Kick off level transition
                if level.complete && !prevent_transition {
                    level.complete = false;
                    fade_out = true;
                    fade_in = false;
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

fn reset_ball(ball: &mut Ball, level: &mut Level) {
    if ball.moving {
        log!("SETTING BALL FADE");
        ball.alpha = 0.1;
    }
    ball.moving = false;
    ball.position = level.start_position;
    ball.velocity = Vector3::ZERO;
}

fn reset(ball: &mut Ball, level: &mut Level) {
    reset_ball(ball, level);
    // user_lines.clear();
    level.reset();
}

fn load_level(
    ball: &mut Ball,
    data: &[&str],
    current_level: u32,
    level: &mut Level,
    mouse_playback: &mut MousePlayback,
    lines: &mut Lines,
    user_lines: &mut Lines,
    wind: &Audio,
) {
    log!("CURRENT LEVEL {:?}", current_level);
    let data = data[current_level as usize];
    lines.clear();
    user_lines.clear();
    level.clear();
    mouse_playback.clear();
    editor::load(mouse_playback, level, data);
    mouse_playback.playing = true;
    ball.position = level.start_position;

    if current_level == 5 || current_level == 7 || current_level == 12 {
        wind.play(1.0, 5.0);
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
