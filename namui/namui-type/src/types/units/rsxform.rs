use crate::*;

/// RSXform represents a rectangle-to-rectangle transformation.
/// It combines scale, rotation, and translation into a compact form.
///
/// The transformation is defined as:
/// - Scale and rotate: scos = scale * cos(radians), ssin = scale * sin(radians)  
/// - Translation: (tx, ty)
///
/// A point (x, y) is transformed to:
/// - x' = scos * x - ssin * y + tx
/// - y' = ssin * x + scos * y + ty
#[derive(Debug, Clone, Copy, PartialEq, State)]
pub struct RSXform {
    /// Scale factor times cosine of rotation angle
    pub scos: f32,
    /// Scale factor times sine of rotation angle
    pub ssin: f32,
    /// X translation
    pub tx: Px,
    /// Y translation
    pub ty: Px,
}

impl RSXform {
    /// Creates a new RSXform with the given values.
    pub fn new(scos: f32, ssin: f32, tx: Px, ty: Px) -> Self {
        Self { scos, ssin, tx, ty }
    }

    /// Creates an identity RSXform (no transformation).
    pub fn identity() -> Self {
        Self::new(1.0, 0.0, px(0.0), px(0.0))
    }

    /// Creates an RSXform from scale, rotation angle (in radians), and translation.
    /// The anchor point is used to rotate around a specific point.
    pub fn from_radians(scale: f32, radians: f32, tx: Px, ty: Px, anchor: Xy<Px>) -> Self {
        let s = radians.sin() * scale;
        let c = radians.cos() * scale;
        Self::new(
            c,
            s,
            tx + anchor.x * (-c) + anchor.y * s,
            ty + anchor.x * (-s) - anchor.y * c,
        )
    }

    /// Creates an RSXform from scale and translation only (no rotation).
    pub fn from_scale_and_translate(scale: f32, tx: Px, ty: Px) -> Self {
        Self::new(scale, 0.0, tx, ty)
    }

    /// Creates an RSXform for simple translation only.
    pub fn from_translate(tx: Px, ty: Px) -> Self {
        Self::new(1.0, 0.0, tx, ty)
    }

    /// Returns true if the transformation keeps rectangles aligned to axes.
    pub fn rect_stays_rect(&self) -> bool {
        self.scos == 0.0 || self.ssin == 0.0
    }
}

impl Default for RSXform {
    fn default() -> Self {
        Self::identity()
    }
}

impl Eq for RSXform {}

impl std::hash::Hash for RSXform {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        OrderedFloat::new(self.scos).hash(state);
        OrderedFloat::new(self.ssin).hash(state);
        self.tx.hash(state);
        self.ty.hash(state);
    }
}
