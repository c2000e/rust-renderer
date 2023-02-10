use crate::texture::Texture;

pub struct Material {
    pub albedo_map: Texture,
    pub normal_map: Texture,
    pub roughness_metalness_map: Texture,
    pub sampler: wgpu::Sampler,
}

impl Material {
    pub fn from_bytes(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        albedo_bytes: &Vec<u8>,
        normal_bytes: &Vec<u8>,
        roughness_metalness_bytes: &Vec<u8>,
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

        let normal_label = label.to_string() + " Normal";
        let normal_map = Texture::from_bytes(
            device,
            queue,
            normal_bytes,
            dimensions,
            &normal_label,
        );

        let roughness_metalness_label = label.to_string() + " Roughness, Metalness";
        let roughness_metalness_map = Texture::from_bytes(
            device,
            queue,
            roughness_metalness_bytes,
            dimensions,
            &roughness_metalness_label,
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
            normal_map,
            roughness_metalness_map,
            sampler,
        }
    }
}