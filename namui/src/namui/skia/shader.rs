/// Please use shader_macro::shader to make shader.
pub trait Shader {
    fn uniforms(&self) -> &[f32];
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::namui;
    use shader_macro::shader;
    use wasm_bindgen_test::wasm_bindgen_test;

    #[test]
    #[wasm_bindgen_test]
    fn shader_should_return_uniforms() {
        shader!(MyShader, {
            uniform float rad_scale;
            uniform float2 in_center;
            uniform float4 in_colors0;
            uniform float4 in_colors1;

            half4 main(float2 p) {
                float2 pp = p - in_center;
                float radius = sqrt(dot(pp, pp));
                radius = sqrt(radius);
                float angle = atan(pp.y / pp.x);
                float t = (angle + 3.1415926/2) / (3.1415926);
                t += radius * rad_scale;
                t = fract(t);
                return half4(mix(in_colors0, in_colors1, t));
            }
        });

        let shader = MyShader::new(
            1.0,
            [2.0, 3.0],
            [4.0, 5.0, 6.0, 7.0],
            [8.0, 9.0, 10.0, 11.0],
        );

        let uniforms = shader.uniforms();

        assert_eq!(uniforms.len(), 11);
        assert_eq!(
            uniforms,
            [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0]
        );
    }
}
