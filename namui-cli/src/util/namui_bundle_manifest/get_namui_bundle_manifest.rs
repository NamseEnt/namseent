use super::{
    lexicon::{Lexer, NamuiBundleManifest},
    token::Tokenizer,
};
use std::{fs, path::PathBuf};

pub fn get_namui_bundle_manifest(
    project_root_path: &PathBuf,
) -> Result<NamuiBundleManifest, String> {
    let namui_bundle_manifest = project_root_path.join(".namuibundle");
    let namui_bundle_manifest_string = match namui_bundle_manifest.exists() {
        true => fs::read(namui_bundle_manifest)
            .map_err(|error| format!("namui config read error: {}", error))
            .and_then(|file| {
                String::from_utf8(file)
                    .map_err(|error| format!("parse namui_bundle fail: {}", error))
            })?,
        false => String::new(),
    };

    Lexer::new(Tokenizer::new(namui_bundle_manifest_string))
        .parse()
        .map_err(|error| format!("Error while parsing namui bundle list: {}", error))
}
