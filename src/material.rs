use crate::texture::Texture;

pub struct Material {
    pub albedo_map: Texture,
    pub sampler: wgpu::Sampler,
}

impl Material {
    pub fn from_bytes(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        albedo_bytes: &Vec<u8>,
        dimensions: (u32, u32),
        label: &str,
    ) -> Self {
        let albedo_label = label.to_string() + " Albedo";
        let albedo_map = Texture::from_bytes(
            device,
            queue,
            albedo_bytes,
            dimensions,
            &albedo_label,
        );

        let sampler_label = label.to_string() + " Sampler";
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            label: Some(&sampler_label),
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Nearest,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });

        Self {
            albedo_map,
            sampler,
        }
    }
}