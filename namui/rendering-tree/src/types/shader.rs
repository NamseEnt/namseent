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
}

impl Shader {
    pub fn blend(&self, blend_mode: BlendMode, shader: &Shader) -> Shader {
        Shader::Blend {
            blend_mode,
            src: Box::new(self.clone()),
            dest: Box::new(shader.clone()),
        }
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
        }
    }
}
