mod doc_part;
mod doc_part_parsed;
mod document;
mod document_parsed;
mod to_snake_case;

#[proc_macro_attribute]
pub fn document(
    attribute_input: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    document::document(attribute_input, input)
}

#[proc_macro_attribute]
/// use [#recursive] to make the field recursive
pub fn doc_part(
    attribute_input: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    doc_part::doc_part(attribute_input, input)
}
