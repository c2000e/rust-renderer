use wgpu::util::DeviceExt;
use wgpu::VertexAttribute;
use wgpu::VertexFormat::Float32x3;

extern crate nalgebra_glm as glm;

pub struct Mesh {
    pub buffer: wgpu::Buffer,
    pub index_count: u32,
    index_range: (u64, u64),
    position_range: (u64, u64),
    normal_range: (u64, u64),
}

impl Mesh {
    pub fn position_layout<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: wgpu::VertexFormat::size(&Float32x3),
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[VertexAttribute {
                offset: 0,
                shader_location: 0,
                format: Float32x3,
            }],
        }
    }
    pub fn normal_layout<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: wgpu::VertexFormat::size(&Float32x3),
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[VertexAttribute {
                offset: 0,
                shader_location: 1,
                format: Float32x3,
            }],
        }
    }

    pub fn index_range(&self) -> std::ops::Range<u64> {
        self.index_range.0..self.index_range.1
    }
    pub fn position_range(&self) -> std::ops::Range<u64> {
        self.position_range.0..self.position_range.1
    }
    pub fn normal_range(&self) -> std::ops::Range<u64> {
        self.normal_range.0..self.normal_range.1
    }

    fn gltf_first_primitive(gltf: &gltf::Document) -> std::option::Option<gltf::Primitive> {
        let mut first_primitive = None;
        for mesh in gltf.meshes() {
            if let Some(primitive) = mesh.primitives().next() {
                first_primitive = Some(primitive);
            }
        }
        first_primitive
    }

    fn index_range_and_count(primitive: &gltf::Primitive) -> ((u64, u64), u32) {
        let accessor = primitive.indices().unwrap();
        let view = accessor.view().unwrap();
        let offset = view.offset() as u64;
        let length = view.length() as u64;
        let count = accessor.count() as u32;
        ((offset, offset + length), count)
    }
    fn attribute_range(primitive: &gltf::Primitive, attribute: &gltf::Semantic) -> (u64, u64) {
        let accessor = primitive.get(attribute).unwrap();
        let view = accessor.view().unwrap();
        let offset = view.offset() as u64;
        let length = view.length() as u64;
        (offset, offset + length)
    }

    pub fn from_gltf<P: AsRef<std::path::Path>>(path: P, device: &wgpu::Device) -> Self {
        // Load data from file
        let load_result = gltf::import(path);
        let (gltf, buffers, _images) = match load_result {
            Ok(data) => data,
            Err(error) => panic!("Problem opening the file: {:?}", error),
        };

        // Create buffer on gpu
        let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Mesh Buffer"),
            contents: bytemuck::cast_slice(&(buffers[0].0)),
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::INDEX,
        });

        // Get buffer access information
        let primitive = Mesh::gltf_first_primitive(&gltf).unwrap();
        let position_range = Mesh::attribute_range(&primitive, &gltf::Semantic::Positions);
        let normal_range = Mesh::attribute_range(&primitive, &gltf::Semantic::Normals);

        let (index_range, index_count) = Mesh::index_range_and_count(&primitive);

        // Return mesh object
        Self {
            buffer,
            index_range,
            index_count,
            position_range,
            normal_range,
        }
    }
}
