use super::*;
use crate::storage::get_project_image_url;
use namui_prebuilt::typography::center_text;
use std::collections::BTreeSet;

const ROW_HEIGHT: Px = px(280.0);
const COLUMN_WIDTH: Px = px(200.0);

impl ImageTable {
    pub fn render(&self, props: Props) -> RenderingTree {
        let project_id = self.project_id;
        let label_keys: BTreeSet<_> = self
            .images
            .iter()
            .flat_map(|image| image.labels.iter().map(|label| label.key.clone()))
            .collect();

        let rows = self.images.iter().map(|image| Row {
            image_id: image.id,
            label_values: label_keys
                .iter()
                .map(|key| {
                    image
                        .labels
                        .iter()
                        .find(|label| &label.key == key)
                        .map(|label| label.value.clone())
                })
                .collect(),
        });

        self.list_view.render(list_view::Props {
            xy: Xy::zero(),
            height: props.wh.height,
            scroll_bar_width: 10.px(),
            item_wh: Wh::new(props.wh.width, ROW_HEIGHT),
            items: rows,
            item_render: |_wh, row| row.render(project_id),
        })
    }
}

struct Row {
    image_id: Uuid,
    label_values: Vec<Option<String>>,
}

impl Row {
    fn render(&self, project_id: Uuid) -> RenderingTree {
        let cell_wh = Wh::new(COLUMN_WIDTH, ROW_HEIGHT);
        let image = namui::try_render(|| {
            let url = get_project_image_url(project_id, self.image_id).unwrap();
            let image = namui::image::try_load_url(&url)?;

            Some(namui::image(ImageParam {
                rect: Rect::from_xy_wh(Xy::zero(), cell_wh),
                source: ImageSource::Image(image),
                style: ImageStyle {
                    fit: ImageFit::Fill,
                    paint_builder: None,
                },
            }))
        });

        render(
            [image]
                .into_iter()
                .chain(self.label_values.iter().map(|label_value| {
                    let border = simple_rect(cell_wh, Color::WHITE, 1.px(), Color::TRANSPARENT);
                    let text = label_value
                        .as_ref()
                        .map(|label_value| {
                            center_text(cell_wh, label_value, Color::WHITE, 18.int_px())
                        })
                        .unwrap_or(RenderingTree::Empty);

                    render([border, text])
                }))
                .enumerate()
                .map(|(index, rendering_tree)| {
                    translate(COLUMN_WIDTH * index, 0.px(), rendering_tree)
                }),
        )
    }
}
