use proc_macro::TokenStream;
use quote::quote;
use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Default)]
struct ModuleNode {
    children: BTreeMap<String, ModuleNode>,
    images: Vec<(String, usize)>,
}

impl ModuleNode {
    fn new() -> Self {
        Self {
            children: BTreeMap::new(),
            images: Vec::new(),
        }
    }
    fn add_image(&mut self, path_parts: &[String], file_name: String, id: usize) {
        if path_parts.is_empty() {
            self.images.push((file_name, id));
        } else {
            let module_name = &path_parts[0];
            let child = self.children.entry(module_name.clone()).or_default();
            child.add_image(&path_parts[1..], file_name, id);
        }
    }
    fn to_tokens(&self) -> proc_macro2::TokenStream {
        let mut modules = Vec::new();

        for (name, child) in &self.children {
            let mod_ident = quote::format_ident!("{}", name);
            let child_tokens = child.to_tokens();
            modules.push(quote! {
                pub mod #mod_ident {
                    use super::Image;
                    #child_tokens
                }
            });
        }

        let mut images = Vec::new();
        for (name, id) in &self.images {
            let const_name = quote::format_ident!("{}", name);
            images.push(quote! {
                pub static #const_name: Image = Image::new(#id);
            });
        }

        quote! {
            #(#modules)*
            #(#images)*
        }
    }
}
fn file_stem_to_const_name(file_stem: &str) -> String {
    file_stem
        .chars()
        .map(|c| {
            if c.is_alphanumeric() {
                c.to_ascii_uppercase()
            } else {
                '_'
            }
        })
        .collect()
}
fn collect_asset_files(asset_dir: &Path) -> Vec<PathBuf> {
    let mut files = Vec::new();

    if !asset_dir.exists() {
        return files;
    }

    fn visit_dirs(dir: &Path, files: &mut Vec<PathBuf>) {
        if let Ok(entries) = fs::read_dir(dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    visit_dirs(&path, files);
                } else if path.is_file() {
                    if let Some(ext) = path.extension() {
                        let ext_str = ext.to_string_lossy().to_lowercase();
                        if ext_str == "jpg" || ext_str == "jpeg" || ext_str == "png" {
                            files.push(path);
                        }
                    }
                }
            }
        }
    }

    visit_dirs(asset_dir, &mut files);
    files
}

#[proc_macro]
pub fn register_assets(_input: TokenStream) -> TokenStream {
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR")
        .expect("CARGO_MANIFEST_DIR environment variable not set");
    let asset_dir = PathBuf::from(&manifest_dir).join("asset");

    let mut asset_files = collect_asset_files(&asset_dir);

    asset_files.sort();
    let mut root = ModuleNode::new();

    for (id, file_path) in asset_files.iter().enumerate() {
        let relative_path = file_path
            .strip_prefix(&asset_dir)
            .expect("Failed to strip asset directory prefix");

        let components: Vec<String> = relative_path
            .parent()
            .map(|p| {
                p.components()
                    .filter_map(|c| c.as_os_str().to_str())
                    .map(|s| s.to_string())
                    .collect()
            })
            .unwrap_or_default();

        let file_stem = relative_path
            .file_stem()
            .and_then(|s| s.to_str())
            .expect("Failed to get file stem");
        let const_name = file_stem_to_const_name(file_stem);

        root.add_image(&components, const_name, id);
    }

    let module_tree = root.to_tokens();

    let expanded = quote! {
        pub mod asset {
            use super::Image;
            #module_tree
        }
    };

    TokenStream::from(expanded)
}
