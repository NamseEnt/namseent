use macro_common_lib::*;
use quote::quote;
use syn::{
    parse::{Parse, ParseStream},
    *,
};

pub enum RelationType {
    OneToOne,
    OneToMany,
    ManyToMany,
}

pub fn relation(
    input: proc_macro::TokenStream,
    relation_type: RelationType,
) -> proc_macro::TokenStream {
    let input: RelationInput = parse_macro_input!(input as RelationInput);

    let output = quote! {};

    output.into()
}

struct RelationInput {
    a: Ident,
    _comma: Token![,],
    b: Ident,
}

impl Parse for RelationInput {
    fn parse(input: ParseStream) -> Result<Self> {
        let a = input.parse()?;
        let comma = input.parse()?;
        let b = input.parse()?;

        Ok(Self {
            a,
            _comma: comma,
            b,
        })
    }
}
