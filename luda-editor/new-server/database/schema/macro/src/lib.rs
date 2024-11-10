mod doc_part;
mod doc_part_parsed;
mod document;
mod document_parsed;
mod relation;

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

#[proc_macro]
pub fn relation_one_to_one(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    relation::relation(input, relation::RelationType::OneToOne)
}

#[proc_macro]
pub fn relation_one_to_many(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    relation::relation(input, relation::RelationType::OneToMany)
}

#[proc_macro]
pub fn relation_many_to_many(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    relation::relation(input, relation::RelationType::ManyToMany)
}
