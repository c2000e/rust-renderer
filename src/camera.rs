use winit::dpi::PhysicalSize;

extern crate nalgebra_glm as glm;

#[derive(Copy, Clone)]
pub struct CameraExtrinsics {
    pub position: glm::Vec4,
    pub yaw: f32,
    pub pitch: f32,
}

impl CameraExtrinsics {
    pub fn to_view_matrix(&self) -> glm::Mat4 {
        let (sin_yaw, cos_yaw) = self.yaw.sin_cos();
        let (sin_pitch, cos_pitch) = self.pitch.sin_cos();
        glm::look_at_rh(
            &self.position.xyz(),
            &glm::Vec3::new(
                cos_pitch * cos_yaw + self.position.x,
                sin_pitch + self.position.y,
                cos_pitch * sin_yaw + self.position.z,
            ),
            &glm::Vec3::y(),
        )
    }
}

#[derive(Copy, Clone)]
pub struct CameraIntrinsics {
    pub aspect: f32,
    pub fovy: f32,
    pub near: f32,
    pub far: f32,
}

impl CameraIntrinsics {
    pub fn to_perspective_matrix(&self) -> glm::Mat4 {
        glm::perspective_zo(self.aspect, self.fovy, self.near, self.far)
    }
}

pub struct Camera {
    pub extrinsics: CameraExtrinsics,
    pub intrinsics: CameraIntrinsics,
    pub view_matrix: glm::Mat4,
    pub proj_matrix: glm::Mat4,
}

impl Camera {
    pub fn new(extrinsics: CameraExtrinsics, intrinsics: CameraIntrinsics) -> Self {
        let view_matrix = extrinsics.to_view_matrix();
        let proj_matrix = intrinsics.to_perspective_matrix();
        Self {
            extrinsics,
            intrinsics,
            view_matrix,
            proj_matrix,
        }
    }

    pub fn to_uniform_matrix(&self) -> [[f32; 4]; 4] {
        (self.proj_matrix * self.view_matrix).into()
    }

    pub fn set_aspect(&mut self, size: PhysicalSize<u32>) {
        self.intrinsics.aspect = size.width as f32 / size.height as f32;
        self.proj_matrix = self.intrinsics.to_perspective_matrix();
    }
}
