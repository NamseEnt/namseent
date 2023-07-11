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
