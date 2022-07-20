use namui::prelude::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub async fn start() {
    let namui_context = namui::init().await;

    namui::start(namui_context, &mut ShaderExample::new(), &()).await
}

struct ShaderExample {}

impl ShaderExample {
    fn new() -> Self {
        Self {}
    }
}

namui::shader!(MyShader, {
     uniform float rad_scale;
     uniform float2 in_center;
     uniform float4 in_colors0;
     uniform float4 in_colors1;

     half4 main(float2 p) {
         float2 pp = p - in_center;
         float radius = sqrt(dot(pp, pp));
         radius = sqrt(radius);
         float angle = atan(pp.y / pp.x);
         float t = (angle + 3.1415926/2) / (3.1415926);
         t += radius * rad_scale;
         t = fract(t);
         return half4(mix(in_colors0, in_colors1, t));
     }
});

impl Entity for ShaderExample {
    type Props = ();

    fn render(&self, _props: &Self::Props) -> RenderingTree {
        let red_scale = (namui::now().as_millis() / 2000.0).sin() / 5.0;
        let in_center = [200.0, 200.0];
        let in_colors0 = [1.0, 0.0, 0.0, 1.0];
        let in_colors1 = [0.0, 1.0, 0.0, 1.0];

        let shader = MyShader::new(red_scale, in_center, in_colors0, in_colors1);
        let paint = PaintBuilder::new().set_shader(shader);
        let rect = PathBuilder::new().add_rect(Rect::Xywh {
            x: 100.px(),
            y: 100.px(),
            width: 200.px(),
            height: 200.px(),
        });

        path(rect, paint)
    }

    fn update(&mut self, _event: &dyn std::any::Any) {}
}
