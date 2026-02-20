/*
    uniform inputs:
    - Orientation
    - global offset
    inputs:
    - buffer of quads (right now just offsets)
*/

@group(0) @binding(0) var<uniform> offset: vec3<f32>;
@group(0) @binding(1) var<uniform> orientation: u32;

@vertex
fn vs_main(
    @location(0) position: vec3<f32>,
    @location(1) quad_offset: vec3<f32>,
) -> @builtin(position) vec4<f32> {
    let rotated_position = vec4(offset+ position + quad_offset, 1.0);
    return rotated_position;
}
