use glow::*;
use kettlewin::*;

pub type GL = Context;

pub fn setup(window: &Window) -> (GLContext, Context) {
    // Create a GLContext
    let mut gl_context = GLContext::new().webgl1().build().unwrap();

    // Assign the GLContext's window.
    gl_context.set_window(Some(window)).unwrap();

    #[cfg(target_arch = "wasm32")]
    let gl = glow::Context::from_webgl1_context(gl_context.webgl1_context().unwrap());
    #[cfg(not(target_arch = "wasm32"))]
    let gl = glow::Context::from_loader_function(|s| gl_context.get_proc_address(s));
    (gl_context, gl)
}
