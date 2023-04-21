mod part_picker;
mod pose_picker;
mod tooltip;

use super::*;
use crate::color;
use namui_prebuilt::*;
use tooltip::*;

impl CharacterEditor {
    pub fn render(&self, props: Props) -> namui::RenderingTree {
        render([
            self.render_background(props),
            self.render_content(props),
            self.render_tooltip(),
        ])
    }

    fn render_background(&self, props: Props) -> namui::RenderingTree {
        simple_rect(props.wh, color::STROKE_NORMAL, 1.px(), color::BACKGROUND)
            .attach_event(|builder| {
                builder.on_mouse_down_out(|_| {
                    namui::event::send(Event::MouseDownOutsideCharacterEditor)
                });
            })
            .with_tooltip_destroyer(self.tooltip.is_some())
    }

    fn render_content(&self, props: Props) -> namui::RenderingTree {
        match self.edit_target {
            EditTarget::NewCharacterPose => {
                self.render_pose_picker(props.wh, &mock_pose_file_list())
            }
            // EditTarget::ExistingCharacterPose => {
            //     self.render_pose_picker(props.wh, &mock_pose_file_list())
            // }
            EditTarget::ExistingCharacterPart => {
                self.render_part_picker(props.wh, &mock_pose_file(""))
            }
        }
    }

    fn render_tooltip(&self) -> namui::RenderingTree {
        match &self.tooltip {
            Some(tooltip) => tooltip.render(),
            None => RenderingTree::Empty,
        }
    }
}

pub struct PoseFile {
    name: String,
    parts: Vec<PosePart>,
}
struct PosePart {
    name: String,
    variants: Vec<PoseVariant>,
}
struct PoseVariant {
    name: String,
}

fn mock_pose_file_list() -> Vec<PoseFile> {
    vec![
        mock_pose_file("PoseFile Name 0"),
        mock_pose_file("PoseFile Name 1"),
        mock_pose_file("PoseFile Name 2"),
        mock_pose_file("PoseFile Name 3"),
        mock_pose_file("PoseFile Name 4"),
        mock_pose_file("PoseFile Name 5"),
        mock_pose_file("PoseFile Name 6"),
        mock_pose_file("PoseFile Name 7"),
        mock_pose_file("PoseFile Name 8"),
        mock_pose_file("PoseFile Name 9"),
        mock_pose_file("PoseFile Name 10"),
        mock_pose_file("PoseFile Name 11"),
    ]
}

fn mock_pose_file(name: &str) -> PoseFile {
    PoseFile {
        name: name.to_string(),
        parts: vec![
            PosePart {
                name: "PosePart name 0".to_string(),
                variants: vec![
                    PoseVariant {
                        name: "PoseVariant name 0".to_string(),
                    },
                    PoseVariant {
                        name: "PoseVariant name 1".to_string(),
                    },
                    PoseVariant {
                        name: "PoseVariant name 2".to_string(),
                    },
                    PoseVariant {
                        name: "PoseVariant name 3".to_string(),
                    },
                    PoseVariant {
                        name: "PoseVariant name 4".to_string(),
                    },
                    PoseVariant {
                        name: "PoseVariant name 5".to_string(),
                    },
                    PoseVariant {
                        name: "PoseVariant name 6".to_string(),
                    },
                    PoseVariant {
                        name: "PoseVariant name 7".to_string(),
                    },
                    PoseVariant {
                        name: "PoseVariant name 8".to_string(),
                    },
                    PoseVariant {
                        name: "PoseVariant name 9".to_string(),
                    },
                ],
            },
            PosePart {
                name: "PosePart name 1".to_string(),
                variants: vec![
                    PoseVariant {
                        name: "PoseVariant name 0".to_string(),
                    },
                    PoseVariant {
                        name: "PoseVariant name 1".to_string(),
                    },
                    PoseVariant {
                        name: "PoseVariant name 2".to_string(),
                    },
                    PoseVariant {
                        name: "PoseVariant name 3".to_string(),
                    },
                    PoseVariant {
                        name: "PoseVariant name 4".to_string(),
                    },
                    PoseVariant {
                        name: "PoseVariant name 5".to_string(),
                    },
                    PoseVariant {
                        name: "PoseVariant name 6".to_string(),
                    },
                    PoseVariant {
                        name: "PoseVariant name 7".to_string(),
                    },
                    PoseVariant {
                        name: "PoseVariant name 8".to_string(),
                    },
                ],
            },
        ],
    }
}
