use super::*;

impl ImageTable {
    pub fn update(&mut self, event: &dyn std::any::Any) {
        if let Some(event) = event.downcast_ref::<InternalEvent>() {
            match event {
                InternalEvent::LoadImages(images) => {
                    self.images = images.clone();
                }
                InternalEvent::LeftClickOnLabelHeader { key } => {
                    self.sort_order_by = match self.sort_order_by.as_ref() {
                        None => Some(SortOrderBy::Ascending { key: key.clone() }),
                        Some(sort_order_by) => match sort_order_by {
                            SortOrderBy::Ascending { key: current_key }
                            | SortOrderBy::Descending { key: current_key }
                                if current_key.ne(key) =>
                            {
                                Some(SortOrderBy::Ascending { key: key.clone() })
                            }
                            SortOrderBy::Ascending { key: current_key } => {
                                Some(SortOrderBy::Descending {
                                    key: current_key.clone(),
                                })
                            }
                            SortOrderBy::Descending { key: _ } => None,
                        },
                    };
                }
                &InternalEvent::LeftClickOnLabelCell {
                    image_id,
                    ref label_key,
                } => {
                    self.editing_target = Some(EditingTarget {
                        image_id,
                        label_key: label_key.clone(),
                    });
                    self.text_input.focus();
                }
            }
        } else if let Some(event) = event.downcast_ref::<text_input::Event>() {
            match event {
                text_input::Event::Focus { id } => {}
                text_input::Event::Blur { id } => {}
                text_input::Event::TextUpdated { id, text } => {
                    if id == self.text_input.get_id() {
                        let editing_target = self.editing_target.as_ref().unwrap();
                        if let Some(image) = self
                            .images
                            .iter_mut()
                            .find(|image| image.id.eq(&editing_target.image_id))
                        {
                            if let Some(label) = image
                                .labels
                                .iter_mut()
                                .find(|label| label.key.eq(&editing_target.label_key))
                            {
                                label.value = text.clone();
                            }
                        }
                    }
                }
                text_input::Event::SelectionUpdated { id, selection } => {}
                text_input::Event::KeyDown { id, code } => {}
            }
        }
        self.list_view.update(event);
    }
}
