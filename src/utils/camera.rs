use glam::{Mat4, Vec3, Quat};

/// Projection modes for the camera.
pub enum ProjectionMode {
    Perspective {
        fov_y: f32,      // Field of view in radians
        aspect_ratio: f32,
        near_plane: f32,
        far_plane: f32,
    },
    Orthographic {
        left: f32,
        right: f32,
        bottom: f32,
        top: f32,
        near_plane: f32,
        far_plane: f32,
    },
}

/// Represents a 3D camera with position, orientation, and projection settings.
pub struct Camera {
    pub position: Vec3,
    pub orientation: Quat,
    pub projection_mode: ProjectionMode,
}

impl Camera {
    /// Creates a new perspective camera.
    pub fn new_perspective(position: Vec3, fov_y: f32, aspect_ratio: f32, near_plane: f32, far_plane: f32) -> Self {
        Self {
            position,
            orientation: Quat::IDENTITY,
            projection_mode: ProjectionMode::Perspective {
                fov_y,
                aspect_ratio,
                near_plane,
                far_plane,
            },
        }
    }

    /// Creates a new orthographic camera.
    pub fn new_orthographic(
        position: Vec3,
        left: f32,
        right: f32,
        bottom: f32,
        top: f32,
        near_plane: f32,
        far_plane: f32,
    ) -> Self {
        Self {
            position,
            orientation: Quat::IDENTITY,
            projection_mode: ProjectionMode::Orthographic {
                left,
                right,
                bottom,
                top,
                near_plane,
                far_plane,
            },
        }
    }

    /// Returns the view matrix for the camera.
    pub fn view_matrix(&self) -> Mat4 {
        let rotation = self.orientation;
        let forward = rotation * Vec3::Z;
        let up = rotation * Vec3::Y;
        let target = self.position + forward;
        Mat4::look_at_rh(self.position, target, up)
    }

    /// Returns the projection matrix for the camera based on its projection mode.
    pub fn projection_matrix(&self) -> Mat4 {
        match self.projection_mode {
            ProjectionMode::Perspective {
                fov_y,
                aspect_ratio,
                near_plane,
                far_plane,
            } => Mat4::perspective_rh(fov_y, aspect_ratio, near_plane, far_plane),
            ProjectionMode::Orthographic {
                left,
                right,
                bottom,
                top,
                near_plane,
                far_plane,
            } => Mat4::orthographic_rh(left, right, bottom, top, near_plane, far_plane),
        }
    }
    
    pub fn translate(& mut self, vec: glam::Vec3) {
        self.position += vec;
    }

    /// Moves the camera forward or backward in its local space.
    pub fn move_angle(&mut self, angle: Vec3, distance: f32) {
        self.position += angle * distance;
    }

    /// Moves the camera forward or backward in its local space.
    pub fn move_forward(&mut self, distance: f32) {
        let forward = self.orientation * Vec3::Z;
        self.position += forward * distance;
    }

    /// Moves the camera left or right in its local space.
    pub fn move_right(&mut self, distance: f32) {
        let right = self.orientation * Vec3::X;
        self.position += right * distance;
    }

    /// Moves the camera up or down in its local space.
    pub fn move_up(&mut self, distance: f32) {
        let up = self.orientation * Vec3::Y;
        self.position += up * distance;
    }

    /// Rotates the camera around its local X-axis (pitch).
    pub fn pitch(&mut self, angle: f32) {
        let rotation = Quat::from_axis_angle(Vec3::X, angle);
        self.orientation = rotation * self.orientation;
    }

    /// Rotates the camera around its local Y-axis (yaw).
    pub fn yaw(&mut self, angle: f32) {
        let rotation = Quat::from_axis_angle(Vec3::Y, angle);
        self.orientation = rotation * self.orientation;
    }

    /// Rotates the camera around its local Z-axis (roll).
    pub fn roll(&mut self, angle: f32) {
        let rotation = Quat::from_axis_angle(Vec3::Z, angle);
        self.orientation = rotation * self.orientation;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_camera_movement() {
        let mut camera = Camera::new_perspective(Vec3::new(0.0, 0.0, 0.0), 1.57, 1.33, 0.1, 100.0);

        // Test forward movement
        camera.move_forward(1.0);
        assert_eq!(camera.position, Vec3::new(0.0, 0.0, 1.0));

        // Test right movement
        camera.move_right(1.0);
        assert_eq!(camera.position, Vec3::new(1.0, 0.0, 1.0));
    }

    #[test]
    fn test_orthographic_projection() {
        let camera = Camera::new_orthographic(Vec3::new(0.0, 0.0, 0.0), -1.0, 1.0, -1.0, 1.0, 0.1, 100.0);
        let proj_matrix = camera.projection_matrix();
        assert!(proj_matrix.w_axis.w == 1.0);
    }
}

