use linked_hash_map::LinkedHashMap;
use luda_editor_rpc::Socket;

const SEQUENCE_INDEX_PATH: &str = "sequence_index.json";

pub struct SequenceIndex {
    sequence_titles: Vec<String>,
}

impl SequenceIndex {
    pub fn new(sequence_titles: Vec<String>) -> Self {
        Self { sequence_titles }
    }

    pub fn sort_title_sequence_map<T>(
        &self,
        title_sequence_map: &LinkedHashMap<String, T>,
    ) -> LinkedHashMap<String, T>
    where
        T: Clone,
    {
        let mut sorted_title_sequence_map: LinkedHashMap<String, T> = LinkedHashMap::new();
        for title in &self.sequence_titles {
            title_sequence_map.get(title).and_then(|sequence| {
                sorted_title_sequence_map.insert(title.clone(), sequence.clone())
            });
        }
        for (title, sequence) in title_sequence_map {
            if sorted_title_sequence_map.contains_key(title) {
                continue;
            }
            sorted_title_sequence_map.insert(title.clone(), sequence.clone());
        }
        sorted_title_sequence_map
    }

    pub async fn save(&self, socket: &Socket) -> Result<(), String> {
        socket
            .write_file(luda_editor_rpc::write_file::Request {
                dest_path: SEQUENCE_INDEX_PATH.to_string(),
                file: serde_json::to_vec(&self.sequence_titles).unwrap(),
            })
            .await
            .and_then(|_| Ok(()))
    }

    pub async fn load(socket: &Socket) -> Result<Self, String> {
        socket
            .read_file(luda_editor_rpc::read_file::Request {
                dest_path: SEQUENCE_INDEX_PATH.to_string(),
            })
            .await
            .and_then(|response| {
                let sequence_titles: Result<Vec<String>, serde_json::Error> =
                    serde_json::from_slice(&response.file);
                match sequence_titles {
                    Ok(sequence_titles) => Ok(sequence_titles),
                    Err(error) => Err(error.to_string()),
                }
            })
            .and_then(|sequence_titles| Ok(Self { sequence_titles }))
    }
}

#[cfg(test)]
mod tests {
    use crate::app::sequence_list::types::SequenceIndex;
    use linked_hash_map::LinkedHashMap;
    use wasm_bindgen_test::wasm_bindgen_test;

    #[test]
    #[wasm_bindgen_test]
    fn sort_indexed_first_then_concat_rest() {
        let index: Vec<String> = vec!["4".to_string(), "2".to_string()];
        let mut map: LinkedHashMap<String, bool> = LinkedHashMap::new();
        map.insert("1".to_string(), true);
        map.insert("2".to_string(), true);
        map.insert("3".to_string(), true);
        map.insert("4".to_string(), true);

        let sorted_map = SequenceIndex::new(index).sort_title_sequence_map(&map);
        let sorted_keys: Vec<String> = sorted_map.keys().map(|key| key.clone()).collect();

        // input:  [1, 2, 3, 4]
        // index:  [4, 2]
        // 1. sort indexed first   [4, 2]  [1, 3]
        // 2. then concat rest     [4, 2, 1, 3]
        // expected:   [4, 2, 1, 3]
        let expected_keys: Vec<String> = vec![
            "4".to_string(),
            "2".to_string(),
            "1".to_string(),
            "3".to_string(),
        ];
        assert_eq!(sorted_keys, expected_keys);
    }

    #[test]
    #[wasm_bindgen_test]
    fn ignore_does_not_exist_in_map() {
        let index: Vec<String> = vec!["4".to_string(), "2".to_string()];
        let mut map: LinkedHashMap<String, bool> = LinkedHashMap::new();
        map.insert("1".to_string(), true);
        map.insert("2".to_string(), true);

        let sorted_map = SequenceIndex::new(index).sort_title_sequence_map(&map);
        let sorted_keys: Vec<String> = sorted_map.keys().map(|key| key.clone()).collect();

        // input:  [1, 2]
        // index:  [4, 2]
        // 1. ignore not existing  [2]  [1]
        // 2. then concat rest     [2, 1]
        // expected:   [2, 1]
        let expected_keys: Vec<String> = vec!["2".to_string(), "1".to_string()];
        assert_eq!(sorted_keys, expected_keys);
    }
}
