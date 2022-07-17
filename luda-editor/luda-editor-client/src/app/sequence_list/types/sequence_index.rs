use crate::app::storage::Storage;
use futures::TryFutureExt;
use linked_hash_map::LinkedHashMap;
use std::sync::Arc;

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
        sorted_title_sequence_map
    }

    pub async fn save(&self, storage: &Arc<Storage>) -> Result<(), String> {
        storage
            .put_sequence_titles(&self.sequence_titles)
            .map_err(|error| format!("failed to save sequence index: {:#?}", error))
            .await?;
        Ok(())
    }

    pub async fn load(storage: &Arc<Storage>) -> Result<Self, String> {
        let sequence_titles = storage
            .get_sequence_titles()
            .map_err(|error| format!("failed to load sequence index: {:#?}", error))
            .await?;
        Ok(Self { sequence_titles })
    }
}

#[cfg(test)]
mod tests {
    use crate::app::sequence_list::types::SequenceIndex;
    use linked_hash_map::LinkedHashMap;
    use wasm_bindgen_test::wasm_bindgen_test;

    #[test]
    #[wasm_bindgen_test]
    fn sort_indexed_then_ignore_rest() {
        let index: Vec<String> = vec!["4".to_string(), "2".to_string()];
        let mut map: LinkedHashMap<String, bool> = LinkedHashMap::new();
        map.insert("1".to_string(), true);
        map.insert("2".to_string(), true);
        map.insert("3".to_string(), true);
        map.insert("4".to_string(), true);

        let sorted_map = SequenceIndex::new(index).sort_title_sequence_map(&map);
        let sorted_keys: Vec<String> = sorted_map.keys().map(|key| key.clone()).collect();

        // input:       [1, 2, 3, 4]
        // index:       [4, 2]
        // expected:    [4, 2]
        let expected_keys: Vec<String> = vec!["4".to_string(), "2".to_string()];
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

        // input:       [1, 2]
        // index:       [4, 2]
        // expected:    [2]
        let expected_keys: Vec<String> = vec!["2".to_string()];
        assert_eq!(sorted_keys, expected_keys);
    }
}
