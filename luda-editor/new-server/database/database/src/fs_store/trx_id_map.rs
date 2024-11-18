use bptree::bp_map::BpMap;
use bytes::Bytes;
use std::{path::Path, time::Duration};

// TODO: Add flag which key is checked, to clean up old trx_id
#[derive(Clone)]
pub struct TrxIdMap {
    inner: BpMap,
}
impl TrxIdMap {
    pub async fn new(dir_path: impl AsRef<Path>) -> std::io::Result<Self> {
        let inner = BpMap::new(dir_path.as_ref().join(".trx_id_map"), 32 * 1024).await?;
        Ok(Self { inner })
    }
    pub async fn insert(&self, trx_id: u128, file_ids: Vec<u128>) {
        let file_ids_bytes = file_ids
            .iter()
            .flat_map(|id| id.to_le_bytes().to_vec())
            .collect::<Vec<_>>();

        let bytes = Bytes::from(file_ids_bytes);

        let mut sleep = Duration::from_millis(10);
        for _ in 0..10 {
            let Err(err) = self.inner.insert(trx_id, bytes.clone()).await else {
                return;
            };
            match err {
                bptree::bp_map::Error::Broken => panic!("BpMap Broken"),
                bptree::bp_map::Error::Temporary => {
                    tokio::time::sleep(sleep).await;
                    sleep = (sleep * 2).min(Duration::from_secs(2));
                }
            }
        }
        panic!("BpMap insert failed");
    }

    pub async fn check_trx_id(&self, trx_id: u128, file_id: u128) -> bool {
        let bytes = 'outer: {
            let mut sleep = Duration::from_millis(10);
            for _ in 0..10 {
                match self.inner.get(trx_id).await {
                    Ok(bytes) => {
                        break 'outer bytes;
                    }
                    Err(err) => match err {
                        bptree::bp_map::Error::Broken => panic!("BpMap Broken"),
                        bptree::bp_map::Error::Temporary => {
                            tokio::time::sleep(sleep).await;
                            sleep = (sleep * 2).min(Duration::from_secs(2));
                        }
                    },
                };
            }
            panic!("BpMap get failed");
        };

        let Some(bytes) = bytes else {
            return false;
        };

        bytes
            .chunks_exact(16)
            .map(|chunk| u128::from_le_bytes(chunk.try_into().unwrap()))
            .any(|id| id == file_id)
    }
}
