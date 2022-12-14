mod error;
mod method;
mod output;
mod queue_method;
mod rar_method;
mod service;

use error::*;
use method::*;
use proc_macro2::Ident;
use queue_method::*;
use quote::quote;
use rar_method::*;
use service::*;
use syn::{parse::Parse, punctuated::Punctuated, Token};

pub struct Services {
    pub services: Vec<Service>,
}

impl Parse for Services {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Punctuated::<Service, Token![,]>::parse_terminated(&input).map(|services| Self {
            services: services.into_iter().collect(),
        })
    }
}

fn to_snake_case(s: &Ident) -> Ident {
    let mut result = String::new();
    let mut prev_is_upper = false;
    let mut is_first = true;
    for c in s.to_string().chars() {
        if c.is_uppercase() {
            if !prev_is_upper && !is_first {
                result.push('_');
            }
            result.push(c.to_ascii_lowercase());
            prev_is_upper = true;
        } else {
            result.push(c);
            prev_is_upper = false;
        }
        is_first = false;
    }
    Ident::new(&result, s.span())
}
