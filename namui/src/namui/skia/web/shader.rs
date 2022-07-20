use super::*;

pub(crate) struct Shader {
    pub(crate) canvas_kit_shader: CanvasKitShader,
}
unsafe impl Send for Shader {}
unsafe impl Sync for Shader {}
impl Shader {
    pub fn new(canvas_kit_shader: CanvasKitShader) -> Shader {
        Shader { canvas_kit_shader }
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        self.canvas_kit_shader.delete();
    }
}
