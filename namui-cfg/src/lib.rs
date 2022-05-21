use lazy_static::lazy_static;
use proc_macro::TokenStream;
use rust_cfg_parser::{parse, CfgValue};
use std::{collections::HashMap, env, process::Command};

static NAMUI_CFG_ENV_PREFIX: &str = "NAMUI_CFG_";

type CfgMap = HashMap<String, String>;

lazy_static! {
    static ref CFG_VALUES: Vec<CfgValue> = {
        let mut cfg_map = CfgMap::new();
        load_cfg_from_namui_cli(&mut cfg_map).unwrap();
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

fn load_cfg_from_namui_cli(cfg_map: &mut CfgMap) -> Result<(), Box<dyn std::error::Error>> {
    let mut log = "".to_string();
    for (key, value) in std::env::vars() {
        log += &format!("{} {}\n", key, value);
    }
    panic!("env -> {}", log);
    let namui_cli_print_cfg_output = String::from_utf8(
        Command::new("/home/runner/.cargo/bin/namui")
            .args(["print", "cfg"])
            .output()
            .map_err(|error| format!("Could not run namui-cli {:?}", error))?
            .stdout,
    )?;

    for line in namui_cli_print_cfg_output.lines() {
        let equal_sign_index = line.find("=");
        match equal_sign_index {
            Some(index) => {
                let key = line[..index].to_string();
                let value = line[index + 1..].to_string();
                cfg_map.insert(key, value);
            }
            None => continue,
        };
    }

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
