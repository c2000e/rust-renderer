mod renderer;
mod pipeline;
mod vertex;

use winit::{
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    window::Window,
};
use wgpu::util::DeviceExt;

const VERTICES: [vertex::Vertex; 3] = [
    vertex::Vertex { position: [0.0, 0.5, 0.0], color: [1.0, 0.0, 0.0] },
    vertex::Vertex { position: [-0.5, -0.5, 0.0], color: [0.0, 1.0, 0.0] },
    vertex::Vertex { position: [0.5, -0.5, 0.0], color: [0.0, 0.0, 1.0] },
];

async fn run() {
    let event_loop = EventLoop::new();
    let window = Window::new(&event_loop).unwrap();
    window.set_title("rust-renderer");

    let mut renderer_state = renderer::RendererState::new(&window).await;

    let render_pipeline = pipeline::create_render_pipeline(
        &renderer_state.device,
        renderer_state.surface_config.format,
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
                match renderer_state.render(&render_pipeline, &vertex_buffer, 3) {
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
