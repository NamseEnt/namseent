use super::*;
use namui_type::{BlendMode, Uuid};
use wasm_bindgen::{JsCast, JsValue};

pub struct CkShader {
    id: Uuid,
    pub(crate) canvas_kit_shader: CanvasKitShader,
}
unsafe impl Send for CkShader {}
unsafe impl Sync for CkShader {}
impl CkShader {
    pub(crate) fn new(canvas_kit_shader: CanvasKitShader) -> CkShader {
        CkShader {
            id: Uuid::new_v4(),
            canvas_kit_shader,
        }
    }

    pub(crate) fn blend(&self, mode: BlendMode, other: &CkShader) -> CkShader {
        let shader = canvas_kit().Shader().MakeBlend(
            mode.into(),
            &self.canvas_kit_shader,
            &other.canvas_kit_shader,
        );
        CkShader::new(shader)
    }
}

impl Drop for CkShader {
    fn drop(&mut self) {
        self.canvas_kit_shader.delete();
    }
}

impl PartialEq for CkShader {
    fn eq(&self, other: &Self) -> bool {
        self.canvas_kit_shader.unchecked_ref::<JsValue>()
            == other.canvas_kit_shader.unchecked_ref::<JsValue>()
    }
}

impl std::fmt::Debug for shader::CkShader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Shader: {:?}",
            self.canvas_kit_shader.unchecked_ref::<JsValue>()
        )
    }
}
