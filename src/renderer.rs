use winit::window::Window;
pub struct RendererState {
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
}

impl RendererState {
    pub async fn new(window: &Window) -> Self {
        let instance = wgpu::Instance::new(
            wgpu::InstanceDescriptor {
                backends: wgpu::Backends::PRIMARY,
                dx12_shader_compiler: wgpu::Dx12Compiler::Fxc,
            }
        );

        let surface = unsafe { instance.create_surface(&window) }.unwrap();

        let adapter = instance.request_adapter(
            &wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                force_fallback_adapter: false,
                compatible_surface: Some(&surface),
            }
        ).await.unwrap();

        let (device, queue) = adapter.request_device(
            &wgpu::DeviceDescriptor {
                label: Some("device"),
                features: wgpu::Features::empty(),
                limits: wgpu::Limits::default(),
            },
            None
        ).await.unwrap();

        let size = window.inner_size();
        let surface_config = surface.get_default_config(
            &adapter,
            size.width,
            size.height
        ).unwrap();
        surface.configure(&device, &surface_config);

        return Self {
            surface,
            device,
            queue
        }
    }

    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let output = self.surface.get_current_texture()?;
        let view = output.texture.create_view(
            &wgpu::TextureViewDescriptor::default()
        );

        let mut encoder = self.device.create_command_encoder(
            &wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            }
        );

        let render_pass = encoder.begin_render_pass(
            &wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[
                    Some(wgpu::RenderPassColorAttachment {
                        view: &view,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(
                                wgpu::Color {
                                    r: 0.1,
                                    g: 0.8,
                                    b: 0.7,
                                    a: 1.0,
                                }
                            ),
                            store: true,
                        },
                    }),
                ],
                depth_stencil_attachment: None,
            }
        );
        drop(render_pass);

        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        return Ok(());
    }
}
