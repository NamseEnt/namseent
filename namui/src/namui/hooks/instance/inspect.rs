use super::*;
use chumsky::prelude::*;
use web_sys::{window, Element};

impl ComponentInstance {
    pub(crate) fn inspect(&self) {
        let document = window().unwrap().document().unwrap();
        let body = document.body().unwrap();

        let style_element = body
            .query_selector(&format!("#inspect-style"))
            .unwrap()
            .unwrap_or_else(|| {
                let style_element = document.create_element("style").unwrap();
                style_element.set_attribute("type", "text/css").unwrap();
                style_element.set_attribute("id", "inspect-style").unwrap();
                body.append_child(&style_element).unwrap();
                style_element
            });

        let inspect_root = body
            .query_selector(&format!("inspect"))
            .unwrap()
            .unwrap_or_else(|| {
                let inspect_root = document.create_element("inspect").unwrap();
                body.append_child(&inspect_root).unwrap();
                inspect_root
            });

        style_element.set_inner_html("");
        inspect_root.set_inner_html("");

        self.internal_inspect(&inspect_root, &style_element, vec![0]);
    }

    fn internal_inspect(
        &self,
        container_element: &Element,
        style_element: &Element,
        children_indexes: Vec<usize>,
    ) {
        let short_name = static_type_short_name(self.component_type_name);
        let escaped_short_name = short_name.replace("<", "__").replace(">", "__");

        let document = window().unwrap().document().unwrap();

        let element = document
            .create_element(&format!("_{escaped_short_name}"))
            .map_err(|_err| format!("Failed to create element: {}", escaped_short_name))
            .unwrap();
        element.set_attribute("name", &short_name).unwrap();

        if let Some(bounding_box) = self.debug_bounding_box.lock().unwrap().as_ref() {
            let inner_html = style_element.inner_html();
            let nth_children = children_indexes
                .iter()
                .map(|x| format!(":nth-child({})", x + 1))
                .collect::<Vec<_>>()
                .join(" > ");
            style_element.set_inner_html(&format!(
                "{inner_html}\n\
                inspect > {nth_children} {{
                    position: fixed;
                    left: {left};
                    top: {top};
                    width: {width};
                    height: {height};
                }}
            ",
                left = bounding_box.left(),
                top = bounding_box.top(),
                width = bounding_box.width(),
                height = bounding_box.height()
            ));
        }

        container_element.append_child(&element).unwrap();

        self.children_instances
            .lock()
            .unwrap()
            .values()
            .enumerate()
            .for_each(|(index, child)| {
                child.internal_inspect(&element, style_element, {
                    let mut children_indexes = children_indexes.clone();
                    children_indexes.push(index);
                    children_indexes
                })
            });
    }
}

fn static_type_short_name(static_type_name: &str) -> String {
    let short_name = parser().parse(static_type_name).unwrap();
    short_name.to_string()
}

#[derive(Clone)]
struct ShortName {
    name: String,
    generic: Option<Box<ShortName>>,
}
impl ShortName {
    fn to_string(&self) -> String {
        let mut s = self.name.clone();
        if let Some(generic) = &self.generic {
            s.push('<');
            s.push_str(&generic.to_string());
            s.push('>');
        }
        s
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
                name.generic = generic.map(|x| Box::new(x));
                name
            })
    });

    path_with_generic
}
