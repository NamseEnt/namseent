use crate::types::ErrorMessage;

pub fn print_build_result(
    compile_error_messages: &Vec<ErrorMessage>,
    cli_error_messages: &Vec<String>,
) {
    clear_console();
    if compile_error_messages.is_empty() && cli_error_messages.is_empty() {
        println!("No errors");
        return;
    }
    println!(
        "Errors {}",
        compile_error_messages.len() + cli_error_messages.len()
    );
    for error_message in compile_error_messages {
        println!(
            "{}\n\t\x1b[34m--> {}:{}:{}\x1b[0m\n",
            error_message.text,
            error_message.absolute_file,
            error_message.line,
            error_message.column
        );
    }
    for error_message in cli_error_messages {
        println!("{}\n", error_message);
    }
}

fn clear_console() {
    // #[cfg(not(feature = "cli_debug"))]
    // print!("{}[2J", 27 as char);
}
