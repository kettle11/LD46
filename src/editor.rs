use crate::*;

pub struct Editor {
    left_mouse_down: bool,
    right_mouse_down: bool,
    mouse_position: (f32, f32),
    dragging_start: bool,
    pub active: bool,
}

impl Editor {
    pub fn new() -> Self {
        Self {
            left_mouse_down: false,
            right_mouse_down: false,
            mouse_position: (0., 0.),
            active: false,
            dragging_start: false,
        }
    }

    pub fn update(
        &mut self,
        event: Event,
        mouse_playback: &mut MousePlayback,
        level: &mut Level,
        level_lines: &mut Lines,
        user_lines: &mut Lines,
        camera: &Camera,
        screen_width: u32,
        screen_height: u32,
    ) {
        mouse_playback.recording = true;
        //  user_lines.clear();
        match event {
            Event::MouseMoved { x, y, .. } => {
                if self.left_mouse_down && !self.dragging_start {
                    let mouse_position =
                        screen_to_world(x, y, &camera, screen_width, screen_height);
                    mouse_playback.record_mouse(Vector2::new(mouse_position.x, mouse_position.y));
                    level_lines.add_segment(mouse_position);
                }
                self.mouse_position = (x, y);
            }
            Event::MouseButtonDown {
                button: MouseButton::Right,
                x,
                y,
                ..
            } => {
                self.right_mouse_down = true;
            }
            Event::MouseButtonUp {
                button: MouseButton::Right,
                x,
                y,
                ..
            } => {
                self.right_mouse_down = false;
            }
            Event::MouseButtonDown {
                button: MouseButton::Left,
                x,
                y,
                ..
            } => {
                self.left_mouse_down = true;
            }
            Event::MouseButtonUp {
                button: MouseButton::Left,
                x,
                y,
                ..
            } => {
                self.left_mouse_down = false;
                level_lines.end_segment();
                mouse_playback.record_mouse_up();
            }
            Event::KeyDown { key: Key::R, .. } => {
                level_lines.clear();
                level.clear();

                mouse_playback.erase_rewind();

                mouse_playback.play_until_end(level_lines, level);
                log!("MOUSE PLAYBACK STATE: {:?}", mouse_playback.current_state);
            }
            Event::KeyRepeat { key: Key::R, .. } => {
                level_lines.clear();
                level.clear();

                mouse_playback.erase_rewind();
                mouse_playback.erase_rewind();
                mouse_playback.erase_rewind();
                mouse_playback.erase_rewind();

                mouse_playback.play_until_end(level_lines, level);
                log!("MOUSE PLAYBACK STATE: {:?}", mouse_playback.current_state);
            }
            Event::KeyDown { key: Key::A, .. } => {
                mouse_playback.clear();
                level.clear();
                level_lines.clear();
            }
            Event::KeyDown { key: Key::C, .. } => {
                // Add a collectible to the world.
                let mouse_pos = screen_to_world(
                    self.mouse_position.0,
                    self.mouse_position.1,
                    &camera,
                    screen_width,
                    screen_height,
                );
                if mouse_playback.recording {
                    mouse_playback.record_collectible(Vector2::new(mouse_pos.x, mouse_pos.y));

                    level.collectibles.push(Collectible {
                        position: mouse_pos,
                        color: Color::new(1.0, 1.0, 1.0, 1.0),
                        radius: 0.015,
                        collected: false,
                        alpha: 1.0,
                    });
                }
            }
            Event::KeyDown { key: Key::S, .. } => {
                save(&mouse_playback, &level);
            }
            Event::Draw { .. } => {
                if self.left_mouse_down {
                    let mouse_pos = screen_to_world(
                        self.mouse_position.0,
                        self.mouse_position.1,
                        &camera,
                        screen_width,
                        screen_height,
                    );
                    if (mouse_pos - level.start_position).length() < 0.05 {
                        self.dragging_start = true;
                        log!("DRAGGING START");
                    }
                }

                if self.dragging_start {
                    let mouse_pos = screen_to_world(
                        self.mouse_position.0,
                        self.mouse_position.1,
                        &camera,
                        screen_width,
                        screen_height,
                    );
                    level.start_position = mouse_pos;
                }

                if !self.left_mouse_down {
                    self.dragging_start = false;
                }
            }
            _ => {}
        }
    }
}

pub fn save(mouse_playback: &MousePlayback, level: &Level) {
    let mut string = String::new();
    string += &level.start_position.x.to_string();
    string += " ";
    string += &level.start_position.y.to_string();
    string += " ";
    for s in &mouse_playback.state {
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

pub fn load(mouse_playback: &mut MousePlayback, level: &mut Level, s: &str) {
    mouse_playback.state.clear();
    let mut s = s.split(" ").peekable();

    level.start_position = Vector3::new(
        s.next().unwrap().parse().unwrap(),
        s.next().unwrap().parse().unwrap(),
        0.,
    );
    while let Some(_) = s.peek() {
        let first = s.next().unwrap();

        match first {
            "a" => {
                let frame = s.next().unwrap().parse().unwrap();

                // Mouse up
                mouse_playback.state.push(MouseState {
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

                mouse_playback.state.push(MouseState {
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
                mouse_playback.state.push(MouseState {
                    position: Vector2::new(x, y),
                    frame,
                    mouse_up: false,
                    collectible_place: false,
                })
            }
        }
    }
}
