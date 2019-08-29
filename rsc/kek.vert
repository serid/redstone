#version 450

layout(push_constant) uniform PushConstantData {
    vec2 win_ratio;
    float scale;
} pc;

// The triangle vertex positions.
layout(location = 0) in vec2 position;

// The per-instance data.
layout(location = 1) in vec2 position_offset;
layout(location = 2) in mat2 rot;
layout(location = 4) in uint tex_shift;

layout(location = 0) out vec2 tex_coords;

void main() {
    gl_Position = vec4(position * vec2(pc.scale) * rot / pc.win_ratio + position_offset, 0.0, 1.0);

    //tex_coords = position + vec2(0.5);
    tex_coords = (position * vec2(0.5) + vec2(0.5)) * vec2(1.0 / 4.0, 1.0) + vec2(float(tex_shift) / 4.0, 0.0);
}
