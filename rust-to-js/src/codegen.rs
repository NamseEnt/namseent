use crate::ast::*;

/// Generate JavaScript code from MIR AST
pub fn generate(program: &Program) -> String {
    let mut output = String::new();

    // Add helper functions
    output.push_str("// Helper functions\n");
    output.push_str("function new_const(arr) { return arr[0]; }\n\n");

    for item in &program.items {
        match item {
            Item::Function(func) => {
                output.push_str(&generate_function(func));
                output.push('\n');
            }
            Item::Const(const_item) => {
                output.push_str(&generate_const(const_item));
                output.push('\n');
            }
        }
    }

    // Call main() if it exists
    if program.items.iter().any(|item| {
        matches!(item, Item::Function(f) if f.name == "main")
    }) {
        output.push_str("main();\n");
    }

    output
}

fn generate_function(func: &Function) -> String {
    let mut output = String::new();

    output.push_str(&format!("function {}() {{\n", sanitize_name(&func.name)));

    // Declare locals
    for local in &func.locals {
        output.push_str(&format!("    let {} = undefined;\n", local.name));
    }

    if !func.locals.is_empty() {
        output.push('\n');
    }

    // Generate basic blocks
    for block in &func.blocks {
        output.push_str(&format!("    // {}\n", block.label));
        for stmt in &block.statements {
            output.push_str(&format!("    {};\n", generate_statement(stmt)));
        }
    }

    output.push_str("}\n");
    output
}

fn generate_const(const_item: &Const) -> String {
    let mut output = String::new();

    output.push_str(&format!("const {} = (function() {{\n", sanitize_name(&const_item.name)));

    // Declare locals
    for local in &const_item.locals {
        output.push_str(&format!("    let {} = undefined;\n", local.name));
    }

    if !const_item.locals.is_empty() {
        output.push('\n');
    }

    // Generate basic blocks
    for block in &const_item.blocks {
        output.push_str(&format!("    // {}\n", block.label));
        for stmt in &block.statements {
            match stmt {
                Statement::Return => {
                    // For const blocks, return the first local
                    if let Some(local) = const_item.locals.first() {
                        output.push_str(&format!("    return {};\n", local.name));
                    } else {
                        output.push_str("    return undefined;\n");
                    }
                }
                _ => {
                    output.push_str(&format!("    {};\n", generate_statement(stmt)));
                }
            }
        }
    }

    output.push_str("})();\n");
    output
}

fn generate_statement(stmt: &Statement) -> String {
    match stmt {
        Statement::Assign { target, value } => {
            format!("{} = {}", target, generate_expr(value))
        }
        Statement::Return => {
            "return".to_string()
        }
    }
}

fn generate_expr(expr: &Expr) -> String {
    match expr {
        Expr::Const(name) => sanitize_name(name),
        Expr::Copy(name) | Expr::Move(name) => name.clone(),
        Expr::Call { function, args, .. } => {
            let func_name = generate_expr(function);
            let args_str = args
                .iter()
                .map(|arg| generate_expr(arg))
                .collect::<Vec<_>>()
                .join(", ");
            format!("{}({})", func_name, args_str)
        }
        Expr::Array(elements) => {
            let elements_str = elements
                .iter()
                .map(|elem| generate_expr(elem))
                .collect::<Vec<_>>()
                .join(", ");
            format!("[{}]", elements_str)
        }
        Expr::Ref(inner) => {
            // In JS, we just use the value directly
            generate_expr(inner)
        }
        Expr::Path(path) => {
            // For function calls, try to extract just the function name
            // First, remove all generic parameters to find the actual last ::
            let without_generics = {
                let mut result = String::new();
                let mut depth = 0;
                for ch in path.chars() {
                    if ch == '<' {
                        depth += 1;
                    } else if ch == '>' {
                        depth -= 1;
                    } else if depth == 0 {
                        result.push(ch);
                    }
                }
                result
            };

            // Remove trailing :: if any
            let trimmed = without_generics.trim_end_matches("::");

            if let Some(pos) = trimmed.rfind("::") {
                trimmed[pos + 2..].to_string()
            } else {
                trimmed.to_string()
            }
        }
    }
}

fn sanitize_name(name: &str) -> String {
    name.replace("::", "_")
        .replace('[', "_")
        .replace(']', "")
        .replace('<', "_")
        .replace('>', "")
        .replace('\'', "")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sanitize_name() {
        assert_eq!(sanitize_name("main::promoted[0]"), "main_promoted_0");
        assert_eq!(sanitize_name("std::fmt::Arguments<'_>"), "std_fmt_Arguments__");
    }

    #[test]
    fn test_generate_simple_function() {
        let func = Function {
            name: "main".to_string(),
            return_type: Type::Unit,
            locals: vec![
                Local {
                    mutable: true,
                    name: "_0".to_string(),
                    ty: Type::Unit,
                }
            ],
            blocks: vec![
                BasicBlock {
                    label: "bb0".to_string(),
                    statements: vec![Statement::Return],
                }
            ],
        };

        let output = generate_function(&func);
        assert!(output.contains("function main()"));
        assert!(output.contains("let _0 = undefined;"));
        assert!(output.contains("return"));
    }
}
