use crate::*;
use std::hash::Hash;

#[derive(Debug, PartialEq, Clone, Eq, State)]
pub enum Shader {
    Image {
        src: Image,
        tile_mode: Xy<TileMode>,
    },
    Blend {
        blend_mode: BlendMode,
        src: Box<Shader>,
        dest: Box<Shader>,
    },
    LinearGradient {
        start_xy: Xy<Px>,
        end_xy: Xy<Px>,
        colors: Vec<Color>,
        tile_mode: TileMode,
    },
    RuntimeEffect {
        sksl: String,
        uniforms: Vec<u8>,
        children: Vec<Shader>,
    },
}

impl Shader {
    pub fn blend(&self, blend_mode: BlendMode, shader: &Shader) -> Shader {
        Shader::Blend {
            blend_mode,
            src: Box::new(self.clone()),
            dest: Box::new(shader.clone()),
        }
    }

    pub fn runtime_effect(
        uniforms: &[f32],
        sksl: &str,
        children: Box<[std::sync::Arc<Shader>]>,
    ) -> std::sync::Arc<Shader> {
        let mut uniform_bytes = Vec::with_capacity(uniforms.len() * 4);
        for value in uniforms {
            uniform_bytes.extend_from_slice(&value.to_le_bytes());
        }

        let child_shaders = children
            .into_vec()
            .into_iter()
            .map(|child| (*child).clone())
            .collect::<Vec<_>>();

        std::sync::Arc::new(Shader::RuntimeEffect {
            sksl: sksl.to_string(),
            uniforms: uniform_bytes,
            children: child_shaders,
        })
    }
}

impl Hash for Shader {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self {
            Shader::Image { src, tile_mode } => {
                src.hash(state);
                tile_mode.hash(state);
            }
            Shader::Blend {
                blend_mode,
                src,
                dest,
            } => {
                blend_mode.hash(state);
                src.hash(state);
                dest.hash(state);
            }
            Shader::LinearGradient {
                start_xy,
                end_xy,
                colors,
                tile_mode,
            } => {
                start_xy.hash(state);
                end_xy.hash(state);
                colors.hash(state);
                tile_mode.hash(state);
            }
            Shader::RuntimeEffect {
                sksl,
                uniforms,
                children,
            } => {
                sksl.hash(state);
                uniforms.hash(state);
                children.hash(state);
            }
        }
    }
}

impl From<std::sync::Arc<Shader>> for Shader {
    fn from(value: std::sync::Arc<Shader>) -> Self {
        (*value).clone()
    }
}
