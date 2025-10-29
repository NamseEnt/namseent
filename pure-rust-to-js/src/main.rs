use std::{collections::BTreeMap, fmt::Debug};
use syn::*;

fn by_string(a: &String) {}
fn by_string2<T>(a: &T) {}

fn main() {
    let a = Default::default();
    by_string2(&a);
    by_string(&a);

    let file = syn::parse_str::<File>(include_str!("../tower-defense.rs")).unwrap();
    let mut type_visitor = TypeVisitor {
        types: Default::default(),
        mods: Default::default(),
    };
    for item in file.items {
        type_visitor.visit_item(item);
    }

    for (type_path, my_type) in type_visitor.types {
        println!("{type_path:?} -> {my_type:?}");
    }
}

struct TypeVisitor {
    types: BTreeMap<TypePath, MyType>,
    mods: Vec<Ident>,
}

impl TypeVisitor {
    fn visit_item(&mut self, item: Item) {
        match item {
            Item::ExternCrate(_item_extern_crate) => {}
            Item::Use(_item_use) => {}
            Item::Mod(item_mod) => {
                let Some(content) = item_mod.content else {
                    return;
                };
                // todo: push mod
                self.mods.push(item_mod.ident);
                for item in content.1 {
                    self.visit_item(item);
                }
                self.mods.pop();
            }
            Item::Struct(item_struct) => {
                let path = self.type_path(item_struct.ident);
                self.types.insert(
                    path.clone(),
                    MyType::Struct(StructDef {
                        path,
                        fields: Default::default(),
                        generics: Default::default(),
                    }),
                );
            }
            Item::Enum(item_enum) => {
                let type_path = self.type_path(item_enum.ident);
                self.types.insert(type_path, MyType::Enum);
            }
            Item::Const(_item_const) => {}
            Item::Fn(_item_fn) => {}
            Item::ForeignMod(_item_foreign_mod) => {}
            Item::Impl(_item_impl) => {}
            Item::Macro(_item_macro) => {}
            Item::Static(_item_static) => {}
            Item::Trait(_item_trait) => {}
            Item::TraitAlias(_item_trait_alias) => {}
            Item::Type(_item_type) => {}
            Item::Union(_item_union) => {}
            Item::Verbatim(_token_stream) => {}
            _ => todo!(),
        }
    }

    fn type_path(&self, ident: Ident) -> TypePath {
        TypePath {
            mods: self.mods.clone(),
            name: ident,
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone)]
struct TypePath {
    mods: Vec<Ident>,
    name: Ident,
}

impl Debug for TypePath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}::{}",
            self.mods
                .iter()
                .map(|ident| ident.to_string())
                .collect::<Vec<_>>()
                .join("::"),
            self.name
        )
    }
}

#[derive(Debug)]
enum MyType {
    Struct(StructDef),
    Enum,
}

#[derive(Debug)]
struct StructDef {
    path: TypePath,
    fields: Vec<(String, MyType)>,
    generics: Vec<Generic>,
}

#[derive(Debug)]
struct Generic {
    ident: Ident,
    bounds: Vec<GenericBound>,
}

#[derive(Debug)]
struct GenericBound {
    // todo
}
