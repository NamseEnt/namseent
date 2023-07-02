use super::component_tree::{with_component_tree_of_key_mut, Key};
use crate::hooks::draw::draw;
use std::{
    collections::VecDeque,
    sync::{Arc, OnceLock},
};
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};

pub(crate) static UPDATE_REQUEST_TX: OnceLock<UnboundedSender<UpdateRequest>> = OnceLock::new();

pub(crate) fn invoke_update(key: Key, source: Arc<Source>) {
    UPDATE_REQUEST_TX
        .get()
        .unwrap()
        .send(UpdateRequest {
            key: key.clone(),
            source,
        })
        .unwrap();
}

#[derive(Debug, Clone)]
pub(crate) struct UpdateRequest {
    pub(crate) key: Key,
    pub(crate) source: Arc<Source>,
}

pub(crate) type Source = ();

pub(crate) async fn update_task(mut update_request_rx: UnboundedReceiver<UpdateRequest>) {
    let mut update_merged_queue: VecDeque<UpdateRequest> = VecDeque::new();

    let mut prev_handling_source: Option<Arc<Source>> = None;
    while let Some(request) = update_request_rx.recv().await {
        let handling_source = prev_handling_source
            .take()
            .unwrap_or_else(|| request.source.clone());

        insert_request(&mut update_merged_queue, request);

        loop {
            while let Ok(request) = update_request_rx.try_recv() {
                insert_request(&mut update_merged_queue, request);
            }
            match pop_request_of_source(&mut update_merged_queue, &handling_source) {
                Some(request) => {
                    update_component(request);

                    if is_update_of_source_finished(&handling_source) {
                        draw();
                        break;
                    }

                    tokio::task::yield_now().await;
                }
                None => {
                    prev_handling_source = Some(handling_source);
                    break;
                }
            }
        }
    }

    fn insert_request(merged_queue: &mut VecDeque<UpdateRequest>, request: UpdateRequest) {
        if let None = merged_queue
            .iter_mut()
            .find(|merged| merged.key == request.key)
        {
            merged_queue.push_back(request);
        }
    }
}

fn is_update_of_source_finished(source: &Arc<Source>) -> bool {
    Arc::strong_count(source) == 1
}

fn pop_request_of_source(
    update_merged_queue: &mut VecDeque<UpdateRequest>,
    source: &Arc<Source>,
) -> Option<UpdateRequest> {
    update_merged_queue
        .iter()
        .position(|merged| Arc::ptr_eq(&merged.source, source))
        .map(|index| update_merged_queue.remove(index).unwrap())
}

fn update_component(request: UpdateRequest) {
    with_component_tree_of_key_mut(request.key, |node| node.update(request.source));
}
