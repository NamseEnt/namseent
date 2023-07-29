use crate::{action_to_create_block, span_contains_lc, EditAction, LineColumn};
use anyhow::Result;
use syn::{spanned::Spanned, visit::Visit};
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

#[wasm_bindgen]
pub fn position_is_in_async_block(file_text: &str, position: LineColumn) -> Result<bool, JsValue> {
    let file = syn::parse_file(file_text).map_err(|e| JsValue::from_str(&format!("{:?}", e)))?;
    Ok(find_cursor_located_expr_span(&file, position, ExprType::Async).is_ok())
}

#[wasm_bindgen]
pub fn position_is_in_closure(file_text: &str, position: LineColumn) -> Result<bool, JsValue> {
    let file = syn::parse_file(file_text).map_err(|e| JsValue::from_str(&format!("{:?}", e)))?;
    Ok(find_cursor_located_expr_span(&file, position, ExprType::Closure).is_ok())
}

#[wasm_bindgen]
pub fn wrap_async_block_in_block(file_text: &str, position: LineColumn) -> Result<String, JsValue> {
    serde_json::to_string(
        &wrap_code_in_block_internal(file_text, position, ExprType::Async)
            .map_err(|e| JsValue::from_str(&format!("{:?}", e)))?,
    )
    .map_err(|e| JsValue::from_str(&format!("{:?}", e)))
}

#[wasm_bindgen]
pub fn wrap_closure_in_block(file_text: &str, position: LineColumn) -> Result<String, JsValue> {
    serde_json::to_string(
        &wrap_code_in_block_internal(file_text, position, ExprType::Closure)
            .map_err(|e| JsValue::from_str(&format!("{:?}", e)))?,
    )
    .map_err(|e| JsValue::from_str(&format!("{:?}", e)))
}

fn wrap_code_in_block_internal(
    file_text: &str,
    position: LineColumn,
    expr_type: ExprType,
) -> Result<EditAction> {
    let file = syn::parse_file(file_text)?;
    let cursor_located_expr_span = find_cursor_located_expr_span(&file, position, expr_type);
    action_to_create_block(cursor_located_expr_span?)
}

fn find_cursor_located_expr_span(
    file: &syn::File,
    lc: LineColumn,
    expr_type: ExprType,
) -> Result<proc_macro2::Span> {
    let mut visitor = CursorLocatedExprFinder {
        lc,
        last_visit_expr: None,
        expr_type,
    };
    visitor.visit_file(file);

    return visitor
        .last_visit_expr
        .map(|expr| expr.span())
        .ok_or(anyhow::anyhow!(
            "cannot find expr at line {}, column {}",
            lc.line,
            lc.column
        ));

    struct CursorLocatedExprFinder<'ast> {
        lc: LineColumn,
        last_visit_expr: Option<&'ast syn::Expr>,
        expr_type: ExprType,
    }

    impl<'ast> syn::visit::Visit<'ast> for CursorLocatedExprFinder<'ast> {
        fn visit_expr(&mut self, expr: &'ast syn::Expr) {
            if self.expr_type.is_same_type_with(expr) && span_contains_lc(expr.span(), self.lc) {
                self.last_visit_expr = Some(expr);
            }

            syn::visit::visit_expr(self, expr);
        }
    }
}

enum ExprType {
    Async,
    Closure,
}
impl ExprType {
    fn is_same_type_with(&self, other: &syn::Expr) -> bool {
        match (self, other) {
            (Self::Async, syn::Expr::Async(..)) | (Self::Closure, syn::Expr::Closure(..)) => true,
            _ => false,
        }
    }
}
