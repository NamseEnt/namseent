use quote::quote;
use spanned::Spanned;
use syn::*;

#[proc_macro_attribute]
pub fn schema(
    _attribute_input: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let input: syn::DeriveInput = parse_macro_input!(input as syn::DeriveInput);
    let parsed = Parsed::new(&input);
    let struct_name = &input.ident;

    let struct_get_define = struct_get_define(&parsed);
    let struct_put_define = struct_put_define(&parsed);
    let struct_create_define = struct_create_define(&parsed);

    let attrs_removed_input = &parsed.attrs_removed_input;

    let output = quote! {
        #[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)]
        #attrs_removed_input

        impl document::Document for #struct_name {
            fn name() -> &'static str {
                stringify!(#struct_name)
            }

            fn heap_archived(value_buffer: document::ValueBuffer) -> document::HeapArchived<Self> {
                document::HeapArchived::new(value_buffer)
            }

            fn from_bytes(bytes: Vec<u8>) -> document::Result<Self> {
                unsafe { Ok(rkyv::from_bytes_unchecked(&bytes)?) }
            }

            fn to_bytes(&self) -> document::Result<Vec<u8>> {
                Ok(rkyv::to_bytes::<_, 1024>(self)?.to_vec())
            }
        }

        #struct_get_define
        #struct_put_define
        #struct_create_define
    };

    output.into()
}

fn struct_get_define(
    Parsed {
        name,
        pk_fields_without_pk_attr,
        ..
    }: &Parsed,
) -> impl quote::ToTokens {
    let get_struct_name = Ident::new(&format!("{}Get", name), name.span());
    let pk_fields_names = pk_fields_without_pk_attr
        .iter()
        .map(|field| field.ident.as_ref().unwrap());
    let pk_fields_as_refs = pk_fields_without_pk_attr.iter().map(|field| {
        let mut field = field.clone();
        field.ty = Type::Reference(TypeReference {
            and_token: Default::default(),
            lifetime: Some(Lifetime::new("'a", field.span())),
            mutability: None,
            elem: Box::new(field.ty),
        });
        field
    });

    quote! {
        pub struct #get_struct_name<'a> {
            #(#pk_fields_as_refs),*
        }
        impl document::DocumentGet for #get_struct_name<'_> {
            type Output = #name;
            fn key(&self) -> String {
                let mut key = String::new();
                #(
                    key += &format!("{}:{}", stringify!(#pk_fields_names),self.#pk_fields_names);
                )*
                key
            }
        }
    }
}

fn struct_put_define(
    Parsed {
        name,
        pk_fields_without_pk_attr,
        fields_without_pk_attr,
        ..
    }: &Parsed,
) -> impl quote::ToTokens {
    let put_struct_name = Ident::new(&format!("{}Put", name), name.span());
    let put_struct_name_internal = Ident::new(&format!("Internal{}Put", name), name.span());
    let pk_fields_names = pk_fields_without_pk_attr
        .iter()
        .map(|field| field.ident.as_ref().unwrap());
    let fields_as_refs = fields_without_pk_attr
        .iter()
        .map(|field| {
            let mut field = field.clone();
            field.ty = Type::Reference(TypeReference {
                and_token: Default::default(),
                lifetime: Some(Lifetime::new("'a", field.span())),
                mutability: None,
                elem: Box::new(field.ty),
            });
            field
        })
        .collect::<Vec<_>>();
    let fields_names = fields_as_refs
        .iter()
        .map(|field| field.ident.as_ref().unwrap());

    quote! {
        pub struct #put_struct_name<'a> {
            #(#fields_as_refs,)*
            pub ttl: Option<std::time::Duration>,
        }
        #[derive(rkyv::Archive, rkyv::Serialize)]
        struct #put_struct_name_internal<'a> {
            #(
                #[with(rkyv::with::Inline)]
                #fields_as_refs,
            )*
        }
        impl TryInto<document::TransactItem> for #put_struct_name<'_> {
            type Error = document::SerErr;
            fn try_into(self) -> Result<document::TransactItem, document::SerErr> {
                let key = {
                    let mut key = String::new();
                    #(
                        key += &format!("{}:{}", stringify!(#pk_fields_names),self.#pk_fields_names);
                    )*
                    key
                };

                use rkyv::ser::{serializers::AllocSerializer, Serializer};
                let mut serializer = AllocSerializer::<1024>::default();
                serializer.serialize_value(&#put_struct_name_internal{
                    #(
                        #fields_names: self.#fields_names,
                    )*
                })?;
                let value = serializer.into_serializer().into_inner().to_vec();

                Ok(document::TransactItem::Put {
                    key,
                    value,
                    ttl: self.ttl
                })
            }
        }
    }
}

fn struct_create_define(
    Parsed {
        name,
        pk_fields_without_pk_attr,
        fields_without_pk_attr,
        ..
    }: &Parsed,
) -> impl quote::ToTokens {
    let create_struct_name = Ident::new(&format!("{}Create", name), name.span());
    let create_struct_name_internal = Ident::new(&format!("Internal{}Create", name), name.span());
    let pk_fields_names = pk_fields_without_pk_attr
        .iter()
        .map(|field| field.ident.as_ref().unwrap());
    let fields_as_refs = fields_without_pk_attr
        .iter()
        .map(|field| {
            let mut field = field.clone();
            field.ty = Type::Reference(TypeReference {
                and_token: Default::default(),
                lifetime: Some(Lifetime::new("'a", field.span())),
                mutability: None,
                elem: Box::new(field.ty),
            });
            field
        })
        .collect::<Vec<_>>();
    let fields_names = fields_as_refs
        .iter()
        .map(|field| field.ident.as_ref().unwrap());

    quote! {
        pub struct #create_struct_name<'a> {
            #(#fields_as_refs,)*
            pub ttl: Option<std::time::Duration>,
        }
        #[derive(rkyv::Archive, rkyv::Serialize)]
        struct #create_struct_name_internal<'a> {
            #(
                #[with(rkyv::with::Inline)]
                #fields_as_refs,
            )*
        }
        impl TryInto<document::TransactItem> for #create_struct_name<'_> {
            type Error = document::SerErr;
            fn try_into(self) -> Result<document::TransactItem, document::SerErr> {
                let key = {
                    let mut key = String::new();
                    #(
                        key += &format!("{}:{}", stringify!(#pk_fields_names),self.#pk_fields_names);
                    )*
                    key
                };

                Ok(document::TransactItem::Create {
                    key,
                    value: {
                        use rkyv::ser::{serializers::AllocSerializer, Serializer};
                        let mut serializer = AllocSerializer::<1024>::default();
                        serializer.serialize_value(&#create_struct_name_internal{
                            #(
                                #fields_names: self.#fields_names,
                            )*
                        })?;
                        serializer.into_serializer().into_inner().to_vec()
                    },
                    ttl: self.ttl
                })
            }
        }
    }
}

struct Parsed<'a> {
    name: &'a Ident,
    attrs_removed_input: DeriveInput,
    pk_fields_without_pk_attr: Vec<Field>,
    fields_without_pk_attr: Vec<Field>,
}

impl<'a> Parsed<'a> {
    fn new(input: &'a DeriveInput) -> Self {
        let name = &input.ident;
        let mut attrs_removed_input = input.clone();
        let mut pk_fields_without_pk_attr = Vec::new();
        let mut fields_without_pk_attr = Vec::new();
        {
            let struct_input = match &mut attrs_removed_input.data {
                Data::Struct(data) => data,
                _ => unreachable!(),
            };
            struct_input.fields.iter_mut().for_each(|field| {
                if field.attrs.iter().any(|attr| attr.path.is_ident("pk")) {
                    field.attrs.retain(|attr| !attr.path.is_ident("pk"));
                    pk_fields_without_pk_attr.push(field.clone());
                }
                fields_without_pk_attr.push(field.clone());
            });
        }

        Self {
            name,
            attrs_removed_input,
            pk_fields_without_pk_attr,
            fields_without_pk_attr,
        }
    }
}
