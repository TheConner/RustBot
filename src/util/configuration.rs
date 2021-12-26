/// Configuration management for various settings
/// The idea behind this is this design can easily be adapted to hook up to a database for multitenancy
use std::{env};
use cached::proc_macro::cached;
use crate::constants::*;
use crate::model::container::{ContainerSettings};

/// Gets a environment configuration string value with a given key
/// If no value is found in environment variables, return default value
fn get_str_config_with_default(key: &str, default: &str)-> String {
    let conf = env::var(key);
    match conf {
        Ok(val) => return val,
        Err(_e) => return String::from(default),
    };
}

/// Gets a environment configuration int value with a given key
/// If no value is found in environment variables, return default value
fn get_u64_config_with_default(key: &str, default: u64) -> u64 {
    let conf = env::var(key);
    match conf {
        Ok(val) => {
            let parsed = val.parse::<u64>();
            match parsed {
                Ok(v) => return v,
                Err(_e) => return default,
            };
        },
        Err(_e) => return default,
    }
}

/// maybe abstract all this as a struct, provide trait/impl methods to access various fields?
/// this way I don't have to make a method for each field
/// ... or maybe that's just overengineered here

#[cached]
pub fn get_bot_prefix() -> String {
    return get_str_config_with_default(ENV_BOT_PREFIX, DEFAULT_PREFIX);
}

#[cached]
pub fn get_bot_token() -> String {
    return get_str_config_with_default(ENV_BOT_TOKEN, "");
}

#[cached]
pub fn get_container_runtime() -> u64 {
    return get_u64_config_with_default(ENV_MAX_RUNTIME, DEFAULT_CONTAINER_RUNTIME);
}

#[cached]
pub fn get_container_settings() -> ContainerSettings {
    return ContainerSettings {
        cpu: get_str_config_with_default(ENV_CONTAINER_CPU, DEFAULT_CONTAINER_CPU),
        memory: get_str_config_with_default(ENV_CONTAINER_MEMORY, DEFAULT_CONTAINER_MEMORY),
        swap: get_str_config_with_default(ENV_CONTAINER_SWAP, DEFAULT_CONTAINER_SWAP)
    };
}