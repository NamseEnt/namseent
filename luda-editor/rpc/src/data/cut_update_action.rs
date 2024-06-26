use super::*;
use crate::simple_error_impl;
use namui_type::{Angle, Uuid};
use std::marker::PhantomData;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum CutUpdateAction {
    ChangeCharacterName {
        name: String,
    },
    ChangeCutLine {
        line: String,
    },
    PushScreenGraphic {
        graphic_index: Uuid,
        screen_graphic: ScreenGraphic,
    },
    ChangeCgKeepCircumscribed {
        graphic_index: Uuid,
        cg: ScreenCg,
    },
    UnselectCgPart {
        graphic_index: Uuid,
        cg_part_name: String,
    },
    TurnOffCgPartVariant {
        graphic_index: Uuid,
        cg_part_name: String,
        cg_part_variant_name: String,
    },
    TurnOnCgPartVariant {
        graphic_index: Uuid,
        cg_part_name: String,
        cg_part_variant_name: String,
    },
    UpdateCircumscribed {
        graphic_index: Uuid,
        circumscribed: Circumscribed<Percent>,
    },
    ChangeGraphicCircumscribed {
        graphic_index: Uuid,
        circumscribed: Circumscribed<Percent>,
    },
    GraphicFitContain {
        graphic_index: Uuid,
        image_width_per_height_ratio: f32,
    },
    GraphicFitCover {
        graphic_index: Uuid,
        image_width_per_height_ratio: f32,
    },
    SetCut {
        cut: Cut,
    },
    DeleteGraphic {
        graphic_index: Uuid,
    },
    ChangeGraphicOrder(ChangeGraphicOrderAction),
    UpdateGraphicRotation {
        graphic_index: Uuid,
        rotation: Angle,
    },
}
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ChangeGraphicOrderAction {
    pub graphic_index: Uuid,
    pub after_graphic_index: Option<Uuid>,
    prevent_direct_creation: PhantomData<()>,
}
impl ChangeGraphicOrderAction {
    pub fn new(
        graphic_index: Uuid,
        after_graphic_index: Option<Uuid>,
    ) -> Result<Self, ChangeGraphicOrderActionCreateError> {
        if after_graphic_index == Some(graphic_index) {
            return Err(ChangeGraphicOrderActionCreateError::MoveAfterItself);
        }
        Ok(Self {
            graphic_index,
            after_graphic_index,
            prevent_direct_creation: PhantomData,
        })
    }
}
impl From<ChangeGraphicOrderAction> for CutUpdateAction {
    fn from(value: ChangeGraphicOrderAction) -> Self {
        Self::ChangeGraphicOrder(value)
    }
}
simple_error_impl!(ChangeGraphicOrderActionCreateError);
#[derive(Debug)]
pub enum ChangeGraphicOrderActionCreateError {
    MoveAfterItself,
}

impl CutUpdateAction {
    pub fn update(self, cut: &mut crate::data::Cut) {
        match self {
            CutUpdateAction::ChangeCharacterName { name } => {
                cut.character_name = name;
            }
            CutUpdateAction::ChangeCutLine { line } => {
                cut.line = line;
            }
            CutUpdateAction::PushScreenGraphic {
                screen_graphic,
                graphic_index,
            } => {
                cut.screen_graphics.push((graphic_index, screen_graphic));
            }
            CutUpdateAction::ChangeCgKeepCircumscribed { graphic_index, cg } => {
                update_cg(cut, graphic_index, |self_cg| {
                    *self_cg = cg;
                });
            }
            CutUpdateAction::UnselectCgPart {
                graphic_index,
                cg_part_name,
            } => update_cg_part(cut, graphic_index, cg_part_name, |part| part.unselect()),
            CutUpdateAction::TurnOffCgPartVariant {
                graphic_index,
                cg_part_name,
                cg_part_variant_name,
            } => update_cg_part(cut, graphic_index, cg_part_name, |part| {
                part.turn_off(cg_part_variant_name)
            }),
            CutUpdateAction::TurnOnCgPartVariant {
                graphic_index,
                cg_part_name,
                cg_part_variant_name,
            } => update_cg_part(cut, graphic_index, cg_part_name, |part| {
                part.turn_on(cg_part_variant_name)
            }),
            CutUpdateAction::UpdateCircumscribed {
                graphic_index,
                circumscribed,
            } => update_graphic(cut, graphic_index, |graphic| {
                graphic.set_circumscribed(circumscribed)
            }),
            CutUpdateAction::ChangeGraphicCircumscribed {
                graphic_index,
                circumscribed,
            } => {
                update_graphic(cut, graphic_index, |graphic| {
                    graphic.set_circumscribed(circumscribed);
                });
            }
            CutUpdateAction::GraphicFitContain {
                graphic_index,
                image_width_per_height_ratio,
            } => update_graphic(cut, graphic_index, |graphic| {
                let screen_width_per_height_ratio = 4.0 / 3.0;
                let radius = if image_width_per_height_ratio > screen_width_per_height_ratio {
                    let width = 4.0 / 5.0;
                    let height = width / image_width_per_height_ratio;
                    Xy::new(width, height).length()
                } else {
                    let height = 3.0 / 5.0;
                    let width = height * image_width_per_height_ratio;
                    Xy::new(width, height).length()
                };

                let circumscribed = graphic.circumscribed_mut();
                circumscribed.center_xy = Xy::single(50.percent());
                circumscribed.radius = Percent::from(radius);
            }),
            CutUpdateAction::GraphicFitCover {
                graphic_index,
                image_width_per_height_ratio,
            } => update_graphic(cut, graphic_index, |graphic| {
                let screen_width_per_height_ratio = 4.0 / 3.0;
                let radius = if image_width_per_height_ratio > screen_width_per_height_ratio {
                    let height = 3.0 / 5.0;
                    let width = height * image_width_per_height_ratio;
                    Xy::new(width, height).length()
                } else {
                    let width = 4.0 / 5.0;
                    let height = width / image_width_per_height_ratio;
                    Xy::new(width, height).length()
                };

                let circumscribed = graphic.circumscribed_mut();
                circumscribed.center_xy = Xy::single(50.percent());
                circumscribed.radius = Percent::from(radius);
            }),
            CutUpdateAction::SetCut { cut: _cut } => {
                *cut = _cut;
            }
            CutUpdateAction::DeleteGraphic { graphic_index } => {
                if let Some(position) = cut
                    .screen_graphics
                    .iter()
                    .position(|(index, _)| *index == graphic_index)
                {
                    cut.screen_graphics.remove(position);
                }
            }
            CutUpdateAction::ChangeGraphicOrder(ChangeGraphicOrderAction {
                graphic_index,
                after_graphic_index,
                ..
            }) => {
                let Some(moving_graphic_position) = cut
                    .screen_graphics
                    .iter()
                    .position(|(index, _)| index == &graphic_index)
                else {
                    return;
                };
                let moving_graphic = cut.screen_graphics.remove(moving_graphic_position);
                let insert_position = match after_graphic_index {
                    Some(after_graphic_index) => {
                        let Some(position) = cut
                            .screen_graphics
                            .iter()
                            .position(|(index, _)| *index == after_graphic_index)
                        else {
                            return;
                        };
                        position + 1
                    }
                    None => 0,
                };

                cut.screen_graphics.insert(insert_position, moving_graphic);
            }
            CutUpdateAction::UpdateGraphicRotation {
                graphic_index,
                rotation,
            } => update_graphic(cut, graphic_index, |graphic| graphic.set_rotation(rotation)),
        }
    }
}

fn update_graphic(cut: &mut Cut, graphic_index: Uuid, f: impl FnOnce(&mut ScreenGraphic)) {
    if let Some((_, graphic)) = cut
        .screen_graphics
        .iter_mut()
        .find(|(index, _)| *index == graphic_index)
    {
        f(graphic);
    };
}

fn update_cg(cut: &mut Cut, graphic_index: Uuid, f: impl FnOnce(&mut ScreenCg)) {
    update_graphic(cut, graphic_index, |graphic| {
        if let ScreenGraphic::Cg(cg) = graphic {
            f(cg);
        }
    });
}

fn update_cg_part(
    cut: &mut Cut,
    graphic_index: Uuid,
    part_name: String,
    f: impl FnOnce(&mut ScreenCgPart),
) {
    update_cg(cut, graphic_index, |cg| {
        if let Some(part) = cg.parts.iter_mut().find(|part| part.name() == part_name) {
            f(part);
        }
    });
}
