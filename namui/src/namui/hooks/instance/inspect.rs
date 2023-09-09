use super::*;
use chumsky::prelude::*;
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

static INSPECT_TOGGLE_ON: AtomicBool = AtomicBool::new(false);

#[wasm_bindgen]
pub fn set_inspect_toggle_on(is_on: bool) {
    INSPECT_TOGGLE_ON.store(is_on, std::sync::atomic::Ordering::Relaxed);
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen()]
    fn onInspect(inspect_tree: JsValue);
}

impl ComponentInstance {
    pub(crate) fn inspect(&self) {
        if !INSPECT_TOGGLE_ON.load(std::sync::atomic::Ordering::Relaxed) {
            return;
        }

        let tree = self.generate_inspect_tree();
        onInspect(serde_wasm_bindgen::to_value(&tree).unwrap());
    }

    fn generate_inspect_tree(&self) -> InspectTree {
        let bounding_box = self.debug_bounding_box.lock().unwrap();
        InspectTree {
            short_name: static_type_short_name(self.component_type_name),
            left: bounding_box.map(|x| x.left().as_f32()),
            top: bounding_box.map(|x| x.top().as_f32()),
            width: bounding_box.map(|x| x.width().as_f32()),
            height: bounding_box.map(|x| x.height().as_f32()),
            children: self
                .children_instances
                .lock()
                .unwrap()
                .values()
                .map(|child| child.generate_inspect_tree())
                .collect(),
        }
    }
}

fn static_type_short_name(static_type_name: &str) -> String {
    let short_name = parser().parse(static_type_name).unwrap();
    short_name.to_string()
}

#[derive(serde::Serialize)]
struct InspectTree {
    #[serde(rename = "shortName")]
    short_name: String,
    left: Option<f32>,
    top: Option<f32>,
    width: Option<f32>,
    height: Option<f32>,
    children: Vec<InspectTree>,
}

#[derive(Clone)]
struct ShortName {
    name: String,
    generic: Option<Box<ShortName>>,
}
impl std::fmt::Display for ShortName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.name)?;
        if let Some(generic) = &self.generic {
            f.write_str("<")?;
            f.write_str(&generic.to_string())?;
            f.write_str(">")?;
        }
        Ok(())
    }
}
fn parser() -> impl Parser<char, ShortName, Error = Simple<char>> {
    let ident = filter(|c: &char| c.is_alphabetic() || *c == '_')
        .repeated()
        .at_least(1)
        .collect::<String>();
    let single_path = recursive(|single_path| {
        ident
            .then(just("::").ignored())
            .then(single_path)
            .map(|(_prefix, name)| name)
            .or(ident)
    });

    let path_with_generic = recursive(|path_with_generic| {
        single_path
            .map(|name| ShortName {
                name,
                generic: None,
            })
            .then(
                just('<')
                    .ignore_then(path_with_generic)
                    .then_ignore(just('>'))
                    .or_not(),
            )
            .map(|(mut name, generic)| {
                name.generic = generic.map(Box::new);
                name
            })
    });

    path_with_generic
}
