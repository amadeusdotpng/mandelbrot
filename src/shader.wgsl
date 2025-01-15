struct VertexInput {
    @location(0) position: vec3<f32>,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) coords: vec2<f32>,
}


@vertex
fn vs_main(vertex: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    out.clip_position = vec4<f32>(vertex.position, 1.0);
    out.coords = vertex.position.xy;

    return out;
}

struct CameraUniform {
    view_mat: mat3x3<f32>
}
@group(0) @binding(0)
var<uniform> camera: CameraUniform;

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    var z = vec2<f32>(0.0, 0.0);
    let c = camera.view_mat * vec3<f32>(in.coords, 1.0);

    var lim = 1000;
    for(var i = 0; i < lim; i++) {
        let z_prime = vec2<f32>(
            (z.x * z.x - z.y * z.y) + c.x,
            (2 * z.x * z.y) + c.y,
        );

        if(z_prime.x*z_prime.x + z_prime.y*z_prime.y >= 4) {
            return vec4<f32>(vec3<f32>(f32(i) / f32(lim)), 1.0);
        }
        z = z_prime;
    }
    return vec4<f32>(vec3<f32>(0.0), 1.0);
}
