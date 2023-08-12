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
            Shader::Image { src, dest_rect } => {
                let ck_image = CkImage::get(src).unwrap();
                crate::log!("new image shader");
                // let matrix = canvas_kit().Matrix().scaled(
                //     dest_rect.width().as_f32(),
                //     dest_rect.height().as_f32(),
                //     None,
                //     None,
                // );
                // let matrix = canvas_kit()
                //     .Matrix()
                //     .translated(-dest_rect.x().as_f32(), -dest_rect.y().as_f32());
                let matrix =
                    Matrix3x3::from_translate(dest_rect.x().as_f32(), dest_rect.y().as_f32())
                        * Matrix3x3::from_scale(
                            dest_rect.width().as_f32() / ck_image.size().width.as_f32() * 4.0,
                            dest_rect.height().as_f32() / ck_image.size().height.as_f32() * 4.0,
                        );
                // crate::log!("matrix: {:?}", matrix);
                CkShader {
                    canvas_kit_shader: ck_image.canvas_kit().makeShaderOptions(
                        TileMode::Clamp.into(),
                        TileMode::Clamp.into(),
                        FilterMode::Linear.into(),
                        MipmapMode::Linear.into(),
                        &matrix.into_linear_slice(),
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
