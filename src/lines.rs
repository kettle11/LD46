use crate::*;

fn produce_end_cap(
    center: Vector3,
    right: Vector3,
    forward: Vector3,
    resolution: u32,
    vertices: &mut Vec<Vector3>,
    indices: &mut Vec<[u32; 3]>,
) {
    let start = vertices.len() as u32;
    vertices.push(center - right);
    vertices.push(center + right);

    let increment = crate::zmath::PI / (resolution + 1) as f32;
    let mut current_angle = 0.;

    for _ in 0..resolution {
        current_angle += increment;

        let new_vertex = vertices.len() as u32;
        vertices.push(center + right * current_angle.cos() + forward * current_angle.sin());

        indices.push([start, new_vertex - 1, new_vertex]);
    }
}

pub fn update_mesh_with_circle(
    gl: &GL,
    mesh: &mut Mesh,
    center: Vector3,
    radius: f32,
    resolution: u32, // Number of sides
) {
    let mut vertices = Vec::new();
    let mut indices = Vec::new();

    let start = vertices.len() as u32;

    vertices.push(center);

    let increment = (crate::zmath::PI * 2.0) / resolution as f32;
    let mut current_angle = increment;

    let right = Vector3::RIGHT * radius;
    let forward = Vector3::UP * radius;

    for i in 0..resolution {
        current_angle += increment;

        let new_vertex = vertices.len() as u32;
        let dir = Vector3::new(current_angle.cos(), current_angle.sin(), 0.0).normal();
        vertices.push(center + dir * radius);

        if i > 0 {
            indices.push([start, new_vertex, new_vertex - 1]);
        }
    }

    indices.push([start, 1, (vertices.len() - 1) as u32]);

    mesh.update(gl, &vertices, &indices);
}

/// Pass in an array where every two lines is a line segment
pub fn update_mesh_with_line(
    gl: &GL,
    mesh: &mut Mesh,
    lines: &[Vector3],
    radius: f32,
    plane_normal: Vector3,
) {
    let resolution = 4;
    let mut vertices = Vec::new();
    let mut indices = Vec::new();

    let line_len = lines.len();

    for i in (1..line_len).step_by(2) {
        let mut forward = lines[i] - lines[i - 1];
        forward.normalize();

        let mut right = Vector3::cross(forward, plane_normal);
        right.normalize();

        produce_end_cap(
            lines[i - 1],
            -right * radius,
            -forward * radius,
            resolution,
            &mut vertices,
            &mut indices,
        );

        let start = vertices.len() as u32;

        vertices.push(-right * radius + lines[i - 1]);
        vertices.push(right * radius + lines[i - 1]);
        vertices.push(right * radius + lines[i]);
        vertices.push(-right * radius + lines[i]);

        indices.push([start + 0, start + 1, start + 2]);
        indices.push([start + 0, start + 2, start + 3]);

        produce_end_cap(
            lines[i],
            right * radius,
            forward * radius,
            resolution,
            &mut vertices,
            &mut indices,
        );
    }

    mesh.update(gl, &vertices, &indices);
}
