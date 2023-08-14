use crate::*;

#[type_derives()]
pub enum Shader {
    Image {
        src: ImageSource,
    },
    Blend {
        blend_mode: BlendMode,
        src: Box<Shader>,
        dest: Box<Shader>,
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
