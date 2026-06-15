struct VertexInput {
    @location(0) color: vec4<f32>,
    @location(1) border_color: vec4<f32>,
    @location(2) corner_radius: vec4<f32>,
    @location(3) position: vec2<f32>,
    @location(4) center: vec2<f32>,
    @location(5) size: vec2<f32>,
    @location(6) tex_coords: vec2<f32>,
    @location(7) rotation: f32,
    @location(8) border_thickness: f32
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec4<f32>,
    @location(1) border_color: vec4<f32>,
    @location(2) corner_radius: vec4<f32>,
    @location(3) position: vec2<f32>,
    @location(4) tex_coords: vec2<f32>,
    @location(5) size: vec2<f32>,
    @location(6) border_thickness: f32
};

@vertex
fn vs_main(vertex: VertexInput) -> VertexOutput {
    var vertex_output: VertexOutput;

    let rotation = vertex.rotation;

    let rotated = vec2(
        vertex.center.x + (vertex.position.x - vertex.center.x) * cos(rotation) - (vertex.position.y - vertex.center.y) * sin(rotation),
        vertex.center.y + (vertex.position.x - vertex.center.x) * sin(rotation) + (vertex.position.y - vertex.center.y) * cos(rotation)
    );

    vertex_output.color = vertex.color;
    vertex_output.tex_coords = vertex.tex_coords;
    vertex_output.clip_position = camera.view_proj * vec4(rotated, 0.0, 1.0);
    vertex_output.border_color = vertex.border_color;
    vertex_output.corner_radius = vertex.corner_radius;
    vertex_output.size = vertex.size;
    vertex_output.border_thickness = vertex.border_thickness;

    return vertex_output;
}

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
    if length(in.corner_radius) > 0.0 {
        let local_pos = (in.tex_coords - 0.5) * in.size;
        let d = rounded(local_pos, in.size / 2.0, in.corner_radius);
        let aa = fwidth(d);
        
        let outer_alpha = smoothstep(0.0, -aa, d);
        let inner_alpha = smoothstep(0.0, -aa, d + in.border_thickness);
        
        let mixed_color = mix(in.border_color, in.color, inner_alpha);
        return vec4(mixed_color.rgb, mixed_color.a * outer_alpha);
    }
    else {
        return vec4(in.color);
    }
}