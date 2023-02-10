use rust_renderer::bind_groups::*;
use rust_renderer::constants::*;
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
    let window = {
        let window = Window::new(&event_loop).unwrap();
        window.set_title("rust-renderer");
        window
    };

    let mut renderer_state = renderer::RendererState::new(&window).await;

    let mut camera = camera::Camera::new(DEFAULT_CAMERA_EXTRINSICS, DEFAULT_CAMERA_INTRINSICS);
    camera.set_aspect(window.inner_size());

    let mut camera_controller = camera_controller::CameraController::new(5.0, 1.0);

    let camera_buffer =
        renderer_state
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Camera Buffer"),
                contents: bytemuck::cast_slice(&[camera.to_uniform_matrix()]),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            });

    let mesh = {
        let mut mesh_path = std::env::current_exe().expect("Failed to find path to executable.");
        mesh_path.pop();
        mesh_path.pop();
        mesh_path.pop();
        mesh_path.push("res/avocado/avocado.gltf");
        mesh::Mesh::from_gltf(&mesh_path, &renderer_state.device, &renderer_state.queue)
    };

    // TODO: block not really necessary, just helps clarify what is minimum
    // required for the main render loop. Feel free to revert this
    let (camera_bind_group, material_bind_group, render_pipeline) = {
        let camera_bind_group_layout =
            camera_bind_group::create_bind_group_layout(&renderer_state.device);
        let material_bind_group_layout =
            material_bind_group::create_bind_group_layout(&renderer_state.device);
        (
            camera_bind_group::create_bind_group(
                &renderer_state.device,
                &camera_bind_group_layout,
                &camera_buffer,
            ),
            material_bind_group::create_bind_group(
                &renderer_state.device,
                &material_bind_group_layout,
                &mesh.material.albedo_map.view,
                &mesh.material.normal_map.view,
                &mesh.material.sampler,
            ),
            mesh_pipeline::create_render_pipeline(
                &renderer_state.device,
                renderer_state.surface_config.format,
                &camera_bind_group_layout,
                &material_bind_group_layout,
            ),
        )
    };

    let depth_texture = texture::Texture::create_depth_texture(
        &renderer_state.device,
        window.inner_size().width,
        window.inner_size().height,
    );

    let mut last_update_time = std::time::Instant::now();
    event_loop.run(move |winit_event, _, control_flow| {
        control_flow.set_poll();

        match winit_event {
            Event::WindowEvent {
                window_id,
                event: window_event,
            } if window.id() == window_id => match window_event {
                WindowEvent::CloseRequested
                | WindowEvent::KeyboardInput {
                    input:
                        KeyboardInput {
                            state: ElementState::Released,
                            virtual_keycode: Some(VirtualKeyCode::Escape),
                            ..
                        },
                    ..
                } => control_flow.set_exit(),
                WindowEvent::KeyboardInput {
                    input:
                        KeyboardInput {
                            virtual_keycode: Some(key),
                            state,
                            ..
                        },
                    ..
                } => {
                    camera_controller.process_keyboard(key, state);
                }
                _ => (),
            },
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
                match renderer_state.render(
                    &render_pipeline,
                    &camera_bind_group,
                    &material_bind_group,
                    &mesh,
                    &depth_texture.view,
                ) {
                    Ok(_) => {}
                    Err(e) => eprintln!("{:?}", e),
                }
            }
            _ => (),
        }
    });
}

fn main() {
    env_logger::init();
    pollster::block_on(run());
}
