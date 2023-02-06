extern crate nalgebra_glm as glm;

pub struct CameraExtrinsics {
    pub position: glm::Vec3,
    pub yaw: f32,
    pub pitch: f32,
}

impl CameraExtrinsics {
    fn to_view_matrix(&self) -> glm::Mat4 {
        let (sin_yaw, cos_yaw) = self.yaw.sin_cos();
        let (sin_pitch, cos_pitch) = self.pitch.sin_cos();
        glm::look_at_rh(
            &self.position,
            &glm::Vec3::new(
                cos_pitch * cos_yaw,
                sin_pitch,
                cos_pitch * sin_yaw,
            ),
            &glm::Vec3::new(0.0, 1.0, 0.0),
        )
    }
}

pub struct CameraIntrinsics {
    pub aspect: f32,
    pub fovy: f32,
    pub near: f32,
    pub far: f32,
}

impl CameraIntrinsics {
    fn to_perspective_matrix(&self) -> glm::Mat4 {
        glm::perspective_zo(self.aspect, self.fovy, self.near, self.far)
    }
}

pub struct Camera {
    view_matrix: glm::Mat4,
    proj_matrix: glm::Mat4,
}

impl Camera {
    pub fn new(
        extrinsics: CameraExtrinsics,
        intrinsics: CameraIntrinsics
    ) -> Self {
        let view_matrix = extrinsics.to_view_matrix();
        let proj_matrix = intrinsics.to_perspective_matrix();
        Self {
            view_matrix,
            proj_matrix,
        }
    }

    pub fn to_uniform_matrix(&self) -> [[f32; 4]; 4] {
        (self.proj_matrix * self.view_matrix).into()
    }
}

