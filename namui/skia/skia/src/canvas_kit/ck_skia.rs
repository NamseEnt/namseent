use super::*;
use crate::*;
use anyhow::anyhow;
use wasm_bindgen::JsCast;
use web_sys::HtmlCanvasElement;

pub struct CkSkia {
    surface: CkSurface,
    calculate: CkCalculate,
}

impl CkSkia {
    pub(crate) async fn new(canvas_element: &HtmlCanvasElement) -> Result<CkSkia> {
        let surface: CanvasKitSurface = canvas_kit()
            .MakeWebGLCanvasSurface(canvas_element, None, None)
            .ok_or(anyhow!("Failed to create WebGL canvas surface"))?;
        // let surface: CanvasKitSurface = 'block: {
        // let navigator = js_sys::Reflect::get(&js_sys::global(), &"navigator".into()).unwrap();
        // let Ok(gpu) = js_sys::Reflect::get(&navigator, &"gpu".into()) else {
        //     break 'block canvas_kit()
        //         .MakeWebGLCanvasSurface(canvas_element, None, None)
        //         .ok_or(anyhow!("Failed to create WebGL canvas surface"))?;
        // };

        // let request_adapter_fn: js_sys::Function =
        //     js_sys::Reflect::get(&gpu, &"requestAdapter".into())
        //         .unwrap()
        //         .dyn_into()
        //         .unwrap();

        // let adapter_promise: js_sys::Promise =
        //     request_adapter_fn.call0(&gpu).unwrap().dyn_into().unwrap();

        // let adapter = wasm_bindgen_futures::JsFuture::from(adapter_promise)
        //     .await
        //     .unwrap();

        // let request_device_fn: js_sys::Function =
        //     js_sys::Reflect::get(&adapter, &"requestDevice".into())
        //         .unwrap()
        //         .dyn_into()
        //         .unwrap();

        // let device_promise: js_sys::Promise = request_device_fn
        //     .call0(&adapter)
        //     .unwrap()
        //     .dyn_into()
        //     .unwrap();

        // let device: js_sys::Object = wasm_bindgen_futures::JsFuture::from(device_promise)
        //     .await
        //     .unwrap()
        //     .into();

        // let device_ctx = canvas_kit()
        //     .MakeGPUDeviceContext(&device)
        //     .ok_or(anyhow!("Failed to create GPU device context"))?;
        // let canvas_ctx = canvas_kit()
        //     .MakeGPUCanvasContext(&device_ctx, canvas_element)
        //     .ok_or(anyhow!("Failed to create GPU canvas context"))?;

        // canvas_kit()
        //     .MakeGPUCanvasSurface(&canvas_ctx, None)
        //     .ok_or(anyhow!("Failed to create GPU canvas surface"))?
        // };

        Ok(Self {
            surface: CkSurface::new(surface),
            calculate: CkCalculate::new(),
        })
    }
}

impl SkSkia for CkSkia {
    fn surface(&mut self) -> &mut dyn SkSurface {
        &mut self.surface
    }

    fn on_resize(&mut self, _wh: Wh<IntPx>) {
        // NOTE: Maybe we need to implement this?
        unreachable!()
    }

    fn move_to_next_frame(&mut self) {
        // browser will handle this
    }

    fn load_image_from_web_image_bitmap(&self, image_bitmap: web_sys::ImageBitmap) -> ImageLoaded {
        CkImage::load_image_from_web_image_bitmap(image_bitmap)
    }

    fn unload_image(&self, image_id: u32) {
        CkImage::unload_image(image_id)
    }
}

impl SkCalculate for CkSkia {
    fn group_glyph(&self, font: &Font, paint: &Paint) -> Arc<dyn GroupGlyph> {
        self.calculate.group_glyph(font, paint)
    }

    fn font_metrics(&self, font: &Font) -> Option<FontMetrics> {
        self.calculate.font_metrics(font)
    }

    fn load_typeface(&self, typeface_name: &str, bytes: &[u8]) -> Result<()> {
        self.calculate.load_typeface(typeface_name, bytes)
    }

    fn path_contains_xy(&self, path: &Path, paint: Option<&Paint>, xy: Xy<Px>) -> bool {
        self.calculate.path_contains_xy(path, paint, xy)
    }

    fn path_bounding_box(&self, path: &Path, paint: Option<&Paint>) -> Option<Rect<Px>> {
        self.calculate.path_bounding_box(path, paint)
    }
}
