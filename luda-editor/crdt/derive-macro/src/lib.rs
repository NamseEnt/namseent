use proc_macro::TokenStream;
use quote::ToTokens;

#[proc_macro_attribute]
pub fn history(args: TokenStream, item: TokenStream) -> TokenStream {
    let args = syn::parse_macro_input!(args as syn::AttributeArgs);
    let version: Option<u32> = {
        args.into_iter().find_map(|arg| {
            if let syn::NestedMeta::Meta(syn::Meta::NameValue(meta)) = arg {
                if meta.path.is_ident("version") {
                    if let syn::Lit::Int(lit) = meta.lit {
                        Some(lit.base10_parse().unwrap())
                    } else {
                        unreachable!(
                            "version must be an integer but {:?}",
                            meta.lit.to_token_stream().to_string()
                        );
                    }
                } else {
                    None
                }
            } else {
                None
            }
        })
    };
    let item = syn::parse_macro_input!(item as syn::ItemStruct);
    let struct_name = item.ident.to_string();
    let fields = item
        .fields
        .iter()
        .map(|field| {
            let type_string = field.ty.to_token_stream().to_string();
            Field {
                ident: field.ident.as_ref().unwrap().to_string(),
                ty: match type_string {
                    x if x.starts_with("List <") => FieldType::List,
                    x if x == "String" => FieldType::Primitive(Primitive::String),
                    x if ["i32"].into_iter().any(|ty| x == ty) => {
                        FieldType::Primitive(Primitive::NonString)
                    }
                    x if x.starts_with("Map <") => FieldType::Map,
                    x if x.starts_with("Single <") => FieldType::Single,
                    _ => FieldType::Any,
                },
            }
        })
        .collect::<Box<[_]>>();
    let inserting_to_map = fields
        .iter()
        .map(|Field { ident, ty }| match ty {
            FieldType::List | FieldType::Map | FieldType::Single => {
                format!("self.{ident}.insert_to_map(txn, &head, \"{ident}\");")
            }
            FieldType::Primitive(_) | FieldType::Any => format!(
                "head.insert(txn, \"{ident}\", crdt::Value::from(self.{ident}).into_any());"
            ),
        })
        .collect::<Vec<_>>()
        .join("\n");
    let value_from = fields
        .iter()
        .map(|Field { ident, ty }| match ty {
            FieldType::List | FieldType::Map | FieldType::Single => format!(
                "{ident}: crdt::Value::from_yrs_value(root.get(\"{ident}\").unwrap()).into(),"
            ),
            FieldType::Primitive(_) | FieldType::Any => {
                format!(
                    "{ident}: crdt::Value::from_yrs_value(root.get(\"{ident}\").unwrap()).deserialize(),"
                )
            }
        })
        .collect::<Vec<_>>()
        .join("\n");
    let update_to_map_body = fields
        .iter()
        .map(|Field { ident, ty }| match ty {
            FieldType::Primitive(Primitive::String) =>format!(
                "if self.{ident} != crdt::Value::from_yrs_value(head.get(\"{ident}\").unwrap()).into_string() {{
                    head.insert(txn, \"{ident}\", self.{ident});
                }}"
            ),
            FieldType::List => format!("self.{ident}.update_to_array(txn, &head.get(\"{ident}\").unwrap().to_yarray().unwrap());"),
            FieldType::Map | FieldType::Single => format!("self.{ident}.update_to_map(txn, &head.get(\"{ident}\").unwrap().to_ymap().unwrap());"),
            FieldType::Primitive(Primitive::NonString) | FieldType::Any => format!(
                "let value_{ident} = crdt::Value::from(self.{ident});
                if value_{ident} != crdt::Value::from_yrs_value(head.get(\"{ident}\").unwrap()) {{
                    head.insert(txn, \"{ident}\", value_{ident}.into_any());
                }}"
            ),
        })
        .collect::<Vec<_>>()
        .join("\n");
    let insert_to_root_body = match version {
        Some(version) => {
            format!(
                "
let head = txn.get_map(\"root\");
head.insert(txn, \"__version__\", {version});
{inserting_to_map}"
            )
        }
        None => {
            format!(
                "
let head = txn.get_map(\"root\");
{inserting_to_map}"
            )
        }
    };
    let migrate_body = match version {
        Some(version) => {
            if version == 0 {
                format!(
                    "
let mut txn = doc.transact();
let root = txn.get_map(\"root\");
Self::from_map(&root)"
                )
            } else {
                let prev_version = version - 1;
                format!(
                    "
if version_of_doc == {version} {{
    let mut txn = doc.transact();
    let root = txn.get_map(\"root\");
    Self::from_map(&root)
}} else {{
    let prev = super::system_tree_{prev_version}::SystemTree::migrate(version_of_doc, doc);
    super::system_tree_{version}::migrate(prev)
}}"
                )
            }
        }
        None => "unreachable!()".to_string(),
    };
    let get_version_body = match version {
        Some(version) => {
            format!("Some({version})")
        }
        None => "None".to_string(),
    };
    let result = format!(
        "
impl History for {struct_name} {{
    fn insert_to_array(self, txn: &mut crdt::yrs::Transaction, array: &crdt::yrs::Array, index: u32) {{
        array.insert(txn, index, crdt::yrs::PrelimMap::<bool>::new());
        let head = array.get(index).unwrap().to_ymap().unwrap();
        {inserting_to_map}
    }}
    fn insert_to_map(
        self,
        txn: &mut crdt::yrs::Transaction,
        map: &crdt::yrs::Map,
        key: impl Into<std::rc::Rc<str>>,
    ) {{
        let key: std::rc::Rc<str> = key.into();
        map.insert(txn, key.clone(), crdt::yrs::PrelimMap::<bool>::new());
        let head = map.get(key.as_ref()).unwrap().to_ymap().unwrap();
        {inserting_to_map}
    }}
    fn insert_to_root(self, txn: &mut crdt::yrs::Transaction) {{
        {insert_to_root_body}
    }}
    fn from_map(root: &crdt::yrs::Map) -> Self {{
        {struct_name} {{
            {value_from}
        }}
    }}
    fn update_to_array(self, _txn: &mut crdt::yrs::Transaction, _head: &crdt::yrs::Array) {{
        unreachable!()
    }}
    fn update_to_map(self, txn: &mut crdt::yrs::Transaction, head: &crdt::yrs::Map) {{
        {update_to_map_body}
    }}
    fn from_value(value: crdt::Value) -> Self {{
        if let crdt::yrs::types::Value::YMap(map) = value.yvalue {{
            Self::from_map(&map)
        }} else {{
            unreachable!(\"value is not a map, got {{:?}}\", value);
        }}
    }}
    fn as_value(&self) -> crdt::Value {{
        unreachable!()
    }}
    fn get_version() -> Option<u32>{{
        {get_version_body}
    }}
    fn migrate(version_of_doc: u32, doc: yrs::Doc) -> Self {{
        {migrate_body}
    }}
}}");

    let derive = "#[derive(Debug, Clone)]".parse::<TokenStream>().unwrap();

    let mut output = TokenStream::new();
    output.extend(derive);
    output.extend(TokenStream::from(item.into_token_stream()));
    output.extend(result.parse::<TokenStream>().unwrap());
    output
}

#[derive(Debug)]
enum FieldType {
    Primitive(Primitive),
    List,
    Map,
    Single,
    Any,
}

#[derive(Debug)]
enum Primitive {
    String,
    NonString,
}

#[derive(Debug)]
struct Field {
    ident: String,
    ty: FieldType,
}
