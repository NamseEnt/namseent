use crate::hooks::{
    component::{Component, ComponentProps, WireClosures},
    render::Render,
};
use std::sync::Arc;

#[derive(PartialEq, Debug)]
pub struct List {
    pub children: Vec<Arc<dyn Component>>,
}

impl List {
    pub fn new() -> List {
        List {
            children: Vec::new(),
        }
    }

    pub fn add(mut self, child: impl Component + 'static) -> List {
        self.children.push(Arc::new(child));
        self
    }

    pub fn from_iter<I>(iter: I) -> List
    where
        I: IntoIterator,
        I::Item: Component + 'static,
    {
        let mut list = List::new();
        for child in iter {
            list = list.add(child);
        }
        list
    }
}

impl ComponentProps for List {
    fn render(&self, mut render: Render) -> Render {
        for child in &self.children {
            render = render.add_arc(child.clone());
        }
        render
    }
}

impl WireClosures for List {
    fn wire_closures(&self, to: &dyn Component) {
        let Some(to) = to.as_any().downcast_ref::<Self>() else {
            return;
        };
        self.children
            .iter()
            .zip(to.children.iter())
            .for_each(|(from, to)| from.wire_closures(to.as_ref()));
    }
}

impl Component for List {}
