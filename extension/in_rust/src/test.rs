use super::*;
use std::{io::Write, process::Command};

impl EditAction {
    fn apply(&self, text: &mut String) {
        self.insert.iter().for_each(|action| action.apply(text));
    }
}

impl EditInsertAction {
    fn apply(&self, text: &mut String) {
        let index = {
            text.lines()
                .enumerate()
                .filter(|(line_index, _)| *line_index < self.line - 1)
                .fold(0, |acc, (_, line)| acc + line.len() + 1)
                + self.column
        };

        text.insert_str(index, &self.text);
    }
}

fn run_rust_fmt(input: &str) -> String {
    let mut rustfmt = Command::new("rustfmt")
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .spawn()
        .unwrap();

    {
        let mut stdin = rustfmt.stdin.take().unwrap();
        stdin.write_all(input.as_bytes()).unwrap();
        stdin.flush().unwrap();
    }

    let output = rustfmt.wait_with_output().unwrap();
    String::from_utf8(output.stdout).unwrap()
}

#[test]
fn test_clone_to_closure_create_new_block() {
    let file_text = r#"fn main() {
    let x = vec![1];
    let closure = move || {
        println!("x: {:?}", x);
    };
    println!("x: {:?}", x);
}
"#;
    let expected = r#"fn main() {
    let x = vec![1];
    let closure = {
        let x = x.clone();
        move || {
            println!("x: {:?}", x);
        }
    };
    println!("x: {:?}", x);
}
"#;

    let expected_action = EditAction {
        insert: vec![
            EditInsertAction {
                line: 5,
                column: 5,
                text: "\n}".to_string(),
            },
            EditInsertAction {
                line: 3,
                column: 18,
                text: "{ let x = x.clone();\n".to_string(),
            },
        ],
    };

    let actual_action = clone_to_closure_internal(
        file_text,
        LineColumn {
            line: 3,
            column: 18,
        },
        "x",
        LineColumn {
            line: 6,
            column: 24,
        },
    )
    .unwrap();

    assert_eq!(expected_action, actual_action);

    let actual = {
        let mut actual = file_text.to_string();
        actual_action.apply(&mut actual);
        run_rust_fmt(&actual)
    };

    assert_eq!(expected, actual);
}

#[test]
fn test_clone_to_closure_use_existing_block() {
    let file_text = r#"fn main() {
    let x = vec![1];
    let closure = {
        move || {
            println!("x: {:?}", x);
        }
    };
    println!("x: {:?}", x);
}
"#;
    let expected = r#"fn main() {
    let x = vec![1];
    let closure = {
        let x = x.clone();
        move || {
            println!("x: {:?}", x);
        }
    };
    println!("x: {:?}", x);
}
"#;

    let expected_action = EditAction {
        insert: vec![EditInsertAction {
            line: 4,
            column: 8,
            text: "let x = x.clone();\n".to_string(),
        }],
    };

    let actual_action = clone_to_closure_internal(
        file_text,
        LineColumn { line: 4, column: 8 },
        "x",
        LineColumn {
            line: 8,
            column: 24,
        },
    )
    .unwrap();

    assert_eq!(expected_action, actual_action);

    let actual = {
        let mut actual = file_text.to_string();
        actual_action.apply(&mut actual);
        run_rust_fmt(&actual)
    };

    assert_eq!(expected, actual);
}

//     #[test]
//     fn test_clone_to_closure_1() {
//         let file_text = r#"mod cut_cell;
// mod example;
// use crate::color;
// use cut_cell::*;
// use namui::prelude::*;
// use namui_prebuilt::*;
// use rpc::data::{Cut, Memo};
// use std::{collections::HashMap, sync::Arc};
// #[derive(Debug, PartialEq)]
// struct C1 {
//     cut_id: Uuid,
// }
// #[derive(Debug, PartialEq, Hooks)]
// pub struct CutListView {
//     pub wh: Wh<Px>,
//     pub cuts: Vec<Cut>,
//     pub selected_cut_id: Option<Uuid>,
//     pub is_focused: bool,
//     pub cut_id_memo_map: HashMap<Uuid, Vec<Memo>>,
//     pub on_press_enter_on_cut: ClosurePtr<OnPressEnterOnCut, ()>,
//     pub on_move_to_next_cut_by_keyboard: ClosurePtr<OnMoveToNextCutByKeyboard, ()>,
//     pub on_click_cut: ClosurePtr<OnClickCutEvent, ()>,
//     pub on_right_click: ClosurePtr<OnRightClickEvent, ()>,
// }
// pub struct OnPressEnterOnCut {
//     pub cut_id: Uuid,
// }
// pub struct OnMoveToNextCutByKeyboard {
//     pub next_cut_id: Uuid,
// }
// pub struct OnClickCutEvent {
//     pub cut_id: Uuid,
// }
// pub struct OnRightClickEvent {
//     pub global_xy: Xy<Px>,
// }
// impl HooksProps for CutListView {
//     fn render(&self, render_ctx: &mut RenderCtx) {
//         let &Self {
//             wh,
//             ref cuts,
//             selected_cut_id,
//             is_focused,
//             ref cut_id_memo_map,
//             ref on_press_enter_on_cut,
//             ref on_move_to_next_cut_by_keyboard,
//             ref on_click_cut,
//             ref on_right_click,
//         } = self;
//         let cuts = cuts.clone();
//         let press_enter_on_cut = on_press_enter_on_cut.clone();
//         let move_to_next_cut_by_keyboard = on_move_to_next_cut_by_keyboard.clone();
//         let on_click_cut = on_click_cut.clone();
//         let on_right_click = on_right_click.clone();
//         let on_key_down = closure({
//             move |event: KeyboardEvent| {
//                 if !is_focused {
//                     return;
//                 }
//                 let Some (selected_cut_id) = selected_cut_id else { return ; } ;
//                 if event.code == Code::Enter {
//                     press_enter_on_cut.invoke(OnPressEnterOnCut {
//                         cut_id: selected_cut_id,
//                     });
//                 } else {
//                     enum UpDown {
//                         Up,
//                         Down,
//                     }
//                     let direction = match event.code {
//                         Code::ArrowUp => UpDown::Up,
//                         Code::ArrowDown => UpDown::Down,
//                         Code::Tab => {
//                             if namui::keyboard::any_code_press([Code::ShiftLeft, Code::ShiftRight])
//                             {
//                                 UpDown::Up
//                             } else {
//                                 UpDown::Down
//                             }
//                         }
//                         _ => return,
//                     };
//                     let cut_index = cuts
//                         .iter()
//                         .position(|cut| cut.id == selected_cut_id)
//                         .unwrap();
//                     let next_cut_id = match direction {
//                         UpDown::Up => {
//                             if cut_index == 0 {
//                                 return;
//                             }
//                             cuts[cut_index - 1].id
//                         }
//                         UpDown::Down => {
//                             if cut_index == cuts.len() - 1 {
//                                 return;
//                             }
//                             cuts[cut_index + 1].id
//                         }
//                     };
//                     move_to_next_cut_by_keyboard
//                         .clone()
//                         .invoke(OnMoveToNextCutByKeyboard { next_cut_id });
//                 }
//             }
//         });
//         render_ctx.add(
//             render([simple_rect(
//                 wh,
//                 color::STROKE_NORMAL,
//                 1.px(),
//                 color::BACKGROUND,
//             )])
//             .attach_event(move |builder| {
//                 builder
//                     .on_mouse_down_in(move |event: MouseEvent| {
//                         if event.button == Some(MouseButton::Right) {
//                             on_right_click.invoke(OnRightClickEvent {
//                                 global_xy: event.global_xy,
//                             });
//                         }
//                     })
//                     .on_key_down(on_key_down);
//             }),
//         );
//         let on_click_cut_cell = closure(move |cut_id: Uuid| {
//             on_click_cut.invoke(OnClickCutEvent { cut_id });
//         });
//         let item_wh = Wh::new(wh.width, 128.px());
//         render_ctx.add(list_view::ListView2 {
//             xy: Xy::zero(),
//             height: wh.height,
//             scroll_bar_width: 12.px(),
//             item_wh,
//             items: cuts
//                 .iter()
//                 .zip(cuts.iter().map(|cut| cut_id_memo_map.get(&cut.id)))
//                 .enumerate()
//                 .map(|(index, (cut, memos))| {
//                     Arc::new(CutCell {
//                         wh,
//                         index,
//                         cut: cut.clone(),
//                         memo_count: memos.map_or(0, |memos| memos.len()),
//                         is_selected: selected_cut_id == Some(cut.id),
//                         is_focused,
//                         on_click: on_click_cut_cell.clone(),
//                     }) as Arc<dyn Component>
//                 })
//                 .collect(),
//         });
//     }
// }
// "#;

//         let expected = syn::parse_file(
//             r#"mod cut_cell;
// mod example;
// use crate::color;
// use cut_cell::*;
// use namui::prelude::*;
// use namui_prebuilt::*;
// use rpc::data::{Cut, Memo};
// use std::{collections::HashMap, sync::Arc};
// #[derive(Debug, PartialEq)]
// struct C1 {
//     cut_id: Uuid,
// }
// #[derive(Debug, PartialEq, Hooks)]
// pub struct CutListView {
//     pub wh: Wh<Px>,
//     pub cuts: Vec<Cut>,
//     pub selected_cut_id: Option<Uuid>,
//     pub is_focused: bool,
//     pub cut_id_memo_map: HashMap<Uuid, Vec<Memo>>,
//     pub on_press_enter_on_cut: ClosurePtr<OnPressEnterOnCut, ()>,
//     pub on_move_to_next_cut_by_keyboard: ClosurePtr<OnMoveToNextCutByKeyboard, ()>,
//     pub on_click_cut: ClosurePtr<OnClickCutEvent, ()>,
//     pub on_right_click: ClosurePtr<OnRightClickEvent, ()>,
// }
// pub struct OnPressEnterOnCut {
//     pub cut_id: Uuid,
// }
// pub struct OnMoveToNextCutByKeyboard {
//     pub next_cut_id: Uuid,
// }
// pub struct OnClickCutEvent {
//     pub cut_id: Uuid,
// }
// pub struct OnRightClickEvent {
//     pub global_xy: Xy<Px>,
// }
// impl HooksProps for CutListView {
//     fn render(&self, render_ctx: &mut RenderCtx) {
//         let &Self {
//             wh,
//             ref cuts,
//             selected_cut_id,
//             is_focused,
//             ref cut_id_memo_map,
//             ref on_press_enter_on_cut,
//             ref on_move_to_next_cut_by_keyboard,
//             ref on_click_cut,
//             ref on_right_click,
//         } = self;
//         let cuts = cuts.clone();
//         let press_enter_on_cut = on_press_enter_on_cut.clone();
//         let move_to_next_cut_by_keyboard = on_move_to_next_cut_by_keyboard.clone();
//         let on_click_cut = on_click_cut.clone();
//         let on_right_click = on_right_click.clone();
//         let on_key_down = closure({
//             let cuts = cuts.clone();
//             move |event: KeyboardEvent| {
//                 if !is_focused {
//                     return;
//                 }
//                 let Some (selected_cut_id) = selected_cut_id else { return ; } ;
//                 if event.code == Code::Enter {
//                     press_enter_on_cut.invoke(OnPressEnterOnCut {
//                         cut_id: selected_cut_id,
//                     });
//                 } else {
//                     enum UpDown {
//                         Up,
//                         Down,
//                     }
//                     let direction = match event.code {
//                         Code::ArrowUp => UpDown::Up,
//                         Code::ArrowDown => UpDown::Down,
//                         Code::Tab => {
//                             if namui::keyboard::any_code_press([Code::ShiftLeft, Code::ShiftRight])
//                             {
//                                 UpDown::Up
//                             } else {
//                                 UpDown::Down
//                             }
//                         }
//                         _ => return,
//                     };
//                     let cut_index = cuts
//                         .iter()
//                         .position(|cut| cut.id == selected_cut_id)
//                         .unwrap();
//                     let next_cut_id = match direction {
//                         UpDown::Up => {
//                             if cut_index == 0 {
//                                 return;
//                             }
//                             cuts[cut_index - 1].id
//                         }
//                         UpDown::Down => {
//                             if cut_index == cuts.len() - 1 {
//                                 return;
//                             }
//                             cuts[cut_index + 1].id
//                         }
//                     };
//                     move_to_next_cut_by_keyboard
//                         .clone()
//                         .invoke(OnMoveToNextCutByKeyboard { next_cut_id });
//                 }
//             }
//         });
//         render_ctx.add(
//             render([simple_rect(
//                 wh,
//                 color::STROKE_NORMAL,
//                 1.px(),
//                 color::BACKGROUND,
//             )])
//             .attach_event(move |builder| {
//                 builder
//                     .on_mouse_down_in(move |event: MouseEvent| {
//                         if event.button == Some(MouseButton::Right) {
//                             on_right_click.invoke(OnRightClickEvent {
//                                 global_xy: event.global_xy,
//                             });
//                         }
//                     })
//                     .on_key_down(on_key_down);
//             }),
//         );
//         let on_click_cut_cell = closure(move |cut_id: Uuid| {
//             on_click_cut.invoke(OnClickCutEvent { cut_id });
//         });
//         let item_wh = Wh::new(wh.width, 128.px());
//         render_ctx.add(list_view::ListView2 {
//             xy: Xy::zero(),
//             height: wh.height,
//             scroll_bar_width: 12.px(),
//             item_wh,
//             items: cuts
//                 .iter()
//                 .zip(cuts.iter().map(|cut| cut_id_memo_map.get(&cut.id)))
//                 .enumerate()
//                 .map(|(index, (cut, memos))| {
//                     Arc::new(CutCell {
//                         wh,
//                         index,
//                         cut: cut.clone(),
//                         memo_count: memos.map_or(0, |memos| memos.len()),
//                         is_selected: selected_cut_id == Some(cut.id),
//                         is_focused,
//                         on_click: on_click_cut_cell.clone(),
//                     }) as Arc<dyn Component>
//                 })
//                 .collect(),
//         });
//     }
// }
//  "#,
//         )
//         .unwrap()
//         .to_token_stream()
//         .to_string();
//         let actual = clone_to_closure_internal(
//             file_text,
//             LineColumn {
//                 line: 56,
//                 column: 12,
//             },
//             "cuts",
//             LineColumn {
//                 line: 135,
//                 column: 19,
//             },
//         )
//         .unwrap();
//         assert_eq!(expected, actual);
//     }
