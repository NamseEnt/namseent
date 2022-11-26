use super::*;
use std::ops::Index;

impl LoadedSequenceEditorPage {
    pub fn copy_issue_info(&self, cut_id: Uuid) {
        let text = self.generate_issue_info(cut_id);
        spawn_local(async move {
            let result = namui::system::clipboard::write_text(text).await;
            if let Err(_) = result {
                namui::event::send(Event::Error("Failed to copy to clipboard".to_string()));
            }
        })
    }

    fn generate_issue_info(&self, cut_id: Uuid) -> String {
        let sequence_title = &self.sequence.name;

        let range = 10;
        let cut_index = self
            .sequence
            .cuts
            .iter()
            .position(|c| c.id() == cut_id)
            .unwrap();

        let front_index = cut_index.checked_sub(range).unwrap_or_default();
        let back_index = (cut_index + range).min(self.sequence.cuts.len());

        let collapsed_front = self.sequence.cuts.index(0..front_index);
        let in_range_cuts = self.sequence.cuts.index(front_index..back_index);
        let collapsed_back = self.sequence.cuts.index(back_index..);

        format!(
            "# 시퀀스 제목: {sequence_title}

{front}

{cuts}

{back}
",
            front = self.get_collapsed_text(collapsed_front, cut_id),
            cuts = self.cuts_to_text(in_range_cuts, cut_id),
            back = self.get_collapsed_text(collapsed_back, cut_id),
        )
    }

    fn get_collapsed_text(&self, collapsed_cuts: &[Cut], selected_cut_id: Uuid) -> String {
        if collapsed_cuts.is_empty() {
            return "".to_string();
        }

        format!(
            "<details>
<summary>중략 더보기</summary>

{}

</details>",
            self.cuts_to_text(collapsed_cuts, selected_cut_id)
        )
    }

    fn cuts_to_text(&self, cuts: &[Cut], selected_cut_id: Uuid) -> String {
        let mut text = String::new();
        for cut in cuts {
            text.push_str(if cut.id() == selected_cut_id {
                "- 🚩--> "
            } else {
                "- "
            });

            let character_id = cut.character_id;
            let character_name = character_id.and_then(|character_id| {
                self.project_shared_data
                    .characters
                    .iter()
                    .find(|c| c.id() == character_id)
                    .map(|c| &c.name)
            });
            let line = &cut.line;
            if let Some(character_name) = character_name {
                text.push_str(&format!("{character_name}\n  - {line}\n"));
            } else {
                text.push_str(&format!("{line}\n"));
            }
        }
        text
    }
}
