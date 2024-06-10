use namui::*;

pub struct SharePreview {
    pub sequence_id: Uuid,
    pub index: usize,
}

impl SharePreview {
    pub fn from_search(search: &str) -> Option<SharePreview> {
        if search.len() < 2 {
            return None;
        }

        let query_tuples = search[1..].split('&').map(|s| {
            let mut iter = s.split('=');
            let key = iter.next().unwrap_or_default();
            let value = iter.next().unwrap_or_default();
            (key, value)
        });
        let mut sequence_id = None;
        let mut is_view_request = false;
        let mut index = 0;
        for (key, value) in query_tuples {
            if key == "sequence_id" {
                sequence_id = Some(value);
            } else if key == "view" {
                is_view_request = true;
            } else if key == "index" {
                index = value.parse().unwrap_or_default();
            }
        }

        if is_view_request && sequence_id.is_some() {
            Some(SharePreview {
                sequence_id: sequence_id.unwrap().parse().unwrap(),
                index,
            })
        } else {
            None
        }
    }
    #[allow(dead_code)]
    pub fn url(&self) -> String {
        format!(
            "{herf}?view&sequence_id={sequence_id}&index={index}",
            herf = web_sys::window().unwrap().location().href().unwrap(),
            sequence_id = self.sequence_id,
            index = self.index
        )
    }
}
