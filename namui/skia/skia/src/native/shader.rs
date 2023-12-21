use super::*;
use std::sync::Arc;

pub struct NativeShader {
    pub(crate) skia_shader: skia_safe::Shader,
}

unsafe impl Send for NativeShader {}
unsafe impl Sync for NativeShader {}
impl NativeShader {
    pub(crate) fn get(shader: &Shader) -> Arc<Self> {
        static NATIVE_SHADER_MAP: SerdeLruCache<Shader, NativeShader, 64> = SerdeLruCache::new();

        NATIVE_SHADER_MAP.get_or_create(shader, |shader| match shader {
            Shader::Image { src } => {
                let native_image = NativeImage::get(src).unwrap();
                NativeShader {
                    skia_shader: native_image
                        .skia()
                        .to_shader(
                            Some((TileMode::Clamp.into(), TileMode::Clamp.into())),
                            skia_safe::SamplingOptions::new(
                                FilterMode::Linear.into(),
                                MipmapMode::Linear.into(),
                            ),
                            None,
                        )
                        .expect("Failed to create shader from image"),
                }
            }
            Shader::Blend {
                blend_mode,
                src,
                dest,
            } => {
                let native_src = NativeShader::get(src);
                let native_dest = NativeShader::get(dest);

                let blended = skia_safe::shaders::blend(
                    skia_safe::BlendMode::from(*blend_mode),
                    &native_src.skia_shader,
                    &native_dest.skia_shader,
                );
                NativeShader {
                    skia_shader: blended,
                }
            }
        })
    }

    pub(crate) fn skia(&self) -> &skia_safe::Shader {
        &self.skia_shader
    }
}
