#[cfg(test)]
mod test;
mod wrap_code_with_block;

use anyhow::Result;
use syn::{spanned::Spanned, visit::Visit};
use wasm_bindgen::prelude::*;
pub use wrap_code_with_block::*;

#[wasm_bindgen]
#[derive(Debug, Clone, Copy)]
pub struct LineColumn {
    pub line: usize,   // 1-based
    pub column: usize, // 0-based
}
#[wasm_bindgen]
impl LineColumn {
    #[wasm_bindgen(constructor)]
    pub fn new(line: usize, column: usize) -> Self {
        Self { line, column }
    }
}

#[derive(Debug, PartialEq, serde::Serialize)]
pub struct EditAction {
    pub insert: Vec<EditInsertAction>,
}

#[derive(Debug, PartialEq, serde::Serialize)]
pub struct EditInsertAction {
    pub line: usize,   // 1-based
    pub column: usize, // 0-based
    pub text: String,
}

#[wasm_bindgen]
pub fn clone_to_closure(
    file_text: &str,
    move_keyword_lc: LineColumn,
    variable_name: &str,
    borrowing_lc: LineColumn,
) -> Result<String, JsValue> {
    serde_json::to_string(
        &clone_to_closure_internal(file_text, move_keyword_lc, variable_name, borrowing_lc)
            .map_err(|e| JsValue::from_str(&format!("{:?}", e)))?,
    )
    .map_err(|e| JsValue::from_str(&format!("{:?}", e)))
}

fn clone_to_closure_internal(
    file_text: &str,
    move_keyword_lc: LineColumn,
    variable_name: &str,
    borrowing_lc: LineColumn,
) -> Result<EditAction> {
    let mut file = syn::parse_file(file_text)?;

    let moving_block_span = find_parent_block_span(&file, move_keyword_lc)?;
    let borrowing_block_span = find_parent_block_span(&file, borrowing_lc)?;

    let is_same_block = moving_block_span.start() == borrowing_block_span.start();

    if is_same_block {
        action_to_create_block_and_put_clone(&mut file, move_keyword_lc, variable_name)
    } else {
        action_to_put_clone_in_block(move_keyword_lc, variable_name)
    }
}

fn action_to_put_clone_in_block(lc_in_stmt: LineColumn, variable_name: &str) -> Result<EditAction> {
    let mut edit_action = EditAction { insert: vec![] };
    edit_action.insert.push(EditInsertAction {
        line: lc_in_stmt.line,
        column: lc_in_stmt.column,
        text: format!("let {variable_name} = {variable_name}.clone();\n"),
    });
    Ok(edit_action)
}

fn action_to_create_block_and_put_clone(
    file: &mut syn::File,
    move_keyword_lc: LineColumn,
    variable_name: &str,
) -> Result<EditAction> {
    let mut visitor = FindClosureSpan {
        lc: move_keyword_lc,
        closure_span: None,
    };
    visitor.visit_file(file);

    let closure_span = visitor.closure_span.ok_or(anyhow::anyhow!(
        "cannot find closure of move keyword at line {}, column {}",
        move_keyword_lc.line,
        move_keyword_lc.column
    ))?;

    let mut edit_action = EditAction { insert: vec![] };
    edit_action.insert.push(EditInsertAction {
        line: closure_span.end().line,
        column: closure_span.end().column,
        text: "\n}".to_string(),
    });
    edit_action.insert.push(EditInsertAction {
        line: closure_span.start().line,
        column: closure_span.start().column,
        text: format!("{{ let {variable_name} = {variable_name}.clone();\n"),
    });
    return Ok(edit_action);

    struct FindClosureSpan {
        lc: LineColumn,
        closure_span: Option<proc_macro2::Span>,
    }

    impl<'ast> syn::visit::Visit<'ast> for FindClosureSpan {
        fn visit_expr(&mut self, node: &syn::Expr) {
            if let syn::Expr::Closure(closure) = &node {
                if let Some(capture) = closure.capture {
                    let span_start = capture.span.start();
                    if span_start.line == self.lc.line && span_start.column == self.lc.column {
                        self.closure_span = Some(closure.span());
                        // let variable_name: syn::Ident = syn::parse_str(self.variable_name).unwrap();

                        // *node = syn::parse_quote!({
                        //     let #variable_name = #variable_name.clone();
                        //     #closure
                        // });
                        return;
                    }
                }
            }

            syn::visit::visit_expr(self, node);
        }
    }
}

pub fn action_to_create_block(expr_span: proc_macro2::Span) -> Result<EditAction> {
    let mut edit_action = EditAction { insert: vec![] };
    edit_action.insert.push(EditInsertAction {
        line: expr_span.end().line,
        column: expr_span.end().column,
        text: "}".to_string(),
    });
    edit_action.insert.push(EditInsertAction {
        line: expr_span.start().line,
        column: expr_span.start().column,
        text: "{".to_string(),
    });
    Ok(edit_action)
}

fn find_parent_block_span(file: &syn::File, lc: LineColumn) -> Result<proc_macro2::Span> {
    let mut visitor = ParentBlockFinder {
        lc,
        last_visit_block: None,
    };
    visitor.visit_file(file);

    return visitor
        .last_visit_block
        .map(|block| block.span())
        .ok_or(anyhow::anyhow!(
            "cannot find parent block of move keyword at line {}, column {}",
            lc.line,
            lc.column
        ));

    struct ParentBlockFinder<'ast> {
        lc: LineColumn,
        last_visit_block: Option<&'ast syn::Block>,
    }

    impl<'ast> syn::visit::Visit<'ast> for ParentBlockFinder<'ast> {
        fn visit_block(&mut self, block: &'ast syn::Block) {
            if block
                .stmts
                .iter()
                .any(|stmt| span_contains_lc(stmt.span(), self.lc))
            {
                self.last_visit_block = Some(block);
            }
            syn::visit::visit_block(self, block);
        }
    }
}

fn span_contains_lc(span: proc_macro2::Span, lc: LineColumn) -> bool {
    let start = span.start();
    let end = span.end();

    (start.line < lc.line && lc.line < end.line)
        || (start.line == lc.line && start.column <= lc.column)
        || (end.line == lc.line && end.column >= lc.column)
}
