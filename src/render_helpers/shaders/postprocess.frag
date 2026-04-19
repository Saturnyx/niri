uniform float noise;
uniform float saturation;
uniform vec4 bg_color;

<<<<<<< HEAD
// Interleaved Gradient Noise
float gradient_noise(vec2 uv) {
    const vec3 magic = vec3(0.06711056, 0.00583715, 52.9829189);
    return fract(magic.z * fract(dot(uv, magic.xy)));
=======
// Sin-less white noise by David Hoskins (MIT License).
// https://www.shadertoy.com/view/4djSRW
float hash12(vec2 p) {
    vec3 p3 = fract(vec3(p.xyx) * 0.1031);
    p3 += dot(p3, p3.yzx + 33.33);
    return fract((p3.x + p3.y) * p3.z);
>>>>>>> upstream/main
}

vec3 saturate(vec3 color, float sat) {
    const vec3 w = vec3(0.2126, 0.7152, 0.0722);
    return mix(vec3(dot(color, w)), color, sat);
}

vec4 postprocess(vec4 color) {
    if (saturation != 1.0) {
        color.rgb = saturate(color.rgb, saturation);
    }

    if (noise > 0.0) {
        vec2 uv = gl_FragCoord.xy;
<<<<<<< HEAD
        color.rgb += (gradient_noise(uv) - 0.5) * noise;
=======
        color.rgb += (hash12(uv) - 0.5) * noise;
>>>>>>> upstream/main
    }

    // Mix bg_color behind the texture (both premultiplied alpha).
    color = color + bg_color * (1.0 - color.a);

    return color;
}
