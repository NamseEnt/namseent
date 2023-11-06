use syn::{parse::Parse, punctuated::Punctuated, spanned::Spanned};

/*
    # input

    closure!(capture0, capture1, capture2, |param| {
        ...
    });

    # output

    closure((capture0, capture1, capture2), |(capture0, capture1, capture2), param| {
        ...
    });
*/

#[proc_macro]
pub fn closure(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ClosureInput {
        captures,
        rust_closure,
    } = syn::parse_macro_input!(item as ClosureInput);

    let inputs = rust_closure.inputs;
    let body = rust_closure.body;
    let output = quote::quote! {
        closure((#captures), |(#captures), #inputs| {
            #body
        })
    };

    output.into()
}

struct ClosureInput {
    captures: Punctuated<syn::Expr, syn::Token![,]>,
    rust_closure: syn::ExprClosure,
}

impl Parse for ClosureInput {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut macro_params: Punctuated<syn::Expr, syn::Token![,]> =
            Punctuated::parse_terminated(input)?;

        let Some(last_macro_param_pair) = macro_params.pop() else {
            return Err(syn::Error::new(input.span(), "expected closure"));
        };

        let rust_closure =
            if let syn::Expr::Closure(expr_closure) = last_macro_param_pair.into_value() {
                expr_closure
            } else {
                return Err(syn::Error::new(macro_params.span(), "expected closure"));
            };

        let captures: Punctuated<syn::Expr, syn::token::Comma> =
            macro_params.into_iter().collect::<Punctuated<_, _>>();

        Ok(Self {
            captures,
            rust_closure,
        })
    }
}
