use crate::*;

impl Draw for &PathDrawCommand {
    fn draw(self, skia: &mut NativeSkia) {
        skia.surface().canvas().draw_path(&self.path, &self.paint);
    }
}
