use crate::*;
pub struct ShaderProgram {
    program: Program,
}

fn compile_shader(gl: &Context, shader_type: u32, source: &str) -> <Context as HasContext>::Shader {
    #[cfg(all(target_arch = "wasm32"))]
    let version = ""; // No version for WebGL1
                      // let version = "#version 300 es";
    #[cfg(all(not(target_arch = "wasm32")))]
    let version = "#version 410";

    let source = &format!("{}\n{}", version, source);
    unsafe {
        let shader = gl.create_shader(shader_type).unwrap();
        gl.shader_source(shader, source);
        gl.compile_shader(shader);

        if !gl.get_shader_compile_status(shader) {
            log!("Type: {:?}", shader_type);
            log!("{}", source);
            log!("{}", gl.get_shader_info_log(shader));
            panic!();
        }

        shader
    }
}

impl ShaderProgram {
    pub fn new(gl: &GL, vertex_source: &str, fragment_source: &str) -> Self {
        let vertex_shader = compile_shader(gl, VERTEX_SHADER, vertex_source);
        let fragment_shader = compile_shader(gl, FRAGMENT_SHADER, fragment_source);

        unsafe {
            let program = gl.create_program().unwrap();
            gl.attach_shader(program, vertex_shader);
            gl.attach_shader(program, fragment_shader);
            gl.link_program(program);

            if !gl.get_program_link_status(program) {
                println!("{}", gl.get_program_info_log(program));
                panic!();
            }

            ShaderProgram { program }
        }
    }

    pub fn use_program(&self, gl: &GL) {
        unsafe {
            gl.use_program(Some(self.program));
        }
    }

    pub fn set_matrix(&self, gl: &GL, name: &str, m: &Matrix4x4) {
        unsafe {
            let location = gl.get_uniform_location(self.program, name);
            gl.uniform_matrix_4_f32_slice(location.as_ref(), false, &m.0);
        }
    }

    pub fn set_color(&self, gl: &GL, name: &str, color: &Color) {
        unsafe {
            let location = gl.get_uniform_location(self.program, name);
            gl.uniform_4_f32(location.as_ref(), color.r, color.g, color.b, color.a);
        }
    }
}
