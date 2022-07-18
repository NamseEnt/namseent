use super::SpecialRenderingNode;
use crate::RenderingTree;
use serde::Serialize;
use std::{
    any::{Any, TypeId},
    sync::{Arc, Mutex},
};

#[allow(dead_code)]
#[derive(Serialize)]
pub struct ReactNode {
    #[serde(skip_serializing)]
    type_id: TypeId,
    #[serde(skip_serializing)]
    pub(crate) props: Arc<dyn Any>,
    #[serde(skip_serializing)]
    rendering_tree: Mutex<Option<Arc<RenderingTree>>>,
    #[serde(skip_serializing)]
    react_entity: Mutex<Option<Box<dyn React>>>,
    #[serde(skip_serializing)]
    create_entity: Arc<dyn Fn() -> Box<dyn React>>,
}

impl Clone for ReactNode {
    fn clone(&self) -> Self {
        Self {
            type_id: self.type_id,
            props: self.props.clone(),
            rendering_tree: Mutex::new(self.rendering_tree.lock().unwrap().clone()),
            react_entity: Mutex::new(None),
            create_entity: self.create_entity.clone(),
        }
    }
}

impl std::fmt::Debug for ReactNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ReactNode")
    }
}

pub trait React {
    fn render(&self, props: &dyn Any) -> RenderingTree;
    fn update(&mut self, event: &dyn Any);
}

pub fn react<TReact, TCreateEntity, TProps>(
    create_entity: TCreateEntity,
    props: TProps,
) -> RenderingTree
where
    TReact: React + 'static,
    TCreateEntity: 'static + Fn() -> Box<dyn React>,
    TProps: Any,
{
    RenderingTree::Special(SpecialRenderingNode::React(ReactNode {
        type_id: TypeId::of::<TReact>(),
        props: Arc::new(props),
        rendering_tree: Mutex::new(None),
        react_entity: Mutex::new(None),
        create_entity: Arc::new(create_entity),
    }))
}

impl ReactNode {
    pub(crate) fn render(&self) -> Arc<RenderingTree> {
        let rendering_tree = self.rendering_tree.lock().unwrap();

        match rendering_tree.as_ref() {
            Some(rendering_tree) => rendering_tree.clone(),
            None => {
                crate::log!("[Debug] rendering_tree is None for ReactNode");
                Arc::new(RenderingTree::Empty)
            }
        }
    }
    pub(crate) fn reconciliate(&self, prev_node: Option<&ReactNode>, event: Option<&dyn Any>) {
        let mut entity = match prev_node {
            Some(prev_node) if prev_node.type_id == self.type_id => {
                prev_node.react_entity.lock().unwrap().take().unwrap()
            }
            _ => (self.create_entity)(),
        };

        if let Some(event) = event {
            entity.update(event);
        }

        let rendering_tree = entity.render(&*self.props);

        self.react_entity.lock().unwrap().replace(entity);
        self.rendering_tree
            .lock()
            .unwrap()
            .replace(Arc::new(rendering_tree));
    }
}

pub fn reconciliate(prev_tree: &RenderingTree, next_tree: &RenderingTree, event: Option<&dyn Any>) {
    match next_tree {
        RenderingTree::Node(_) => {}
        RenderingTree::Children(children) => {
            if let RenderingTree::Children(prev_children) = prev_tree {
                for (index, child) in children.iter().enumerate() {
                    if let Some(prev_child) = prev_children.get(index) {
                        reconciliate(prev_child, child, event);
                    } else {
                        renew_react(child, event);
                    }
                }
            } else {
                for child in children {
                    renew_react(child, event);
                }
            }
        }
        RenderingTree::Special(special) => {
            if let RenderingTree::Special(prev_special) = prev_tree {
                if is_same_special_variant(prev_special, special) {
                    match (prev_special, special) {
                        (
                            SpecialRenderingNode::React(prev_react),
                            SpecialRenderingNode::React(next_react),
                        ) => {
                            next_react.reconciliate(Some(prev_react), event);
                        }
                        _ => {}
                    }
                    reconciliate(
                        &prev_special.get_rendering_tree(),
                        &special.get_rendering_tree(),
                        event,
                    );
                } else {
                    renew_react(&special.get_rendering_tree(), event);
                }
            } else {
                renew_react(&special.get_rendering_tree(), event);
            }
        }
        RenderingTree::Empty => {}
    }
}

fn is_same_special_variant(a: &SpecialRenderingNode, b: &SpecialRenderingNode) -> bool {
    std::mem::discriminant(a) == std::mem::discriminant(b)
}

fn renew_react(tree: &RenderingTree, event: Option<&dyn Any>) {
    match tree {
        RenderingTree::Node(_) => {}
        RenderingTree::Children(children) => {
            children.iter().for_each(|child| renew_react(child, event));
        }
        RenderingTree::Special(special) => {
            if let SpecialRenderingNode::React(react) = special {
                react.reconciliate(None, event);
            }
            let child = special.get_rendering_tree();
            renew_react(&child, event);
        }
        RenderingTree::Empty => {}
    }
}
