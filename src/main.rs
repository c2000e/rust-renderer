mod renderer;
mod pipeline;
mod vertex;
mod texture;
mod bind_groups;
mod camera;

use bind_groups::{
    camera_bind_group,
    texture_bind_group,
};

use winit::{
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    window::Window,
};
use wgpu::util::DeviceExt;

const VERTICES: [vertex::Vertex; 3] = [
    vertex::Vertex { position: [0.0, 0.5, 0.0], tex_coords: [0.0, 1.0] },
    vertex::Vertex { position: [-0.5, -0.5, 0.0], tex_coords: [0.0, 0.0] },
    vertex::Vertex { position: [0.5, -0.5, 0.0], tex_coords: [1.0, 0.0] },
];

async fn run() {
    let event_loop = EventLoop::new();
    let window = Window::new(&event_loop).unwrap();
    window.set_title("rust-renderer");

    let mut renderer_state = renderer::RendererState::new(&window).await;

    let camera = camera::Camera::new(
        camera::CameraExtrinsics {
            position: nalgebra_glm::Vec3::new(0.0, 0.0, 5.0),
            yaw: -1.5707,
            pitch: 0.0,
        },
        camera::CameraIntrinsics {
            aspect: 1.0, // TODO: fix this!
            fovy: 1.04,
            near: 0.01,
            far: 50.0,
        },
    );
    let camera_buffer = renderer_state.device.create_buffer_init(
        &wgpu::util::BufferInitDescriptor {
            label: Some("Camera Buffer"),
            contents: bytemuck::cast_slice(&[camera.to_uniform_matrix()]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        },
    );
    let camera_bind_group_layout = camera_bind_group::create_bind_group_layout(
        &renderer_state.device,
    );
    let camera_bind_group = camera_bind_group::create_bind_group(
        &renderer_state.device,
        &camera_bind_group_layout,
        &camera_buffer
    );

    let albedo_bytes = include_bytes!("../res/bricks-albedo.png");
    let albedo_image = image::load_from_memory(albedo_bytes).unwrap();
    let texture = texture::Texture::from_image(
        &renderer_state.device,
        &renderer_state.queue,
        &albedo_image,
        Some("Texture"),
    );
    let texture_bind_group_layout = texture_bind_group::create_bind_group_layout(
        &renderer_state.device,
    );
    let texture_bind_group = texture_bind_group::create_bind_group(
        &renderer_state.device,
        &texture_bind_group_layout,
        &texture.view,
        &texture.sampler,
    );

    let render_pipeline = pipeline::create_render_pipeline(
        &renderer_state.device,
        renderer_state.surface_config.format,
        &camera_bind_group_layout,
        &texture_bind_group_layout,
    );

    let vertex_buffer = renderer_state.device.create_buffer_init(
        &wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(&VERTICES),
            usage: wgpu::BufferUsages::VERTEX,
        }
    );

    event_loop.run(move |event, _, control_flow| {
        control_flow.set_poll();

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => control_flow.set_exit(),
            Event::MainEventsCleared => {
                match renderer_state.render(
                    &render_pipeline,
                    &camera_bind_group,
                    &texture_bind_group,
                    &vertex_buffer,
                    3
                ) {
                    Ok(_) => {},
                    Err(e) => eprintln!("{:?}", e),
                }
            },
            _ => (),
        }
    });
}

fn main() {
    pollster::block_on(run());
}
