use crate::*;
use namui_type::*;

impl XyIn for DrawCommand {
    fn xy_in(&self, xy: Xy<Px>) -> bool {
        match self {
            DrawCommand::Path { command } => command.xy_in(xy),
            DrawCommand::Text { command } => command.xy_in(xy),
            DrawCommand::Image { command } => command.xy_in(xy),
            DrawCommand::Atlas { command } => command.xy_in(xy),
        }
    }
}
impl XyIn for PathDrawCommand {
    fn xy_in(&self, xy: Xy<Px>) -> bool {
        NativePath::get(&self.path).contains(Some(&self.paint), xy)
    }
}
impl XyIn for TextDrawCommand {
    fn xy_in(&self, xy: Xy<Px>) -> bool {
        self.bounding_box().is_some_and(|x| x.is_xy_inside(xy))
    }
}

impl XyIn for ImageDrawCommand {
    fn xy_in(&self, xy: Xy<Px>) -> bool {
        let path = Path::new().add_rect(self.rect);
        NativePath::get(&path).contains(self.paint.as_ref(), xy)
    }
}

impl XyIn for AtlasDrawCommand {
    fn xy_in(&self, xy: Xy<Px>) -> bool {
        // Check if the point is inside any of the transformed sprites
        for sprite in &self.sprites {
            let tex = &sprite.tex;
            let xform = &sprite.xform;

            // Get the sprite dimensions
            let w = tex.width();
            let h = tex.height();

            // Inverse transform the test point to sprite-local coordinates
            // RSXform: x' = scos * x - ssin * y + tx
            //          y' = ssin * x + scos * y + ty
            // Inverse: x = (scos * (x' - tx) + ssin * (y' - ty)) / (scos² + ssin²)
            //          y = (-ssin * (x' - tx) + scos * (y' - ty)) / (scos² + ssin²)
            let det = xform.scos * xform.scos + xform.ssin * xform.ssin;
            if det == 0.0 {
                continue;
            }

            let dx = xy.x - xform.tx;
            let dy = xy.y - xform.ty;

            let local_x = (dx * xform.scos + dy * xform.ssin) / det;
            let local_y = (dx * (-xform.ssin) + dy * xform.scos) / det;

            // Check if the local point is within the sprite bounds
            if local_x >= px(0.0) && local_x <= w && local_y >= px(0.0) && local_y <= h {
                return true;
            }
        }
        false
    }
}
