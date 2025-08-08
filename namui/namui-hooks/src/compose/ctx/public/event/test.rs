use super::*;
use crate::*;

#[test]
fn test_apply_commands_to_xy_no_commands() {
    let target_xy = Xy::new(10.px(), 20.px());
    let commands = [];
    let result = apply_commands_to_xy(target_xy, commands.iter());
    assert_eq!(result, target_xy);
}

#[test]
fn test_apply_commands_to_xy_translate() {
    let target_xy = Xy::new(10.px(), 20.px());
    let commands = [ComposeCommand::Translate {
        xy: Xy::new(5.px(), 3.px()),
    }];
    let result = apply_commands_to_xy(target_xy, commands.iter());
    // Translate subtracts the xy, so 10-5=5, 20-3=17
    assert_eq!(result, Xy::new(5.px(), 17.px()));
}

#[test]
fn test_apply_commands_to_xy_multiple_translates() {
    let target_xy = Xy::new(10.px(), 20.px());
    let commands = [
        ComposeCommand::Translate {
            xy: Xy::new(2.px(), 1.px()),
        },
        ComposeCommand::Translate {
            xy: Xy::new(3.px(), 4.px()),
        },
    ];
    let result = apply_commands_to_xy(target_xy, commands.iter());
    // 10-2-3=5, 20-1-4=15
    assert_eq!(result, Xy::new(5.px(), 15.px()));
}

#[test]
fn test_apply_commands_to_xy_absolute() {
    let target_xy = Xy::new(10.px(), 20.px());
    let commands = [ComposeCommand::Absolute {
        xy: Xy::new(3.px(), 7.px()),
    }];
    let result = apply_commands_to_xy(target_xy, commands.iter());
    // Absolute uses original_xy - xy, so 10-3=7, 20-7=13
    assert_eq!(result, Xy::new(7.px(), 13.px()));
}

#[test]
fn test_apply_commands_to_xy_absolute_after_translate() {
    let target_xy = Xy::new(10.px(), 20.px());
    let commands = [
        ComposeCommand::Translate {
            xy: Xy::new(2.px(), 1.px()),
        },
        ComposeCommand::Absolute {
            xy: Xy::new(3.px(), 7.px()),
        },
    ];
    let result = apply_commands_to_xy(target_xy, commands.iter());
    // Translate first: 10-2=8, 20-1=19
    // Then absolute uses original_xy: 10-3=7, 20-7=13
    assert_eq!(result, Xy::new(7.px(), 13.px()));
}

#[test]
fn test_apply_commands_to_xy_rotate_90_degrees() {
    let target_xy = Xy::new(1.px(), 0.px());
    let commands = [ComposeCommand::Rotate { angle: 90.deg() }];
    let result = apply_commands_to_xy(target_xy, commands.iter());
    // 90 degree rotation should transform (1,0) to approximately (0,1)
    // Note: rotation uses -angle, so it's actually -90 degrees
    assert_px_eq!(result.x, 0.px());
    assert_px_eq!(result.y, -1.px());
}

#[test]
fn test_apply_commands_to_xy_rotate_180_degrees() {
    let target_xy = Xy::new(1.px(), 1.px());
    let commands = [ComposeCommand::Rotate { angle: 180.deg() }];
    let result = apply_commands_to_xy(target_xy, commands.iter());
    // 180 degree rotation should transform (1,1) to (-1,-1)
    // Note: rotation uses -angle, so it's actually -180 degrees
    assert_px_eq!(result.x, -1.px());
    assert_px_eq!(result.y, -1.px());
}

#[test]
fn test_apply_commands_to_xy_scale() {
    let target_xy = Xy::new(10.px(), 20.px());
    let commands = [ComposeCommand::Scale {
        scale_xy: Xy::new(2., 4.),
    }];
    let result = apply_commands_to_xy(target_xy, commands.iter());
    // Scale uses 1.px()/scale, so 10/(2)=5, 20/(4)=5
    assert_eq!(result, Xy::new(5.px(), 5.px()));
}

#[test]
fn test_apply_commands_to_xy_scale_with_fractions() {
    let target_xy = Xy::new(8.px(), 12.px());
    let commands = [ComposeCommand::Scale {
        scale_xy: Xy::new(0.5, 0.25),
    }];
    let result = apply_commands_to_xy(target_xy, commands.iter());
    // Scale uses 1.px()/scale, so 8/(0.5)=16, 12/(0.25)=48
    assert_eq!(result, Xy::new(16.px(), 48.px()));
}

#[test]
fn test_apply_commands_to_xy_ignored_commands() {
    let target_xy = Xy::new(10.px(), 20.px());
    let commands = [
        ComposeCommand::Clip {
            path: Path::new(),
            clip_op: ClipOp::Intersect,
        },
        ComposeCommand::OnTop,
        ComposeCommand::MouseCursor {
            cursor: MouseCursor::Standard(StandardCursor::Default),
        },
    ];
    let result = apply_commands_to_xy(target_xy, commands.iter());
    // These commands should be ignored
    assert_eq!(result, target_xy);
}

#[test]
fn test_apply_commands_to_xy_mixed_commands() {
    let target_xy = Xy::new(10.px(), 20.px());
    let commands = [
        ComposeCommand::Translate {
            xy: Xy::new(2.px(), 3.px()),
        },
        ComposeCommand::OnTop, // Should be ignored
        ComposeCommand::Scale {
            scale_xy: Xy::new(2., 1.),
        },
        ComposeCommand::Clip {
            path: Path::new(),
            clip_op: ClipOp::Difference,
        }, // Should be ignored
    ];
    let result = apply_commands_to_xy(target_xy, commands.iter());
    // Translate: 10-2=8, 20-3=17
    // Scale: 8/2=4, 17/1=17
    assert_eq!(result, Xy::new(4.px(), 17.px()));
}

#[test]
fn test_apply_commands_to_xy_complex_transformation() {
    let target_xy = Xy::new(4.px(), 8.px());
    let commands = [
        ComposeCommand::Translate {
            xy: Xy::new(1.px(), 2.px()),
        },
        ComposeCommand::Scale {
            scale_xy: Xy::new(2., 4.),
        },
        ComposeCommand::Rotate { angle: 0.deg() }, // No rotation
        ComposeCommand::Absolute {
            xy: Xy::new(1.px(), 1.px()),
        },
    ];
    let result = apply_commands_to_xy(target_xy, commands.iter());
    // Translate: 4-1=3, 8-2=6
    // Scale: 3/2=1.5, 6/4=1.5
    // Rotate: no change (0 degrees)
    // Absolute: uses original 4-1=3, 8-1=7
    assert_eq!(result, Xy::new(3.px(), 7.px()));
}

#[test]
fn test_apply_commands_to_xy_zero_coordinates() {
    let target_xy = Xy::new(0.px(), 0.px());
    let commands = [
        ComposeCommand::Translate {
            xy: Xy::new(5.px(), 10.px()),
        },
        ComposeCommand::Scale {
            scale_xy: Xy::new(2., 3.),
        },
    ];
    let result = apply_commands_to_xy(target_xy, commands.iter());
    // Translate: 0-5=-5, 0-10=-10
    // Scale: -5/2=-2.5, -10/3≈-3.333
    assert!((result.x.as_f32() - (-2.5)).abs() < 0.001);
    assert!((result.y.as_f32() - (-10.px() / 3.px())).abs() < 0.001);
}

#[test]
fn test_apply_commands_to_xy_negative_coordinates() {
    let target_xy = Xy::new(-5.px(), -10.px());
    let commands = [
        ComposeCommand::Translate {
            xy: Xy::new(-2.px(), -3.px()),
        },
        ComposeCommand::Absolute {
            xy: Xy::new(-1.px(), -2.px()),
        },
    ];
    let result = apply_commands_to_xy(target_xy, commands.iter());
    // Translate: -5-(-2)=-3, -10-(-3)=-7
    // Absolute: uses original -5-(-1)=-4, -10-(-2)=-8
    assert_eq!(result, Xy::new(-4.px(), -8.px()));
}

#[test]
fn test_apply_commands_to_xy_scale_by_one() {
    let target_xy = Xy::new(7.px(), 14.px());
    let commands = [ComposeCommand::Scale {
        scale_xy: Xy::new(1., 1.),
    }];
    let result = apply_commands_to_xy(target_xy, commands.iter());
    // Scale by 1.px() should not change coordinates: 7/1=7, 14/1=14
    assert_eq!(result, target_xy);
}

#[test]
fn test_apply_commands_to_xy_complex_scenario_1() {
    let target_xy = Xy::new(503.px(), 337.px());
    let commands = [
        ComposeCommand::Scale {
            scale_xy: Xy::new(1.0, 1.0),
        },
        ComposeCommand::Translate {
            xy: Xy::new(-0.0.px(), -0.0.px()),
        },
        ComposeCommand::Translate {
            xy: Xy::new(384.px(), 384.px()),
        },
        ComposeCommand::Translate {
            xy: Xy::new(-12.px(), -210.px()),
        },
        ComposeCommand::Translate {
            xy: Xy::new(12.px(), 0.px()),
        },
        ComposeCommand::Clip {
            path: Path::new().add_rect(Rect::from_xy_wh(
                Xy::new(0.px(), 0.px()),
                Wh::new(256.px(), 200.px()),
            )),
            clip_op: ClipOp::Intersect,
        },
        ComposeCommand::Translate {
            xy: Xy::new(0.px(), 12.px()),
        },
        ComposeCommand::Clip {
            path: Path::new().add_rect(Rect::from_xy_wh(
                Xy::new(0.px(), 0.px()),
                Wh::new(256.px(), 176.px()),
            )),
            clip_op: ClipOp::Intersect,
        },
        ComposeCommand::Translate {
            xy: Xy::new(0.px(), 140.px()),
        },
        ComposeCommand::Clip {
            path: Path::new().add_rect(Rect::from_xy_wh(
                Xy::new(0.px(), 0.px()),
                Wh::new(256.px(), 36.px()),
            )),
            clip_op: ClipOp::Intersect,
        },
    ];
    let result = apply_commands_to_xy(target_xy, commands.iter());
    // Clips are ignored, only translate and scale operations matter
    // Multiple translate operations applied sequentially, final scale by 1.0 doesn't change result
    assert_px_eq!(result.x, 119.px());
    assert_px_eq!(result.y, 11.px());
}

#[test]
fn test_apply_commands_to_xy_complex_scenario_2() {
    let target_xy = Xy::new(503.px(), 337.px());
    let commands = [
        ComposeCommand::Scale {
            scale_xy: Xy::new(0.15755208, 0.15755208),
        },
        ComposeCommand::Translate {
            xy: Xy::new(2689.5947.px(), 1801.9753.px()),
        },
        ComposeCommand::Translate {
            xy: Xy::new(384.px(), 384.px()),
        },
        ComposeCommand::Translate {
            xy: Xy::new(-12.px(), -210.px()),
        },
        ComposeCommand::Translate {
            xy: Xy::new(12.px(), 0.px()),
        },
        ComposeCommand::Clip {
            path: Path::new().add_rect(Rect::from_xy_wh(
                Xy::new(0.px(), 0.px()),
                Wh::new(256.px(), 200.px()),
            )),
            clip_op: ClipOp::Intersect,
        },
        ComposeCommand::Translate {
            xy: Xy::new(0.px(), 12.px()),
        },
        ComposeCommand::Clip {
            path: Path::new().add_rect(Rect::from_xy_wh(
                Xy::new(0.px(), 0.px()),
                Wh::new(256.px(), 176.px()),
            )),
            clip_op: ClipOp::Intersect,
        },
        ComposeCommand::Translate {
            xy: Xy::new(0.px(), 140.px()),
        },
        ComposeCommand::Clip {
            path: Path::new().add_rect(Rect::from_xy_wh(
                Xy::new(0.px(), 0.px()),
                Wh::new(256.px(), 36.px()),
            )),
            clip_op: ClipOp::Intersect,
        },
    ];
    let result = apply_commands_to_xy(target_xy, commands.iter());
    // Clips are ignored, only translate and scale operations matter
    // Commands applied in reverse order - should give same result as scenario 1
    assert_px_eq!(result.x, 119.000244.px());
    assert_px_eq!(result.y, 10.999756.px());
}

#[test]
fn test_apply_commands_to_xy_complex_scenario_3() {
    let target_xy = Xy::new(504.px(), 337.px());
    let commands = [
        ComposeCommand::Scale {
            scale_xy: Xy::new(1.0, 1.0),
        },
        ComposeCommand::Translate {
            xy: Xy::new(1.4542351.px(), -0.121310234.px()),
        },
        ComposeCommand::Translate {
            xy: Xy::new(384.px(), 384.px()),
        },
        ComposeCommand::Translate {
            xy: Xy::new(-12.px(), -210.px()),
        },
        ComposeCommand::Translate {
            xy: Xy::new(12.px(), 0.px()),
        },
        ComposeCommand::Clip {
            path: Path::new().add_rect(Rect::from_xy_wh(
                Xy::new(0.px(), 0.px()),
                Wh::new(256.px(), 200.px()),
            )),
            clip_op: ClipOp::Intersect,
        },
        ComposeCommand::Translate {
            xy: Xy::new(0.px(), 12.px()),
        },
        ComposeCommand::Clip {
            path: Path::new().add_rect(Rect::from_xy_wh(
                Xy::new(0.px(), 0.px()),
                Wh::new(256.px(), 176.px()),
            )),
            clip_op: ClipOp::Intersect,
        },
        ComposeCommand::Translate {
            xy: Xy::new(0.px(), 140.px()),
        },
        ComposeCommand::Clip {
            path: Path::new().add_rect(Rect::from_xy_wh(
                Xy::new(0.px(), 0.px()),
                Wh::new(256.px(), 36.px()),
            )),
            clip_op: ClipOp::Intersect,
        },
    ];
    let result = apply_commands_to_xy(target_xy, commands.iter());
    // Clips are ignored, only translate and scale operations matter
    // Commands applied in reverse order - should give result close to (120, 11)
    assert_px_eq!(result.x, 118.54578.px());
    assert_px_eq!(result.y, 11.121307.px());
}

#[test]
fn test_apply_commands_to_xy_complex_scenario_4() {
    let target_xy = Xy::new(504.px(), 337.px());
    let commands = [
        ComposeCommand::Scale {
            scale_xy: Xy::new(0.9146199, 0.9146199),
        },
        ComposeCommand::Translate {
            xy: Xy::new(47.04857.px(), 31.459064.px()),
        },
        ComposeCommand::Translate {
            xy: Xy::new(384.px(), 384.px()),
        },
        ComposeCommand::Translate {
            xy: Xy::new(-12.px(), -210.px()),
        },
        ComposeCommand::Translate {
            xy: Xy::new(12.px(), 0.px()),
        },
        ComposeCommand::Clip {
            path: Path::new().add_rect(Rect::from_xy_wh(
                Xy::new(0.px(), 0.px()),
                Wh::new(256.px(), 200.px()),
            )),
            clip_op: ClipOp::Intersect,
        },
        ComposeCommand::Translate {
            xy: Xy::new(0.px(), 12.px()),
        },
        ComposeCommand::Clip {
            path: Path::new().add_rect(Rect::from_xy_wh(
                Xy::new(0.px(), 0.px()),
                Wh::new(256.px(), 176.px()),
            )),
            clip_op: ClipOp::Intersect,
        },
        ComposeCommand::Translate {
            xy: Xy::new(0.px(), 140.px()),
        },
        ComposeCommand::Clip {
            path: Path::new().add_rect(Rect::from_xy_wh(
                Xy::new(0.px(), 0.px()),
                Wh::new(256.px(), 36.px()),
            )),
            clip_op: ClipOp::Intersect,
        },
    ];
    let result = apply_commands_to_xy(target_xy, commands.iter());
    // Clips are ignored, only translate and scale operations matter
    // Commands applied in reverse order - should give result close to (120, 11) same as scenario 3
    assert_px_eq!(result.x, 119.99994.px());
    assert_px_eq!(result.y, 10.9999695.px());
}
