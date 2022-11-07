use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;

#[proc_macro_attribute]
pub fn version(attribute_input: TokenStream, input: TokenStream) -> TokenStream {
    let attribute_args = parse_macro_input!(attribute_input as syn::AttributeArgs);

    let version = attribute_args
        .iter()
        .find_map(|arg| match arg {
            syn::NestedMeta::Lit(syn::Lit::Int(lit)) => Some(lit.base10_parse::<u32>().unwrap()),
            _ => None,
        })
        .expect("version attribute must have a integer version");

    let input = parse_macro_input!(input as syn::DeriveInput);

    let struct_name = input.ident.clone();

    let named_fields = match input.clone().data {
        syn::Data::Struct(syn::DataStruct {
            fields: syn::Fields::Named(syn::FieldsNamed { named, .. }),
            ..
        }) => named,
        _ => panic!("version attribute can only be used on structs with named fields"),
    };
    let field_count = named_fields.len();

    let fields_serialize = named_fields.iter().map(|field| {
        let field_name = field.ident.as_ref().unwrap();
        quote! {
            state.serialize_field(stringify!(#field_name), &self.#field_name)?;
        }
    });

    let serialize_impl = quote! {
        impl serde::Serialize for #struct_name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                use serde::ser::SerializeStruct;
                let mut state = serializer.serialize_struct(stringify!(#struct_name), #field_count + 1)?;
                state.serialize_field("_v", &#version)?;
                #(#fields_serialize)*
                state.end()
            }
        }
    };

    let fields_deserialize = named_fields.iter().map(|field| {
        let field_name = field.ident.as_ref().unwrap();
        quote! {
            #field_name: serde_json::from_value(json_object.remove(stringify!(#field_name)).unwrap()).unwrap(),
        }
    });

    let deserialize_impl = quote! {
        impl<'de> serde::Deserialize<'de> for #struct_name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                use serde::{de::*, *};
                use serde_json::{Map, Number, Value};
                macro_rules! tri {
                    ($e:expr $(,)?) => {
                        match $e {
                            core::result::Result::Ok(val) => val,
                            core::result::Result::Err(err) => return core::result::Result::Err(err),
                        }
                    };
                }

                enum KeyClass {
                    Map(String),
                }

                struct KeyClassifier;
                impl<'de> DeserializeSeed<'de> for KeyClassifier {
                    type Value = KeyClass;

                    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
                    where
                        D: serde::Deserializer<'de>,
                    {
                        deserializer.deserialize_str(self)
                    }
                }

                impl<'de> Visitor<'de> for KeyClassifier {
                    type Value = KeyClass;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                        formatter.write_str("a string key")
                    }

                    fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
                    where
                        E: serde::de::Error,
                    {
                        match s {
                            _ => Ok(KeyClass::Map(s.to_owned())),
                        }
                    }

                    fn visit_string<E>(self, s: String) -> Result<Self::Value, E>
                    where
                        E: de::Error,
                    {
                        match s.as_str() {
                            _ => Ok(KeyClass::Map(s)),
                        }
                    }
                }

                struct ValueVisitor;
                impl<'de> Visitor<'de> for ValueVisitor {
                    type Value = Value;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                        formatter.write_str("any valid JSON value")
                    }

                    #[inline]
                    fn visit_bool<E>(self, value: bool) -> Result<Value, E> {
                        Ok(Value::Bool(value))
                    }

                    #[inline]
                    fn visit_i64<E>(self, value: i64) -> Result<Value, E> {
                        Ok(Value::Number(value.into()))
                    }

                    #[inline]
                    fn visit_u64<E>(self, value: u64) -> Result<Value, E> {
                        Ok(Value::Number(value.into()))
                    }

                    #[inline]
                    fn visit_f64<E>(self, value: f64) -> Result<Value, E> {
                        Ok(Number::from_f64(value).map_or(Value::Null, Value::Number))
                    }

                    #[inline]
                    fn visit_str<E>(self, value: &str) -> Result<Value, E>
                    where
                        E: serde::de::Error,
                    {
                        self.visit_string(String::from(value))
                    }

                    #[inline]
                    fn visit_string<E>(self, value: String) -> Result<Value, E> {
                        Ok(Value::String(value))
                    }

                    #[inline]
                    fn visit_none<E>(self) -> Result<Value, E> {
                        Ok(Value::Null)
                    }

                    #[inline]
                    fn visit_some<D>(self, deserializer: D) -> Result<Value, D::Error>
                    where
                        D: serde::Deserializer<'de>,
                    {
                        Deserialize::deserialize(deserializer)
                    }

                    #[inline]
                    fn visit_unit<E>(self) -> Result<Value, E> {
                        Ok(Value::Null)
                    }

                    #[inline]
                    fn visit_seq<V>(self, mut visitor: V) -> Result<Value, V::Error>
                    where
                        V: SeqAccess<'de>,
                    {
                        let mut vec = Vec::new();

                        while let Some(elem) = tri!(visitor.next_element()) {
                            vec.push(elem);
                        }

                        Ok(Value::Array(vec))
                    }

                    #[inline]
                    fn visit_map<V>(self, mut visitor: V) -> Result<Value, V::Error>
                    where
                        V: MapAccess<'de>,
                    {
                        match visitor.next_key_seed(KeyClassifier)? {
                            Some(KeyClass::Map(first_key)) => {
                                let mut values = Map::new();

                                values.insert(first_key, tri!(visitor.next_value()));
                                while let Some((key, value)) = tri!(visitor.next_entry()) {
                                    values.insert(key, value);
                                }

                                Ok(Value::Object(values))
                            }
                            None => Ok(Value::Object(Map::new())),
                        }
                    }
                }

                let mut json_value = deserializer.deserialize_any(ValueVisitor)?;
                let json_object = json_value
                    .as_object_mut()
                    .ok_or_else(|| serde::de::Error::custom(format!("expected object")))?;

                let version = json_object
                    .get("_v")
                    .map(|value| {
                        value.as_u64().ok_or_else(|| {
                            serde::de::Error::custom(format!("expected u64, but got {}", value))
                        })
                    })
                    .unwrap_or(Ok(0))?;

                if version < 1 {
                    let previous = serde_json::from_value(json_value).map_err(|e| {
                        serde::de::Error::custom(format!("failed to deserialize previous version: {}", e))
                    })?;
                    Ok(#struct_name::migrate(previous))
                } else {
                    Ok(#struct_name {
                        #(#fields_deserialize)*
                    })
                }
            }
        }
    };

    let output = quote! {
        #input
        #serialize_impl
        #deserialize_impl
    };

    output.into()
}
