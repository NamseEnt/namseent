mod generated;

use super::*;

impl App {
    pub fn save(&self) -> String {
        let app_id = self.id;
        let save_data = SaveData {
            app_id,
            entities: self
                .entities
                .iter()
                .map(|entity| EntitySaveData {
                    entity_id: entity.id(),
                    components: generated::save_components(app_id, entity.id()),
                })
                .collect(),
        };

        ron::to_string(&save_data).unwrap()
    }
    pub fn load(save_data_ron: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let save_data = ron::from_str::<SaveData>(save_data_ron)?;

        let mut app = Self::new_with_id(save_data.app_id);
        for entity_save_data in save_data.entities {
            let entity = app.new_entity_with_id(entity_save_data.entity_id);

            for component_save_data in entity_save_data.components {
                generated::load_component(entity, component_save_data)?;
            }
        }

        Ok(app)
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
struct SaveData {
    app_id: Uuid,
    entities: Vec<EntitySaveData>,
}

#[derive(serde::Serialize, serde::Deserialize)]
struct EntitySaveData {
    entity_id: Uuid,
    components: Vec<ComponentSaveData>,
}

#[derive(serde::Serialize, serde::Deserialize)]
struct ComponentSaveData {
    component_type: String,
    component_ron: String,
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::app::game::*;
    use crate::component::*;
    use namui::prelude::*;
    use std::str::FromStr;
    use wasm_bindgen_test::wasm_bindgen_test;

    #[test]
    #[wasm_bindgen_test]
    fn save_should_works() {
        let app_id = Uuid::from_str("e2dec5e5-c48d-4476-9133-d52719c7c25e").unwrap();
        let mut app = App::new_with_id(app_id);

        let entity_id = Uuid::from_str("95d0fe84-cddd-4887-90f8-5b854ced76ac").unwrap();
        app.new_entity_with_id(entity_id)
            .add_component(Collider::from_circle(Xy::zero(), 1.tile()));

        let saved = app.save();

        let expected_pretty = r#"
            (
                app_id:"e2dec5e5-c48d-4476-9133-d52719c7c25e",
                entities:[(
                    entity_id:"95d0fe84-cddd-4887-90f8-5b854ced76ac",
                    components:[(
                        component_type:"Collider",
                        component_ron:"(rigid_body_at_origin:Circle((center:(x:0.0,y:0.0),radius:1.0)))"
                    )]
                )]
            )"#;
        let expected = expected_pretty
            .replace(" ", "")
            .replace("\t", "")
            .replace("\n", "");

        assert_eq!(expected, saved);
    }

    #[test]
    #[wasm_bindgen_test]
    fn load_should_works() {
        let ron = r#"
            (
                app_id:"e2dec5e5-c48d-4476-9133-d52719c7c25e",
                entities:[(
                    entity_id:"95d0fe84-cddd-4887-90f8-5b854ced76ac",
                    components:[(
                        component_type:"Collider",
                        component_ron:"(rigid_body_at_origin:Circle((center:(x:0.0,y:0.0),radius:1.0)))"
                    )]
                )]
            )"#;
        let app = App::load(ron).unwrap();
        assert_eq!(
            app.id,
            Uuid::from_str("e2dec5e5-c48d-4476-9133-d52719c7c25e").unwrap()
        );

        let query = app.query_entities::<&Collider>();

        assert_eq!(1, query.len());

        for (entity, _collider) in query {
            assert_eq!(
                Uuid::from_str("95d0fe84-cddd-4887-90f8-5b854ced76ac").unwrap(),
                entity.id()
            );
        }
    }
}
