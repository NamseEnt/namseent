mod atom;
mod components;
mod loaded;
mod sequence;

use crate::components::name_quick_slot::NameQuickSlot;
use ::futures::try_join;
use loaded::LoadedSequenceEditorPage;
use namui::*;
use namui_prebuilt::*;
use rpc::data::{CgFile, ImageWithLabels, Memo, ProjectSharedData, Sequence};
use std::collections::HashMap;

#[namui::component]
pub struct SequenceEditPage {
    pub wh: Wh<Px>,
    pub project_id: Uuid,
    pub sequence_id: Uuid,
}

impl Component for SequenceEditPage {
    fn render(self, ctx: &RenderCtx)  {
        let Self {
            wh,
            project_id,
            sequence_id,
        } = self;
        let (data, set_data) = ctx.state::<Option<Result<LoadData, String>>>(|| None);

        ctx.effect("Load data", || {
            spawn_local(async move {
                match load_data(project_id, sequence_id).await {
                    Ok(data) => set_data.set(Some(Ok(data))),
                    Err(error) => {
                        set_data.set(Some(Err(error)));
                    }
                }
            })
        });

        ctx.compose(|ctx| {
            match data.as_ref() {
                Some(result) => match result {
                    Ok(data) => ctx.add(LoadedSequenceEditorPage {
                        cut_id_memos_map: data.cut_id_memos_map.clone(),
                        project_shared_data: data.project_shared_data.clone(),
                        sequence: data.sequence.clone(),
                        user_id: data.user_id,
                        wh,
                        cg_files: data.cg_files.clone(),
                        images: data.images.clone(),
                        name_quick_slot: data.name_quick_slot.clone(),
                    }),
                    Err(err) => ctx.add(typography::body::center(
                        wh,
                        format!("Error: {}", err),
                        Color::WHITE,
                    )),
                },
                None => ctx.add(typography::body::center(wh, "loading...", Color::WHITE)),
            };
        })
        .done()
    }
}

#[derive(Debug)]
struct LoadData {
    sequence: Sequence,
    project_shared_data: ProjectSharedData,
    cut_id_memos_map: HashMap<Uuid, Vec<Memo>>,
    user_id: Uuid,
    cg_files: Vec<CgFile>,
    images: Vec<ImageWithLabels>,
    name_quick_slot: NameQuickSlot,
}
async fn load_data(project_id: namui::Uuid, sequence_id: namui::Uuid) -> Result<LoadData, String> {
    let result = try_join!(
        load_sequence_and_project_shared_data(sequence_id),
        load_memos(sequence_id),
        get_user_id(),
        get_cg_files(project_id),
        get_images(project_id),
        get_name_quick_slot(project_id),
    );
    return match result {
        Ok((
            (sequence, project_shared_data),
            cut_id_memos_map,
            user_id,
            cg_files,
            images,
            name_quick_slot,
        )) => Ok(LoadData {
            sequence,
            project_shared_data,
            cut_id_memos_map,
            user_id,
            cg_files,
            images,
            name_quick_slot,
        }),
        Err(error) => Err(error.to_string()),
    };

    async fn load_sequence_and_project_shared_data(
        sequence_id: namui::Uuid,
    ) -> Result<(Sequence, ProjectSharedData), Box<dyn std::error::Error>> {
        let rpc::get_sequence_and_project_shared_data::Response {
            sequence,
            project_shared_data_json,
        }: rpc::get_sequence_and_project_shared_data::Response = crate::RPC
            .get_sequence_and_project_shared_data(
                rpc::get_sequence_and_project_shared_data::Request { sequence_id },
            )
            .await?;
        let project_shared_data = serde_json::from_str(&project_shared_data_json)?;
        Ok((sequence, project_shared_data))
    }
    async fn load_memos(
        sequence_id: namui::Uuid,
    ) -> Result<HashMap<Uuid, Vec<Memo>>, Box<dyn std::error::Error>> {
        let response = crate::RPC
            .list_sequence_memos(rpc::list_sequence_memos::Request { sequence_id })
            .await?;
        let cut_id_memos_map =
            response
                .memos
                .into_iter()
                .fold(HashMap::<Uuid, Vec<Memo>>::new(), |mut acc, memo| {
                    match acc.get_mut(&memo.cut_id) {
                        Some(memos) => memos.push(memo),
                        None => {
                            acc.insert(memo.cut_id, vec![memo]);
                        }
                    };
                    acc
                });
        Ok(cut_id_memos_map)
    }
    async fn get_user_id() -> Result<Uuid, Box<dyn std::error::Error>> {
        let response = crate::RPC.get_user_id(rpc::get_user_id::Request {}).await?;
        Ok(response.user_id)
    }
    async fn get_cg_files(
        project_id: Uuid,
    ) -> Result<Vec<rpc::data::CgFile>, Box<dyn std::error::Error>> {
        let response = crate::RPC
            .list_cg_files(rpc::list_cg_files::Request { project_id })
            .await?;
        Ok(response.cg_files)
    }
    async fn get_images(
        project_id: Uuid,
    ) -> Result<Vec<rpc::data::ImageWithLabels>, Box<dyn std::error::Error>> {
        let response = crate::RPC
            .list_images(rpc::list_images::Request { project_id })
            .await?;
        Ok(response.images)
    }
    async fn get_name_quick_slot(
        project_id: Uuid,
    ) -> Result<NameQuickSlot, Box<dyn std::error::Error>> {
        let name_quick_slot = NameQuickSlot::load_from_cache(project_id).await?;
        Ok(name_quick_slot)
    }
}
