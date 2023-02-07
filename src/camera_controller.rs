use crate::camera::Camera;
use winit::event::{
    VirtualKeyCode,
    ElementState,
};
extern crate nalgebra_glm as glm;

pub struct CameraController {
    pub speed: f32,
    pub sensitivity: f32,
    x_axis_positive: f32,
    x_axis_negative: f32,
    y_axis_positive: f32,
    y_axis_negative: f32,
    z_axis_positive: f32,
    z_axis_negative: f32,
    u_axis_positive: f32,
    u_axis_negative: f32,
    v_axis_positive: f32,
    v_axis_negative: f32,
}

impl CameraController {
    pub fn new(speed: f32, sensitivity: f32) -> Self {
        Self {
            speed,
            sensitivity,
            x_axis_positive: 0.0,
            x_axis_negative: 0.0,
            y_axis_positive: 0.0,
            y_axis_negative: 0.0,
            z_axis_positive: 0.0,
            z_axis_negative: 0.0,
            u_axis_positive: 0.0,
            u_axis_negative: 0.0,
            v_axis_positive: 0.0,
            v_axis_negative: 0.0,
        }
    }

    pub fn process_keyboard(
        &mut self,
        key: VirtualKeyCode,
        state: ElementState
    ) -> bool {
        let amount = if state == ElementState::Pressed { 1.0 } else { 0.0 };
        match key {
            VirtualKeyCode::D => {
                self.x_axis_positive = amount;
                true
            },
            VirtualKeyCode::A => {
                self.x_axis_negative = amount;
                true
            },
            VirtualKeyCode::Q => {
                self.y_axis_positive = amount;
                true
            },
            VirtualKeyCode::E => {
                self.y_axis_negative = amount;
                true
            },
            VirtualKeyCode::S => {
                self.z_axis_positive = amount;
                true
            },
            VirtualKeyCode::W => {
                self.z_axis_negative = amount;
                true
            },
            VirtualKeyCode::L => {
                self.u_axis_positive = amount;
                true
            },
            VirtualKeyCode::J => {
                self.u_axis_negative = amount;
                true
            },
            VirtualKeyCode::I => {
                self.v_axis_positive = amount;
                true
            },
            VirtualKeyCode::K => {
                self.v_axis_negative = amount;
                true
            },
            _ => false,
        }
    }

    fn update_position(&self, camera: &mut Camera, dt: std::time::Duration) {
        if let Some(relative_direction) = glm::Vec4::new(
            self.x_axis_positive - self.x_axis_negative,
            self.y_axis_positive - self.y_axis_negative,
            self.z_axis_positive - self.z_axis_negative,
            0.0,
        ).try_normalize(1.0e-6) {
            let absolute_direction = camera.view_matrix.transpose() * relative_direction;
            let velocity = absolute_direction * self.speed;
            let change_in_position = velocity * dt.as_secs_f32();
            camera.extrinsics.position += change_in_position;
        }
    }

    fn update_rotation(&self, camera: &mut Camera, dt: std::time::Duration) {
        let change_in_yaw = (
            self.u_axis_positive - self.u_axis_negative
        ) * self.sensitivity * dt.as_secs_f32();
        camera.extrinsics.yaw += change_in_yaw;

        let change_in_pitch = (
            self.v_axis_positive - self.v_axis_negative
        ) * self.sensitivity * dt.as_secs_f32();
        camera.extrinsics.pitch += change_in_pitch;
        let half_pi = glm::half_pi::<f32>();
        glm::clamp_scalar(camera.extrinsics.pitch, -half_pi, half_pi);
    }

    pub fn update_camera(&self, camera: &mut Camera, dt: std::time::Duration) {
        self.update_position(camera, dt);
        self.update_rotation(camera, dt);
        camera.view_matrix = camera.extrinsics.to_view_matrix();
    }
}