use crate::*;
pub struct Mesh {
    vertex_buffer: Buffer,
    index_buffer: Buffer,
    count: u32,
}

pub type Vertex = ([f32; 3], [f32; 2]);

pub type Tri = [u32; 3];
impl Mesh {
    pub fn update(&self, gl: &GL, vertices: &[Vertex], indices: &[Tri]) {
        unsafe {
            gl.bind_buffer(ARRAY_BUFFER, Some(self.vertex_buffer));
            gl.buffer_data_u8_slice(ARRAY_BUFFER, slice_to_bytes(&vertices), STATIC_DRAW);

            gl.bind_buffer(ELEMENT_ARRAY_BUFFER, Some(self.index_buffer));
            gl.buffer_data_u8_slice(ELEMENT_ARRAY_BUFFER, slice_to_bytes(&indices), STATIC_DRAW);
        }
    }

    pub fn new(gl: &GL, vertices: &[Vertex], indices: &[Tri]) -> Mesh {
        unsafe {
            let vertex_buffer = gl.create_buffer().unwrap();
            let index_buffer = gl.create_buffer().unwrap();
            let mesh = Mesh {
                vertex_buffer,
                index_buffer,
                count: (indices.len() * 3) as u32,
            };

            mesh.update(gl, vertices, indices);
            mesh
        }
    }

    pub fn draw(&self, gl: &GL) {
        unsafe {
            gl.bind_buffer(ARRAY_BUFFER, Some(self.vertex_buffer));
            gl.bind_buffer(ELEMENT_ARRAY_BUFFER, Some(self.index_buffer));
            gl.vertex_attrib_pointer_f32(0, 3, FLOAT, false, 5 * 4, 0);
            gl.vertex_attrib_pointer_f32(1, 2, FLOAT, false, 5 * 4, 3 * 4);
            gl.enable_vertex_attrib_array(0);
            gl.enable_vertex_attrib_array(1);

            gl.draw_elements(TRIANGLES, self.count as i32, UNSIGNED_INT, 0);
        }
    }
}

unsafe fn slice_to_bytes<T>(t: &[T]) -> &[u8] {
    let ptr = t.as_ptr() as *const u8;
    let size = std::mem::size_of::<T>() * t.len();
    std::slice::from_raw_parts(ptr, size)
}
