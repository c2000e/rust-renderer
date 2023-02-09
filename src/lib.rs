pub mod bind_groups {
    pub mod camera_bind_group;
    pub mod texture_bind_group;
}
pub mod pipelines {
    pub mod mesh_pipeline;
}
pub mod camera;
pub mod camera_controller;
pub mod mesh;
pub mod renderer;

pub mod constants {
    use crate::camera::{CameraExtrinsics, CameraIntrinsics};
    pub const DEFAULT_CAMERA_EXTRINSICS: CameraExtrinsics = CameraExtrinsics {
        position: nalgebra_glm::Vec4::new(0.0, 0.0, 5.0, 1.0),
        yaw: -1.5707,
        pitch: 0.0,
    };

    pub const DEFAULT_CAMERA_INTRINSICS: CameraIntrinsics = CameraIntrinsics {
        aspect: 1.0,
        fovy: 1.04,
        near: 0.01,
        far: 50.0,
    };
}
