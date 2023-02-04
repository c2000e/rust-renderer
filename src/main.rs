mod renderer;
mod pipeline;

use winit::{
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    window::Window,
};

async fn run() {
    let event_loop = EventLoop::new();
    let window = Window::new(&event_loop).unwrap();
    window.set_title("rust-renderer");

    let mut renderer_state = renderer::RendererState::new(&window).await;

    let render_pipeline = pipeline::create_render_pipeline(
        &renderer_state.device,
        renderer_state.surface_config.format,
    );

    event_loop.run(move |event, _, control_flow| {
        control_flow.set_poll();

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => control_flow.set_exit(),
            Event::MainEventsCleared => {
                match renderer_state.render(&render_pipeline) {
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
