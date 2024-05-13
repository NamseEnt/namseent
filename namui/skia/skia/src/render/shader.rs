use crate::*;

#[type_derives()]
pub enum Shader {
    Image {
        src: Image,
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
