use crate::*;
use js_sys;
use wasm_bindgen::{prelude::*, JsCast};
use wasm_bindgen_futures::JsFuture;
use web_sys::*;

#[wasm_bindgen(module = "/src/helpers.js")]
extern "C" {
    pub fn setup();
    pub fn loadAudio(path: &str) -> js_sys::Promise;
    pub fn playAudio(audio_buffer: &AudioBuffer, rate: f64, gain: f64);
    pub fn playBallAudio(audio_buffer: &AudioBuffer, rate: f64, gain: f64);
    pub fn ballAudio(gain: f64, rate: f64);

}

pub struct Audio {
    audio_buffer: AudioBuffer,
}

pub async fn load_audio(path: &str) -> Result<Audio, ()> {
    let path = path.to_owned();
    let audio = JsFuture::from(loadAudio(&path)).await.unwrap();
    let audio_buffer: AudioBuffer = audio.dyn_into().unwrap();

    unsafe { Ok(Audio { audio_buffer }) }
}

impl Audio {
    pub fn play(&self, rate: f64, gain: f64) {
        playAudio(&self.audio_buffer, rate, gain);
    }
    pub fn play_ball_audio(&self) {
        playBallAudio(&self.audio_buffer, 1.0, 1.0);
    }
}

pub fn ball_audio(gain: f64, rate: f64) {
    ballAudio(gain, rate);
}

pub fn random() -> f64 {
    js_sys::Math::random()
}
