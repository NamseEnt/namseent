use super::{RuntimeEffect, Shader};
use once_cell::sync::OnceCell;
use ordered_float::OrderedFloat;
use std::{
    hash::Hash,
    sync::{Arc, Mutex},
};

/// Please use shader_macro::shader to make shader.

/// This is namui internal shader.
#[derive(Clone, PartialEq)]
struct ShaderCacheKey {
    uniforms: Box<[f32]>,
    code: &'static str,
    children: Box<[Arc<Shader>]>,
}

impl Hash for ShaderCacheKey {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        for uniform in self.uniforms.iter() {
            OrderedFloat(*uniform).hash(state);
        }
        self.code.hash(state);
    }
}
impl Eq for ShaderCacheKey {}

static RUNTIME_EFFECT_CACHE: OnceCell<Mutex<lru::LruCache<&str, Arc<RuntimeEffect>>>> =
    OnceCell::new();
static SHADER_CACHE: OnceCell<Mutex<lru::LruCache<ShaderCacheKey, Arc<Shader>>>> = OnceCell::new();

pub fn make_runtime_effect_shader(
    uniforms: &[f32],
    code: &'static str,
    children: Box<[Arc<Shader>]>,
) -> Arc<Shader> {
    let cache_key = ShaderCacheKey {
        uniforms: Box::from(uniforms),
        code,
        children,
    };
    if let Some(shader) = cache_key.try_get_shader_cache() {
        return shader;
    }

    let runtime_effect = match cache_key.try_get_runtime_effect_cache() {
        Some(runtime_effect) => runtime_effect,
        None => {
            let runtime_effect = Arc::new(RuntimeEffect::new(cache_key.code));
            cache_key.put_runtime_effect_cache(runtime_effect.clone());
            runtime_effect
        }
    };

    let shader =
        Arc::new(runtime_effect.make_shader(&cache_key.uniforms, cache_key.children.iter()));
    cache_key.put_shader_cache(shader.clone());
    shader
}

impl ShaderCacheKey {
    fn try_get_shader_cache(&self) -> Option<Arc<Shader>> {
        SHADER_CACHE
            .get_or_init(|| Mutex::new(lru::LruCache::new(1024)))
            .lock()
            .unwrap()
            .get(self)
            .map(|shader| shader.clone())
    }

    fn try_get_runtime_effect_cache(&self) -> Option<Arc<RuntimeEffect>> {
        RUNTIME_EFFECT_CACHE
            .get_or_init(|| Mutex::new(lru::LruCache::new(1024)))
            .lock()
            .unwrap()
            .get(&self.code)
            .map(|runtime_effect| runtime_effect.clone())
    }

    fn put_shader_cache(&self, shader: Arc<Shader>) {
        SHADER_CACHE
            .get_or_init(|| Mutex::new(lru::LruCache::new(1024)))
            .lock()
            .unwrap()
            .put(self.clone(), shader);
    }

    fn put_runtime_effect_cache(&self, runtime_effect: Arc<RuntimeEffect>) {
        RUNTIME_EFFECT_CACHE
            .get_or_init(|| Mutex::new(lru::LruCache::new(1024)))
            .lock()
            .unwrap()
            .put(&self.code, runtime_effect.clone());
    }
}

#[cfg(test)]
mod tests {
    use shader_macro::shader;
    use wasm_bindgen_test::wasm_bindgen_test;

    #[test]
    #[wasm_bindgen_test]
    fn shader_should_return_uniforms() {
        use crate::namui;
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
