use crate::*;

pub struct CkRuntimeEffect {
    canvas_kit_runtime_effect: CanvasKitRuntimeEffect,
}
unsafe impl Send for CkRuntimeEffect {}
unsafe impl Sync for CkRuntimeEffect {}
impl CkRuntimeEffect {
    pub fn new(code: &str) -> CkRuntimeEffect {
        CkRuntimeEffect {
            canvas_kit_runtime_effect: canvas_kit().RuntimeEffect().Make(code).unwrap(),
        }
    }
    pub fn make_shader(
        &self,
        uniforms: &[f32],
        children: impl IntoIterator<Item = impl AsRef<CkShader>>,
    ) -> CkShader {
        todo!()
        // let children = children
        //     .into_iter()
        //     .map(|child| child.as_ref().canvas_kit_shader.clone())
        //     .collect();
        // CkShader::new(
        //     self.canvas_kit_runtime_effect
        //         .makeShaderWithChildren(uniforms, Some(children)),
        // )
    }
}

impl Drop for CkRuntimeEffect {
    fn drop(&mut self) {
        self.canvas_kit_runtime_effect.delete();
    }
}
