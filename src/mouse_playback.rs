use crate::*;

pub struct MouseState {
    pub position: Vector2,
    pub frame: u32,
    pub mouse_up: bool,
    pub collectible_place: bool,
}

pub struct MousePlayback {
    pub state: Vec<MouseState>,
    pub current_frame: u32,
    pub current_state: usize,
    pub current_frame_recording: u32,
    pub playing: bool,
    pub recording: bool,
    pub complete: bool,
}

impl MousePlayback {
    pub fn new() -> Self {
        Self {
            state: Vec::new(),
            current_state: 0,
            current_frame_recording: 0,
            playing: false,
            current_frame: 0,
            recording: false,
            complete: true,
        }
    }

    pub fn increment_frame(&mut self) {
        self.current_frame_recording += 1;
    }

    pub fn record_collectible(&mut self, position: Vector2) {
        let max_frame_difference = 30;

        if self.recording {
            /*
            if let Some(last_frame) = self.state.last().as_ref() {
                if self.current_frame - last_frame.frame > max_frame_difference {
                    self.current_frame = last_frame.frame + max_frame_difference;
                }
            }
            */
            self.state.push(MouseState {
                position,
                frame: self.current_frame_recording,
                mouse_up: false,
                collectible_place: true,
            });
            self.current_state += 1;
            self.current_frame = self.current_frame_recording;
        }
    }

    pub fn record_mouse(&mut self, position: Vector2) {
        let max_frame_difference = 10;

        if self.recording {
            /*
            if let Some(last_frame) = self.state.last().as_ref() {
                if self.current_frame - last_frame.frame > max_frame_difference {
                    self.current_frame = last_frame.frame + max_frame_difference;
                }
            }
            */

            self.state.push(MouseState {
                position,
                frame: self.current_frame_recording,
                mouse_up: false,
                collectible_place: false,
            });
            self.current_state += 1;
            self.current_frame = self.current_frame_recording;
        }
    }

    pub fn record_mouse_up(&mut self) {
        if self.recording {
            self.state.push(MouseState {
                position: Vector2::new(0., 0.),
                frame: self.current_frame_recording,
                mouse_up: true,
                collectible_place: false,
            });
            self.current_state += 1;
            self.current_frame = self.current_frame_recording;
        }
    }

    pub fn clear(&mut self) {
        self.reset_playback();
        self.state.clear();
        self.complete = false;
    }

    pub fn reset_playback(&mut self) {
        self.current_frame = 0;
        self.current_state = 0;
    }

    pub fn erase_rewind(&mut self) {
        let state = self.state.pop();
        if let Some(state) = state {
            self.current_state -= 1;
            self.current_frame = state.frame;
        }
    }

    pub fn play_until_end(&mut self, lines: &mut Lines, level: &mut Level) {
        self.current_state = 0;
        self.current_frame = 0;
        while self.current_state < self.state.len() {
            self.playback(100, lines, level)
        }
    }

    pub fn playback(&mut self, frames: u32, lines: &mut Lines, level: &mut Level) {
        if self.current_state < self.state.len() {
            if self.current_state == 0 {
                lines.end_segment();
                self.current_frame = self.state[self.current_state].frame; // Skip beginning delay.
            }

            // Prevent long gaps
            let skip_ahead = self.current_state < self.state.len()
                && self.state[self.current_state].frame - self.current_frame > 240;
            if skip_ahead {
                self.current_frame = self.state[self.current_state].frame;
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
                        alpha: 0.0,
                    })
                } else if current_state.mouse_up {
                    lines.end_segment();
                } else {
                    lines.add_segment(Vector3::new(position.x, position.y, 0.0));
                }
                self.current_state += 1;
            }
        } else {
            self.complete = true;
            self.playing = false;
        }
    }
}
