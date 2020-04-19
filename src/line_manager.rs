use crate::*;
pub struct Lines {
    pub last_position: Option<Vector3>,
    pub line_points: Vec<Vector3>,
    pub needs_update: bool,
    pub mesh: Mesh,
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

    pub fn erase(&mut self, position: Vector3, radius: f32) {
        let len = self.line_points.len();
        let mut to_remove = Vec::new();

        for i in (1..len).step_by(2) {
            let intersection =
                point_with_line_segment(position, self.line_points[i - 1], self.line_points[i]);
            if intersection.0 < radius + LINE_RADIUS {
                to_remove.push(i);
            }
        }

        if to_remove.len() > 0 {
            self.needs_update = true;
        }
        // Swap points to remove to the end and then pop.
        let mut len = self.line_points.len();

        for i in &to_remove {
            let i = *i;
            self.line_points.swap(i, len - 1);
            self.line_points.swap(i - 1, len - 2);
            len -= 2;
        }

        for i in to_remove {
            self.line_points.pop();
            self.line_points.pop();
        }
    }

    pub fn clear(&mut self) {
        self.needs_update = true;
        self.last_position = None;
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
