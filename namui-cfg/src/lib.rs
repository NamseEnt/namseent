use lazy_static::lazy_static;
use namui_user_config::get_namui_user_config;
use proc_macro::TokenStream;
use rust_cfg_parser::{parse, CfgValue};
use std::{collections::HashMap, env};

static NAMUI_CFG_ENV_PREFIX: &str = "NAMUI_CFG_";

type CfgMap = HashMap<String, String>;

lazy_static! {
    static ref CFG_VALUES: Vec<CfgValue> = {
        let mut cfg_map = CfgMap::new();
        load_cfg_from_user_config(&mut cfg_map).unwrap();
        load_cfg_from_env(&mut cfg_map).unwrap();
        cfg_map
            .iter()
            .map(|(key, value)| match value.is_empty() {
                true => CfgValue::Name(key.clone()),
                false => CfgValue::KeyPair(key.clone(), value.clone()),
            })
            .collect()
    };
}

#[proc_macro_attribute]
pub fn namui_cfg(attr: TokenStream, item: TokenStream) -> TokenStream {
    let expression = parse(&format!("cfg({})", attr.to_string().replace("\n", " "))).unwrap();
    match expression.matches(&CFG_VALUES) {
        true => item,
        false => TokenStream::new(),
    }
}

fn load_cfg_from_user_config(cfg_map: &mut CfgMap) -> Result<(), Box<dyn std::error::Error>> {
    let user_config = get_namui_user_config()?;
    cfg_map.extend(user_config.cfg_map.into_iter());
    Ok(())
}

fn load_cfg_from_env(cfg_map: &mut CfgMap) -> Result<(), Box<dyn std::error::Error>> {
    for (key, value) in env::vars() {
        match key.starts_with(NAMUI_CFG_ENV_PREFIX) {
            true => {
                let key = key[NAMUI_CFG_ENV_PREFIX.len()..].to_ascii_lowercase();
                let value = value.to_string();
                cfg_map.insert(key, value);
            }
            false => continue,
        }
    }

    Ok(())
}
