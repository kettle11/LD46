use glow::*;
use kettlewin::*;

mod gl;

fn main() {
    let (app, mut event_loop) = initialize();

    event_loop.run_async(app, run);
}

async fn run(app: Application, mut events: Events) {
    let window = app.new_window().build().unwrap();
    let (mut gl_context, gl) = gl::setup(&window);
    loop {
        match events.next_event().await {
            Event::WindowCloseRequested { .. } => app.quit(),
            Event::Draw { .. } => {
                // Clear the screen.
                unsafe {
                    gl.clear_color(0.3765, 0.3137, 0.8627, 1.0);
                    gl.clear(COLOR_BUFFER_BIT | DEPTH_BUFFER_BIT);
                }
                // Finally display what we've drawn.
                gl_context.swap_buffers();

                //window.request_redraw();
            }
            _ => {}
        }
    }
}
