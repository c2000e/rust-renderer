// Vertex shader
struct CameraUniform {
    view_proj: mat4x4<f32>,
};

@group(0) @binding(0)
var<uniform> camera: CameraUniform;

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) tangent: vec4<f32>,
    @location(3) texcoords: vec2<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) tangent: vec4<f32>,
    @location(3) texcoords: vec2<f32>,
};

@vertex
fn vs_main(model: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    out.clip_position = camera.view_proj * vec4<f32>(model.position, 1.0);
    out.position = model.position;
    out.normal = model.normal;
    out.tangent = model.tangent;
    out.texcoords = model.texcoords;
    return out;
}

// Fragment shader
@group(1) @binding(0)
var t_albedo: texture_2d<f32>;
@group(1) @binding(1)
var t_normal: texture_2d<f32>;
@group(1) @binding(2)
var s_material: sampler;

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let tangent = normalize(in.tangent.xyz - dot(in.tangent.xyz, in.normal) * in.normal);
    let bitangent = cross(tangent, in.normal);
    let tbn_matrix = mat3x3<f32>(tangent, bitangent, in.normal);
    let normal_map = 2.0 * textureSample(t_normal, s_material, in.texcoords).rgb - 1.0;
    let normal = normalize(tbn_matrix * normal_map);

    let object_color = 0.5 * (normal + 1.0);

    return vec4<f32>(normal, 1.0);
}
