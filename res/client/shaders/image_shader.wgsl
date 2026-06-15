struct VertexInput {
    @location(0) color_mask: vec4<f32>,
    @location(1) corner_radius: vec4<f32>,
    @location(2) position: vec2<f32>,
    @location(3) center: vec2<f32>,
    @location(4) size: vec2<f32>,
    @location(5) tex_coords: vec2<f32>,
    @location(6) image_index: u32,
    @location(7) rotation: f32
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec4<f32>,
    @location(1) corner_radius: vec4<f32>,
    @location(2) position: vec2<f32>,
    @location(3) tex_coords: vec2<f32>,
    @location(4) size: vec2<f32>,
    @location(5) image_index: u32
};

@vertex
fn vs_main(vertex: VertexInput) -> VertexOutput {
    var vertex_output: VertexOutput;

    let rotation = vertex.rotation;

    let rotated = vec2(
        vertex.center.x + (vertex.position.x - vertex.center.x) * cos(rotation) - (vertex.position.y - vertex.center.y) * sin(rotation),
        vertex.center.y + (vertex.position.x - vertex.center.x) * sin(rotation) + (vertex.position.y - vertex.center.y) * cos(rotation)
    );

    vertex_output.color = vertex.color_mask;
    vertex_output.tex_coords = vertex.tex_coords;
    vertex_output.clip_position = camera.view_proj * vec4(rotated, 0.0, 1.0);
    vertex_output.image_index = vertex.image_index;
    vertex_output.corner_radius = vertex.corner_radius;
    vertex_output.size = vertex.size;

    return vertex_output;
}

@group(0) @binding(0)
var textures: texture_2d_array<f32>;

@group(0) @binding(1)
var sample: sampler;

struct Camera {
    view_proj: mat4x4<f32>
}

@group(1) @binding(0)
var<uniform> camera: Camera;

//top left, top right, bottom right, bottom left
fn rounded(position: vec2<f32>, half_size: vec2<f32>, corners: vec4<f32>) -> f32 {
    var q_r: vec2<f32> = select(corners.xy, corners.zw, position.y > 0.0);
    var rad: f32 = select(q_r.y, q_r.x, position.x > 0.0);

    let q = abs(position) - half_size + rad;
    return min(max(q.x, q.y), 0.0) + length(max(q, vec2<f32>(0.0))) - rad;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let local_pos = (in.tex_coords - 0.5) * in.size;

    let d = rounded(local_pos, in.size / 2.0, in.corner_radius);
    let aa = fwidth(d);
    let alpha = smoothstep(0.0, -aa, d);

    return in.color * textureSample(textures, sample, in.tex_coords, in.image_index) * vec4(1.0, 1.0, 1.0, alpha);
}