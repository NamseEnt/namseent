use quote::{format_ident, quote};
use std::process::Command;
use walkdir::WalkDir;

fn main() {
    println!("cargo:rerun-if-changed=src/component");
    let files = WalkDir::new("src/component")
        .into_iter()
        .filter_map(|entry| {
            let entry = entry.unwrap();
            if entry.file_type().is_file() && entry.path().extension().unwrap() == "rs" {
                Some(entry.path().to_str().unwrap().to_string())
            } else {
                None
            }
        });

    let component_idents = files
        .flat_map(|file| {
            let content = std::fs::read_to_string(file).unwrap();
            let mut component_idents = Vec::new();
            enum State {
                FindingComponentAttribute,
                FindingComponentName,
            }
            let mut state = State::FindingComponentAttribute;
            for line in content.lines() {
                match state {
                    State::FindingComponentAttribute => {
                        if line.starts_with("#[ecs_macro::component]") {
                            state = State::FindingComponentName;
                        }
                    }
                    State::FindingComponentName => {
                        if line.starts_with("pub struct ") {
                            let component_name = line
                                .split("pub struct ")
                                .nth(1)
                                .unwrap()
                                .split(" ")
                                .nth(0)
                                .unwrap();
                            component_idents.push(format_ident!("{component_name}"));
                            state = State::FindingComponentAttribute;
                        }
                    }
                }
            }
            component_idents
        })
        .collect::<Vec<_>>();

    let save_components_lines = component_idents.iter().map(|component_ident| {
        let set_ident = format_ident!("{}_SET", component_ident);
        quote! {
            unsafe {
                let component = #set_ident.get().and_then(|entity_set| {
                    entity_set
                        .get(&app_id)
                        .and_then(|component_set| component_set.get(&entity_id))
                });
                if let Some(component) = component {
                    components.push(ComponentSaveData {
                        component_type: stringify!(#component_ident).to_string(),
                        component_ron: ron::to_string(component).unwrap(),
                    });
                }
            }
        }
    });

    let load_component_lines = component_idents.iter().map(|component_ident| {
        quote! {
            if stringify!(#component_ident) == component_save_data.component_type {
                let component: #component_ident = ron::from_str(&component_save_data.component_ron)?;
                entity.add_component(component);
                return Ok(());
            }
        }
    });

    let file_content = quote! {
        use crate::component::*;
        use super::*;

        pub(super) fn save_components(app_id: namui::Uuid, entity_id: namui::Uuid) -> Vec<ComponentSaveData> {
            let mut components = vec![];

            #(#save_components_lines)*

            components
        }

        pub(super) fn load_component(
            entity: &mut Entity,
            component_save_data: ComponentSaveData
        ) -> Result<(), Box::<dyn std::error::Error>> {
            #(#load_component_lines)*

            Err(format!("Component type {} not found", component_save_data.component_type).into())
        }
    }.to_string();

    let path = "src/ecs/app/save/generated.rs";

    std::fs::write(path, file_content).unwrap();

    Command::new("rustfmt")
        .arg(path)
        .spawn()
        .unwrap()
        .wait()
        .unwrap();
}
