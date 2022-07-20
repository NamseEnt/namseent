mod token_consume;

use core::panic;

use proc_macro::*;
use token_consume::*;

///
/// # Warning for namui developer
/// If you want to use shader in namui, please `use crate::namui` before using this macro.
/// Because this macro is using `namui::MakeShader` trait, so to import namui, you need to use `use crate::namui`.
///
/// # Example
/// ```
/// // macro call
/// shader!(MyShader, {
///     uniform float rad_scale;
///     uniform float2 in_center;
///     uniform float4 in_colors0;
///     uniform float4 in_colors1;
///
///     half4 main(float2 p) {
///         float2 pp = p - in_center;
///         float radius = sqrt(dot(pp, pp));
///         radius = sqrt(radius);
///         float angle = atan(pp.y / pp.x);
///         float t = (angle + 3.1415926/2) / (3.1415926);
///         t += radius * rad_scale;
///         t = fract(t);
///         return half4(mix(in_colors0, in_colors1, t));
///     }
/// });
///
/// // output
/// pub struct MyShader {
///     code: String,
///     pub rad_scale: f32,
///     pub in_center: [f32; 2],
///     pub in_colors0: [f32; 4],
///     pub in_colors1: [f32; 4],
/// }
/// impl MyShader {
///     pub fn new(red_scale: f32, in_center: [f32; 2], in_colors0: [f32; 4], in_colors1: [f32; 4]) -> Self {
///         MyShader {
///             rad_scale,
///             in_center,
///             in_colors0,
///             in_colors1,
///         }
///     }
///     fn uniforms(&self) -> [f32; 11] {
///         [
///             rad_scale,
///             in_center[0],
///             in_center[1],
///             in_colors0[0],
///             in_colors0[1],
///             in_colors0[2],
///             in_colors0[3],
///             in_colors1[0],
///             in_colors1[1],
///             in_colors1[2],
///             in_colors1[3]
///         ]
///     }
///     fn code(&self) -> &'static str {
///         "
///             uniform float rad_scale;
///             uniform float2 in_center;
///             uniform float4 in_colors0;
///             uniform float4 in_colors1;
///
///             half4 main(float2 p) {
///                 float2 pp = p - in_center;
///                 float radius = sqrt(dot(pp, pp));
///                 radius = sqrt(radius);
///                 float angle = atan(pp.y / pp.x);
///                 float t = (angle + 3.1415926/2) / (3.1415926);
///                 t += radius * rad_scale;
///                 t = fract(t);
///                 return half4(mix(in_colors0, in_colors1, t));
///             }
///        "
///     }
/// }
/// impl namui::MakeShader for MyShader {
///     fn make(&self) -> namui::IntermediateShader {
///         namui::IntermediateShader::new(&self.uniforms(), self.code())
///     }
/// }
///
/// // example
/// let shader = MyShader::make(1.0, [0.0, 0.0], [1.0, 0.0, 0.0, 1.0], [0.0, 1.0, 0.0, 1.0]);
/// ```
///
#[proc_macro]
pub fn shader(item: TokenStream) -> TokenStream {
    let mut iter = item.into_iter();
    let shader_ident = iter.consume_any_ident();
    iter.consume_punct(',');

    let mut uniforms = Vec::new();

    let mut main_func_appear_count = 0;

    let sksl_code_group = iter.consume_group(|group| {
        let mut iter = group.stream().into_iter();

        while let Some(ident) = iter.try_consume_any_ident() {
            match ident.to_string().as_str() {
                "uniform" => {
                    let uniform_type = iter.consume_any_ident();
                    let uniform_ident = iter.consume_any_ident();
                    iter.consume_punct(';');

                    uniforms.push(Uniform {
                        ident: uniform_ident.to_string(),
                        ty: match uniform_type.to_string().as_str() {
                            "float" => UniformType::Float,
                            "float2" => UniformType::Float2,
                            "float3" => UniformType::Float3,
                            "float4" => UniformType::Float4,
                            _ => panic!("Unsupported uniform type: {}", uniform_type.to_string()),
                        },
                    });
                }
                _ => {
                    let _func_return_type = ident;
                    let func_ident = iter.consume_any_ident();
                    if func_ident.to_string() == "main" {
                        main_func_appear_count += 1;
                    }
                    iter.consume_any_group();
                    iter.consume_any_group();
                }
            }
        }
    });

    let group_string = sksl_code_group.to_string();
    let sksl_code = &group_string[1..group_string.len() - 1];

    if main_func_appear_count != 1 {
        panic!("Shader must have one main function");
    }
    let uniform_f32_size: usize = uniforms.iter().map(|uniform| uniform.ty.f32_size()).sum();

    let struct_fields = uniforms
        .iter()
        .map(|uniform| format!("{}: {},", uniform.ident, uniform.ty.to_rust_type()))
        .collect::<Vec<_>>()
        .join("\n");

    let new_params = uniforms
        .iter()
        .map(|uniform| format!("{}: {}", uniform.ident, uniform.ty.to_rust_type()))
        .collect::<Vec<_>>()
        .join(", ");

    let new_body = {
        uniforms
            .iter()
            .map(|uniform| format!("{}", uniform.ident))
            .collect::<Vec<_>>()
            .join(",\n")
    };

    let uniforms_body = uniforms
        .iter()
        .map(|uniform| match uniform.ty {
            UniformType::Float => format!("self.{}", uniform.ident),
            UniformType::Float2 => format!("self.{}[0], self.{}[1]", uniform.ident, uniform.ident),
            UniformType::Float3 => format!(
                "self.{}[0], self.{}[1], self.{}[2]",
                uniform.ident, uniform.ident, uniform.ident
            ),
            UniformType::Float4 => format!(
                "self.{}[0], self.{}[1], self.{}[2], self.{}[3]",
                uniform.ident, uniform.ident, uniform.ident, uniform.ident
            ),
        })
        .collect::<Vec<_>>()
        .join(",\n");

    let result = format!(
        "
#[derive(Debug, Clone)]
pub struct {shader_ident} {{
    {struct_fields}
}}
impl {shader_ident} {{
    pub fn new({new_params}) -> Self {{
        {shader_ident} {{
            {new_body}
        }}
    }}
    fn uniforms(&self) -> [f32; {uniform_f32_size}] {{
        [
            {uniforms_body}
        ]
    }}
    fn code() -> &'static str {{
        \"{sksl_code}\"
    }}
}}
impl namui::MakeShader for {shader_ident} {{
    fn make(&self) -> namui::IntermediateShader {{
        namui::IntermediateShader::new(&self.uniforms(), Self::code())
    }}
}}
"
    );

    result.parse().unwrap()
}

enum UniformType {
    Float,
    Float2,
    Float3,
    Float4,
}
impl UniformType {
    fn to_rust_type(&self) -> String {
        match self {
            UniformType::Float => "f32".to_string(),
            UniformType::Float2 => "[f32; 2]".to_string(),
            UniformType::Float3 => "[f32; 3]".to_string(),
            UniformType::Float4 => "[f32; 4]".to_string(),
        }
    }
    fn f32_size(&self) -> usize {
        match self {
            UniformType::Float => 1,
            UniformType::Float2 => 2,
            UniformType::Float3 => 3,
            UniformType::Float4 => 4,
        }
    }
}
struct Uniform {
    ident: String,
    ty: UniformType,
}
