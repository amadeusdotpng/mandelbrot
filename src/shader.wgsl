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

struct Complex {
    real: f32,
    imag: f32,
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    var z: Complex;
    z.real = 0.0;
    z.imag = 0.0;

    var c: Complex;
    c.real = in.coords.x * 2 - 0.5;
    c.imag = in.coords.y * 2;

    var lim = 500;
    for(var i = 0; i < lim; i++) {
        var z_prime: Complex;
        z_prime.real = (z.real * z.real - z.imag * z.imag) + c.real;
        z_prime.imag = (2 * z.real * z.imag) + c.imag;

        z = z_prime;
        if(sqrt(z.real*z.real + z.imag*z.imag) >= 2) {
            return vec4<f32>(vec3<f32>(f32(i) / f32(lim)), 1.0);
        }
    }
    return vec4<f32>(vec3<f32>(0.0), 1.0);
}
