use super::*;

#[wasm_bindgen]
extern "C" {
    // #[wasm_bindgen(structural, method)]
    // pub(crate) fn delete(this: &CanvasKitGrDirectContext);

    // #[wasm_bindgen(structural, method)]
    // pub(crate) fn delete(this: &CanvasKitParagraph);

    // #[wasm_bindgen(structural, method)]
    // pub(crate) fn delete(this: &CanvasKitParagraphBuilder);

    // #[wasm_bindgen(structural, method)]
    // pub(crate) fn delete(this: &CanvasKitParticles);

    // #[wasm_bindgen(structural, method)]
    // pub(crate) fn delete(this: &CanvasKitAnimatedImage);

    /// Do not delete CanvasKitCanvas!
    /// https://chromium.googlesource.com/skia/+/8f46ecc84fab83ffccd2977a633006d77ec3c161/modules/canvaskit/canvaskit/types/index.d.ts#2288
    // #[wasm_bindgen(structural, method)]
    // pub(crate) fn delete(this: &CanvasKitCanvas);

    #[wasm_bindgen(structural, method)]
    pub(crate) fn delete(this: &CanvasKitColorFilter);

    // #[wasm_bindgen(structural, method)]
    // pub(crate) fn delete(this: &CanvasKitContourMeasureIter);

    // #[wasm_bindgen(structural, method)]
    // pub(crate) fn delete(this: &CanvasKitContourMeasure);

    #[wasm_bindgen(structural, method)]
    pub(crate) fn delete(this: &CanvasKitFont);

    // #[wasm_bindgen(structural, method)]
    // pub(crate) fn delete(this: &CanvasKitFontMgr);

    #[wasm_bindgen(structural, method)]
    pub(crate) fn delete(this: &CanvasKitImage);

    // #[wasm_bindgen(structural, method)]
    // pub(crate) fn delete(this: &CanvasKitImageFilter);

    // #[wasm_bindgen(structural, method)]
    // pub(crate) fn delete(this: &CanvasKitMaskFilter);

    #[wasm_bindgen(structural, method)]
    pub(crate) fn delete(this: &CanvasKitPaint);

    #[wasm_bindgen(structural, method)]
    pub(crate) fn delete(this: &CanvasKitPath);

    // #[wasm_bindgen(structural, method)]
    // pub(crate) fn delete(this: &CanvasKitPathEffect);

    // #[wasm_bindgen(structural, method)]
    // pub(crate) fn delete(this: &CanvasKitSkPicture);

    // #[wasm_bindgen(structural, method)]
    // pub(crate) fn delete(this: &CanvasKitPictureRecorder);

    #[wasm_bindgen(structural, method)]
    pub(crate) fn delete(this: &CanvasKitRuntimeEffect);

    #[wasm_bindgen(structural, method)]
    pub(crate) fn delete(this: &CanvasKitShader);

    #[wasm_bindgen(structural, method)]
    pub(crate) fn delete(this: &CanvasKitSurface);

    #[wasm_bindgen(structural, method)]
    pub(crate) fn delete(this: &CanvasKitTextBlob);

    #[wasm_bindgen(structural, method)]
    pub(crate) fn delete(this: &CanvasKitTypeface);

    // #[wasm_bindgen(structural, method)]
    // pub(crate) fn delete(this: &CanvasKitVertices);

    // #[wasm_bindgen(structural, method)]
    // pub(crate) fn delete(this: &CanvasKitSkottieAnimation);

    // #[wasm_bindgen(structural, method)]
    // pub(crate) fn delete(this: &CanvasKitTypefaceFontProvider);

    // #[wasm_bindgen(structural, method)]
    // pub(crate) fn delete(this: &CanvasKitColorSpace);
}
