use rust_renderer::bind_groups::*;
use rust_renderer::pipelines::*;
use rust_renderer::*;

use wgpu::util::DeviceExt;
use winit::{
    event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent},
    event_loop::EventLoop,
    window::Window,
};

async fn run() {
    let event_loop = EventLoop::new();
    let window = Window::new(&event_loop).unwrap();
    window.set_title("rust-renderer");

    let mut renderer_state = renderer::RendererState::new(&window).await;

    let mut camera = camera::Camera::new(
        camera::CameraExtrinsics {
            position: nalgebra_glm::Vec4::new(0.0, 0.0, 5.0, 1.0),
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
    let mut camera_controller = camera_controller::CameraController::new(5.0, 1.0);
    let camera_buffer =
        renderer_state
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Camera Buffer"),
                contents: bytemuck::cast_slice(&[camera.to_uniform_matrix()]),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            });
    let camera_bind_group_layout =
        camera_bind_group::create_bind_group_layout(&renderer_state.device);
    let camera_bind_group = camera_bind_group::create_bind_group(
        &renderer_state.device,
        &camera_bind_group_layout,
        &camera_buffer,
    );

    let render_pipeline = mesh_pipeline::create_render_pipeline(
        &renderer_state.device,
        renderer_state.surface_config.format,
        &camera_bind_group_layout,
    );

    let mut mesh_path = std::env::current_exe().expect("Failed to find path to executable.");
    mesh_path.pop();
    mesh_path.pop();
    mesh_path.pop();
    mesh_path.push("res/icosphere.gltf");
    let mesh = mesh::Mesh::from_gltf(mesh_path, &renderer_state.device);

    let mut last_update_time = std::time::Instant::now();
    event_loop.run(move |event, _, control_flow| {
        control_flow.set_poll();

        match event {
            Event::WindowEvent {
                event:
                    WindowEvent::CloseRequested
                    | WindowEvent::KeyboardInput {
                        input:
                            KeyboardInput {
                                state: ElementState::Released,
                                virtual_keycode: Some(VirtualKeyCode::Escape),
                                ..
                            },
                        ..
                    },
                ..
            } => control_flow.set_exit(),
            Event::WindowEvent {
                event:
                    WindowEvent::KeyboardInput {
                        input:
                            winit::event::KeyboardInput {
                                state,
                                virtual_keycode: Some(key),
                                ..
                            },
                        ..
                    },
                ..
            } => {
                camera_controller.process_keyboard(key, state);
            }
            Event::MainEventsCleared => {
                let this_update_time = std::time::Instant::now();
                let dt = this_update_time - last_update_time;
                last_update_time = this_update_time;

                camera_controller.update_camera(&mut camera, dt);
                renderer_state.queue.write_buffer(
                    &camera_buffer,
                    0,
                    bytemuck::cast_slice(&[camera.to_uniform_matrix()]),
                );
                match renderer_state.render(&render_pipeline, &camera_bind_group, &mesh) {
                    Ok(_) => {}
                    Err(e) => eprintln!("{:?}", e),
                }
            }
            _ => (),
        }
    });
}

fn main() {
    pollster::block_on(run());
}
