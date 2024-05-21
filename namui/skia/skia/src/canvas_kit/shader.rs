use super::*;
use crate::*;
use std::sync::Arc;

pub struct CkShader {
    pub(crate) canvas_kit_shader: CanvasKitShader,
}

unsafe impl Send for CkShader {}
unsafe impl Sync for CkShader {}
impl CkShader {
    pub(crate) fn get(shader: &Shader) -> Arc<Self> {
        static CK_SHADER_MAP: SerdeMap<Shader, CkShader> = SerdeMap::new();

        CK_SHADER_MAP.get_or_create(shader, {
            |shader| match shader {
                Shader::Image { src } => CkShader {
                    canvas_kit_shader: src.ck_image.canvas_kit().makeShaderOptions(
                        TileMode::Clamp.into(),
                        TileMode::Clamp.into(),
                        FilterMode::Linear.into(),
                        MipmapMode::Linear.into(),
                        None,
                    ),
                },
                &Shader::Blend {
                    blend_mode,
                    ref src,
                    ref dest,
                } => {
                    let ck_src = CkShader::get(src);
                    let ck_dest = CkShader::get(dest);

                    let blended = canvas_kit().Shader().MakeBlend(
                        blend_mode.into(),
                        &ck_src.canvas_kit_shader,
                        &ck_dest.canvas_kit_shader,
                    );
                    CkShader {
                        canvas_kit_shader: blended,
                    }
                }
                &Shader::LinearGradient {
                    start_xy,
                    end_xy,
                    ref colors,
                    tile_mode,
                } => {
                    let colors: Vec<js_sys::Float32Array> = colors
                        .iter()
                        .map(|color| color.to_float32_array())
                        .collect();

                    let shader = canvas_kit().Shader().MakeLinearGradient(
                        &[start_xy.x.as_f32(), start_xy.y.as_f32()],
                        &[end_xy.x.as_f32(), end_xy.y.as_f32()],
                        colors, // colors: Vec<Float32Array>,
                        None,
                        tile_mode.into(),
                        None,
                        None,
                        None,
                    );

                    CkShader {
                        canvas_kit_shader: shader,
                    }
                }
            }
        })
    }

    pub(crate) fn canvas_kit(&self) -> &CanvasKitShader {
        &self.canvas_kit_shader
    }
}

impl Drop for CkShader {
    fn drop(&mut self) {
        self.canvas_kit_shader.delete();
    }
}
