use super::*;

pub(crate) struct RuntimeEffect {
    canvas_kit_runtime_effect: CanvasKitRuntimeEffect,
}
unsafe impl Send for RuntimeEffect {}
unsafe impl Sync for RuntimeEffect {}
impl RuntimeEffect {
    pub fn new(code: &str) -> RuntimeEffect {
        RuntimeEffect {
            canvas_kit_runtime_effect: canvas_kit().RuntimeEffect().Make(code).unwrap(),
        }
    }
    pub fn make_shader(&self, uniforms: &[f32]) -> Shader {
        Shader::new(self.canvas_kit_runtime_effect.makeShader(uniforms))
    }
}

impl Drop for RuntimeEffect {
    fn drop(&mut self) {
        self.canvas_kit_runtime_effect.delete();
    }
}
