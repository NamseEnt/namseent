use super::*;
use std::sync::Arc;

pub struct CkShader {
    pub(crate) canvas_kit_shader: CanvasKitShader,
}

unsafe impl Send for CkShader {}
unsafe impl Sync for CkShader {}
impl CkShader {
    pub(crate) fn get(shader: &Shader) -> Arc<Self> {
        static CK_SHADER_MAP: SerdeMap<Shader, CkShader> = SerdeMap::new();

        CK_SHADER_MAP.get_or_create(shader, |shader| match shader {
            Shader::Image { src } => {
                let ck_image = CkImage::get(src).unwrap();
                CkShader {
                    canvas_kit_shader: ck_image.canvas_kit().makeShaderOptions(
                        TileMode::Clamp.into(),
                        TileMode::Clamp.into(),
                        FilterMode::Linear.into(),
                        MipmapMode::Linear.into(),
                        None,
                    ),
                }
            }
            Shader::Blend {
                blend_mode,
                src,
                dest,
            } => {
                let ck_src = CkShader::get(src);
                let ck_dest = CkShader::get(dest);

                let blended = canvas_kit().Shader().MakeBlend(
                    (*blend_mode).into(),
                    &ck_src.canvas_kit_shader,
                    &ck_dest.canvas_kit_shader,
                );
                CkShader {
                    canvas_kit_shader: blended,
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
