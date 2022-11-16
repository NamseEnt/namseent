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
            }
        }
        self.list_view.update(event);
    }
}
