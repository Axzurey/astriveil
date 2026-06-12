struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) tangent: vec3<f32>,
    @location(3) color: vec4<f32>,

    @location(4) uv: vec2<f32>,
    @location(5) weights: vec4<f32>,
    @location(6) joints: vec4<u32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,

    @location(0) uv: vec2<f32>,
    @location(1) color: vec4<f32>
};

struct Camera {
    view_pos: vec4<f32>,
    view_proj: mat4x4<f32>
};

struct Model {
    model: mat4x4<f32>
};

struct Bones {
    matrices: array<mat4x4<f32>, 128>,
};

@group(0) @binding(0)
var<uniform> camera: Camera;

@group(1) @binding(0)
var tex: texture_2d<f32>;

@group(1) @binding(1)
var tex_sampler: sampler;

@group(2) @binding(0)
var<uniform> model: Model;

@group(2) @binding(1)
var<storage, read> bones: Bones;

@vertex
fn vs_main(input: VertexInput) -> VertexOutput {
    var out: VertexOutput;

    let skin =
        bones.matrices[input.joints.x] * input.weights.x +
        bones.matrices[input.joints.y] * input.weights.y +
        bones.matrices[input.joints.z] * input.weights.z +
        bones.matrices[input.joints.w] * input.weights.w;

    let skinned_pos = skin * vec4<f32>(input.position, 1.0);

    let world_pos = model.model * skinned_pos;

    out.clip_position = camera.view_proj * world_pos;
    out.uv = input.uv;
    out.color = input.color;

    return out;
}

@fragment
fn fs_main(input: VertexOutput) -> @location(0) vec4<f32> {
    let sampled = textureSample(tex, tex_sampler, input.uv);
    return vec4(sampled.xyz, 1.0) + input.color;
}