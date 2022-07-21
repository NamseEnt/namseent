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
    pub fn make_shader(
        &self,
        uniforms: &[f32],
        children: impl IntoIterator<Item = impl AsRef<Shader>>,
    ) -> Shader {
        let children = children
            .into_iter()
            .map(|child| child.as_ref().canvas_kit_shader.clone())
            .collect();
        Shader::new(self.canvas_kit_runtime_effect.makeShaderWithChildren(
            uniforms,
            Option::None,
            Some(children),
        ))
    }
}

impl Drop for RuntimeEffect {
    fn drop(&mut self) {
        self.canvas_kit_runtime_effect.delete();
    }
}
