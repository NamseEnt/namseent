use super::*;
use crate::Uuid;
use wasm_bindgen::{JsCast, JsValue};

#[derive(serde::Serialize)]
pub struct Shader {
    id: Uuid,
    #[serde(skip_serializing)]
    pub(crate) canvas_kit_shader: CanvasKitShader,
}
unsafe impl Send for Shader {}
unsafe impl Sync for Shader {}
impl Shader {
    pub(crate) fn new(canvas_kit_shader: CanvasKitShader) -> Shader {
        Shader {
            id: Uuid::new_v4(),
            canvas_kit_shader,
        }
    }

    pub(crate) fn blend(&self, mode: BlendMode, other: &Shader) -> Shader {
        let shader = canvas_kit().Shader().MakeBlend(
            mode.into_canvas_kit(),
            &self.canvas_kit_shader,
            &other.canvas_kit_shader,
        );
        Shader::new(shader)
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        self.canvas_kit_shader.delete();
    }
}

impl PartialEq for Shader {
    fn eq(&self, other: &Self) -> bool {
        self.canvas_kit_shader.unchecked_ref::<JsValue>()
            == other.canvas_kit_shader.unchecked_ref::<JsValue>()
    }
}

impl std::fmt::Debug for shader::Shader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Shader: {:?}",
            self.canvas_kit_shader.unchecked_ref::<JsValue>()
        )
    }
}
