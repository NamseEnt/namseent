use super::*;
use namui::network::http::HttpError;

use std::sync::{Arc, Mutex};
use yrs::updates::decoder::Decode;

pub struct ServerStorage<T: crdt::History> {
    phantom: std::marker::PhantomData<T>,
    rpc: rpc::Rpc,
}

impl<T: crdt::History> ServerStorage<T> {
    pub fn new(rpc: rpc::Rpc) -> Self {
        Self {
            phantom: std::marker::PhantomData,
            rpc,
        }
    }
}

impl<T: crdt::History> Storage<T> for ServerStorage<T> {
    fn get<'a>(
        &'a self,
    ) -> Pin<Box<dyn 'a + Future<Output = Result<crdt::HistorySystem<T>, GetError>>>> {
        Box::pin(async move {
            let response = self
                .rpc.get_sequence(rpc::get_sequence::Request {
                    sequence_id
                })
                .get_repository_content_raw(&self.branch, DOC_PATH)
                .await;
            if let Err(GithubClientError::NetworkError(HttpError::Status { status: 404, .. })) =
                response
            {
                return Err(GetError::NotExists);
            }

            let raw = response?;
            let history_system = crdt::HistorySystem::decode(raw);

            Ok(history_system)
        })
    }

    fn start_sync<'a>(
        &'a self,
        update_queue: Arc<Mutex<Vec<yrs::Update>>>,
        sync_status: Arc<Mutex<SyncStatus>>,
    ) -> Pin<Box<dyn 'a + Future<Output = ()>>> {
        *sync_status.lock().unwrap() = SyncStatus::Idle;
        Box::pin(async move {
            const DEFAULT_DELAY_TIME: namui::Time = namui::Time::Ms(100.0);
            const MAX_DELAY_TIME: namui::Time = namui::Time::Ms(1000.0);
            let mut delay_time = DEFAULT_DELAY_TIME;
            let doc = crdt::yrs::Doc::new();

            loop {
                let queue = std::mem::replace(&mut *update_queue.lock().unwrap(), Vec::new());
                if queue.is_empty() {
                    namui::time::delay(DEFAULT_DELAY_TIME).await;
                    continue;
                }

                *sync_status.lock().unwrap() = SyncStatus::Sending(namui::time::now());
                {
                    let mut transact = doc.transact();
                    for update in queue {
                        transact.apply_update(update)
                    }
                }

                let sha =
                    match self
                        .github_client
                        .get_repository_content(&self.branch, DOC_PATH)
                        .await
                    {
                        Ok(response) => Some(response.sha),
                        Err(GithubClientError::NetworkError(HttpError::Status {
                            status, ..
                        })) if status == 404 => None,
                        Err(err) => {
                            namui::log!("get_repository_content error: {:?}", err);
                            namui::time::delay(delay_time).await;
                            delay_time = (delay_time * 2.0).min(MAX_DELAY_TIME);
                            continue;
                        }
                    };

                if sha.is_some() {
                    let remote_encoded_update = match self
                        .github_client
                        .get_repository_content_raw(&self.branch, DOC_PATH)
                        .await
                    {
                        Ok(remote_encoded_update) => remote_encoded_update,
                        Err(err) => {
                            namui::log!("get_repository_content_raw error: {:?}", err);
                            namui::time::delay(delay_time).await;
                            delay_time = (delay_time * 2.0).min(MAX_DELAY_TIME);
                            continue;
                        }
                    };
                    let is_same_with_remote = doc
                        .encode_state_as_update_v2(&yrs::StateVector::default())
                        .into_boxed_slice()
                        == remote_encoded_update;

                    if is_same_with_remote {
                        *sync_status.lock().unwrap() = SyncStatus::Idle;
                        continue;
                    }

                    doc.transact()
                        .apply_update(yrs::Update::decode_v2(&remote_encoded_update).unwrap());
                }

                let response = self
                    .github_client
                    .put_repository_content(
                        &self.branch,
                        DOC_PATH,
                        sha.as_ref().map(String::as_str),
                        &base64::encode(
                            doc.encode_state_as_update_v2(&yrs::StateVector::default()),
                        ),
                        "sync",
                    )
                    .await;

                if let Err(err) = response {
                    namui::log!("put_repository_content error: {:?}", err);
                    namui::time::delay(delay_time).await;
                    delay_time = (delay_time * 2.0).min(MAX_DELAY_TIME);
                    continue;
                }
                *sync_status.lock().unwrap() = SyncStatus::Sent(namui::time::now());
            }
        })
    }

    fn upload_resource<'a>(
        &'a self,
        path: String,
        data: &'a [u8],
    ) -> Pin<Box<dyn 'a + Future<Output = Result<(), UploadResourceError>>>> {
        Box::pin(async move {
            let path = to_resource_path(&path);
            let sha = match self
                .github_client
                .get_repository_content(&self.branch, &path)
                .await
            {
                Ok(response) => Some(response.sha),
                Err(GithubClientError::NetworkError(HttpError::Status { status, .. }))
                    if status == 404 =>
                {
                    None
                }
                Err(err) => return Err(err.into()),
            };

            let response = self
                .github_client
                .put_repository_content(
                    &self.branch,
                    &path,
                    sha.as_ref().map(String::as_str),
                    &base64::encode(data),
                    &format!("upload resource {path}"),
                )
                .await;

            response.map(|_| ()).map_err(|err| match err {
                GithubClientError::NetworkError(HttpError::Status { status, .. })
                    if status == 409 =>
                {
                    UploadResourceError::Conflict
                }
                error => error.into(),
            })
        })
    }

    fn list_resources<'a>(
        &'a self,
    ) -> Pin<Box<dyn 'a + Future<Output = Result<Box<[String]>, ListResourceError>>>> {
        Box::pin(async move {
            #[derive(serde::Deserialize)]
            struct QueryResult {
                data: Data,
            }
            #[derive(serde::Deserialize)]
            struct Data {
                repository: Repository,
            }
            #[derive(serde::Deserialize)]
            struct Repository {
                r#ref: Ref,
            }
            #[derive(serde::Deserialize)]
            struct Ref {
                target: Target,
            }
            #[derive(serde::Deserialize)]
            struct Target {
                tree: RootTree,
            }
            #[derive(serde::Deserialize)]
            struct RootTree {
                entries: Box<[Entry]>,
            }
            #[derive(serde::Deserialize)]
            struct Entry {
                name: String,
                object: SubTree,
            }
            #[derive(serde::Deserialize)]
            struct SubTree {
                entries: Option<Box<[SubEntry]>>,
            }
            #[derive(serde::Deserialize)]
            struct SubEntry {
                name: String,
            }
            let query_result: QueryResult = self
                .github_client
                .graphql_query(format!(
                    r#"
{{
    repository(owner: "{owner}", name: "{repo}") {{
        ref(qualifiedName: "{branch}") {{
            target {{
                ... on Commit {{
                    tree {{
                        entries {{
                            name
                            object {{
                                ... on Tree {{
                                    entries {{
                                        name
                                    }}
                                }}
                            }}
                        }}
                    }}
                }}
            }}
        }}
    }}
}}
"#,
                    owner = self.github_client.owner(),
                    repo = self.github_client.repo(),
                    branch = self.branch,
                ))
                .await?;

            let mut resources: Vec<String> = vec![];

            let root_entries = query_result.data.repository.r#ref.target.tree.entries;
            if let Some(resources_tree) = root_entries
                .iter()
                .find(|entry| entry.name == RESOURCES_PREFIX)
            {
                if let Some(entries) = &resources_tree.object.entries {
                    resources = entries
                        .iter()
                        .map(|entry| entry.name.clone())
                        .collect::<Vec<_>>();
                }
            }

            Ok(resources.into_boxed_slice())
        })
    }

    fn get_resource<'a>(
        &'a self,
        path: String,
    ) -> Pin<Box<dyn 'a + Future<Output = Result<Box<[u8]>, GetResourceError>>>> {
        Box::pin(async move {
            let path = to_resource_path(&path);

            let response = self
                .github_client
                .get_repository_content_raw(&self.branch, &path)
                .await?;
            Ok(response)
        })
    }
}

impl From<GithubClientError> for GetError {
    fn from(error: GithubClientError) -> Self {
        if let GithubClientError::NetworkError(network_error) = &error {
            if let HttpError::Status { status, .. } = network_error {
                if *status == 404 {
                    return GetError::NotExists;
                }
            }
        }
        GetError::Unknown(error.into())
    }
}

impl From<GithubClientError> for UploadResourceError {
    fn from(error: GithubClientError) -> Self {
        Self::Unknown(error.into())
    }
}

impl From<GithubClientError> for ListResourceError {
    fn from(error: GithubClientError) -> Self {
        Self::Unknown(error.into())
    }
}

impl From<GithubClientError> for GetResourceError {
    fn from(error: GithubClientError) -> Self {
        Self::Unknown(error.into())
    }
}

#[cfg(test)]
mod test {
    // use super::*;
    // use crdt::*;
    // use std::sync::{atomic::AtomicI32, Arc};

    // #[wasm_bindgen_test::wasm_bindgen_test]
    // async fn sync_should_retry_on_conflict() {
    //     let mut mock_client = GithubClient::default();

    //     #[history]
    //     #[derive(Clone)]
    //     struct State {}
    //     let mut history_system = crdt::HistorySystem::new(State {});

    //     let encoded = history_system.encode();

    //     mock_client
    //         .expect_get_repository_content()
    //         .returning(|_, _, _| {
    //             Ok(crate::github_client::GetRepositoryContentResponseBody {
    //                 r#type: crate::github_client::Type::File,
    //                 encoding: None,
    //                 size: u32::default(),
    //                 name: String::default(),
    //                 path: String::default(),
    //                 content: None,
    //                 sha: String::default(),
    //             })
    //         });

    //     mock_client
    //         .expect_get_repository_content_raw()
    //         .returning(move |_, _| Ok(encoded.clone().into()));

    //     let call_count = Arc::new(AtomicI32::new(0));
    //     mock_client.expect_put_repository_content().returning({
    //         let call_count = call_count.clone();
    //         move |_, _, _, _, _| {
    //             call_count.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    //             if call_count.load(std::sync::atomic::Ordering::Relaxed) == 5 {
    //                 Ok(crate::github_client::PutRepositoryContentResponseBody {})
    //             } else {
    //                 Err(crate::github_client::GithubClientError::NetworkError(
    //                     namui::network::http::HttpError::Status {
    //                         status: 409,
    //                         message: "".to_string(),
    //                     },
    //                 ))
    //             }
    //         }
    //     });

    //     let github_storage = ServerStorage::new(mock_client, "master".to_string());

    //     let result = github_storage.start_sync(&mut history_system).await;
    //     assert!(result.is_ok());
    //     assert_eq!(call_count.load(std::sync::atomic::Ordering::Relaxed), 5);
    // }
}
