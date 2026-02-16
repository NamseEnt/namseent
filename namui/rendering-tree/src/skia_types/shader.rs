use crate::*;
use std::sync::Arc;

struct NativeRuntimeEffect(skia_safe::RuntimeEffect);
unsafe impl Send for NativeRuntimeEffect {}
unsafe impl Sync for NativeRuntimeEffect {}

impl std::ops::Deref for NativeRuntimeEffect {
    type Target = skia_safe::RuntimeEffect;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub struct NativeShader {
    pub skia_shader: skia_safe::Shader,
}
unsafe impl Send for NativeShader {}
unsafe impl Sync for NativeShader {}

struct DefaultRuntimeEffectOptions;

impl<'a, 'b> From<DefaultRuntimeEffectOptions>
    for Option<&'a skia_safe::runtime_effect::Options<'b>>
{
    fn from(_: DefaultRuntimeEffectOptions) -> Self {
        None
    }
}
impl NativeShader {
    pub fn get(shader: &Shader) -> Arc<Self> {
        static NATIVE_SHADER_MAP: LruCache<Shader, NativeShader, 64> = LruCache::new();

        NATIVE_SHADER_MAP.get_or_create(shader, |shader| match shader {
            Shader::Image { src, tile_mode } => NativeShader {
                skia_shader: src
                    .skia_image()
                    .to_shader(
                        Some((tile_mode.x.into(), tile_mode.y.into())),
                        skia_safe::SamplingOptions::new(
                            FilterMode::Linear.into(),
                            MipmapMode::Linear.into(),
                        ),
                        None,
                    )
                    .expect("Failed to create shader from image"),
            },
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
            &Shader::LinearGradient {
                start_xy,
                end_xy,
                ref colors,
                tile_mode,
            } => {
                let colors: Vec<_> = colors
                    .iter()
                    .map(|color| skia_safe::Color::from(*color))
                    .collect();

                let shader = skia_safe::gradient_shader::linear(
                    (
                        skia_safe::Point::new(start_xy.x.into(), start_xy.y.into()),
                        skia_safe::Point::new(end_xy.x.into(), end_xy.y.into()),
                    ),
                    skia_safe::gradient_shader::GradientShaderColors::Colors(colors.as_slice()),
                    None,
                    tile_mode.into(),
                    None,
                    None,
                )
                .unwrap();
                NativeShader {
                    skia_shader: shader,
                }
            }
            Shader::RuntimeEffect {
                sksl,
                uniforms,
                children,
            } => {
                // RuntimeEffect 컴파일 결과를 sksl 기반으로 캐싱 (무거운 작업)
                static RUNTIME_EFFECT_SHADER_MAP: LruCache<Arc<str>, NativeRuntimeEffect, 32> =
                    LruCache::new();

                // sksl을 Arc<str>로 변환하여 캐시 키로 사용
                let sksl_key: Arc<str> = Arc::from(sksl.as_str());
                let effect = RUNTIME_EFFECT_SHADER_MAP.get_or_create(&sksl_key, |sksl_str| {
                    NativeRuntimeEffect(
                        skia_safe::RuntimeEffect::make_for_shader(
                            sksl_str,
                            DefaultRuntimeEffectOptions,
                        )
                        .expect("Failed to compile runtime shader"),
                    )
                });

                // children shader 변환 (가벼운 작업)
                let child_ptrs = children
                    .iter()
                    .map(|child| {
                        let native_child = NativeShader::get(child);
                        skia_safe::runtime_effect::ChildPtr::from(native_child.skia().clone())
                    })
                    .collect::<Vec<_>>();

                // 컴파일된 effect로 shader 생성 (가벼운 작업)
                let shader = effect
                    .make_shader(
                        skia_safe::Data::new_copy(uniforms.as_slice()),
                        &child_ptrs,
                        None,
                    )
                    .expect("Failed to create shader from runtime effect");

                NativeShader {
                    skia_shader: shader,
                }
            }
        })
    }

    pub fn skia(&self) -> &skia_safe::Shader {
        &self.skia_shader
    }
}
