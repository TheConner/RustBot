use crate::constants::*;
use crate::model::container::ContainerSettings;
use cached::proc_macro::cached;
/// Configuration management for various settings
/// The idea behind this is this design can easily be adapted to hook up to a database for multitenancy
use std::env;

/// Gets a environment configuration string value with a given key
/// If no value is found in environment variables, return default value
fn get_str_config_with_default(key: &str, default: &str) -> String {
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
        }
        Err(_e) => return default,
    }
}

/// Gets a environment configuration bool with a given key
/// If no value is found in environment variables, return default value
fn get_bool_config_with_default(key: &str, default: bool) -> bool {
    let conf = env::var(key);
    match conf {
        Ok(val) => {
            return val.parse::<bool>().unwrap_or(default);
        }
        Err(_e) => {
            return default;
        }
    }
}

/// maybe abstract all this as a struct, provide trait/impl methods to access various fields?
/// this way I don't have to make a method for each field
/// ... or maybe that's just overengineered here

/// This is conditionally compiled if we are in debug mode (local development)
/// For local development, return true
#[cfg(debug_assertions)]
pub fn is_debug() -> bool {
    return true;
}

/// This is conditionally compiled if we are in debug mode (local development)
/// For local development, return true
#[cfg(not(debug_assertions))]
pub fn is_debug() -> bool {
    return false;
}

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

/// Returns the container image to use depending on if we are doing local development
/// or if we are running a relase build
fn get_container_image() -> String {
    if is_debug() {
        // Debug true, use local container image
        return get_str_config_with_default(ENV_CONTAINER_IMAGE, DEFAULT_LOCAL_CONTAINER_IMAGE);
    } else {
        // Debug false, use remote container image
        return get_str_config_with_default(ENV_CONTAINER_IMAGE, DEFAULT_CONTAINER_IMAGE);
    }
}

#[cached]
pub fn get_container_settings() -> ContainerSettings {
    return ContainerSettings {
        cpu: get_str_config_with_default(ENV_CONTAINER_CPU, DEFAULT_CONTAINER_CPU),
        memory: get_str_config_with_default(ENV_CONTAINER_MEMORY, DEFAULT_CONTAINER_MEMORY),
        swap: get_str_config_with_default(ENV_CONTAINER_SWAP, DEFAULT_CONTAINER_SWAP),
        image: get_container_image(),
    };
}

/// Returns true if RustBot is running in a container
#[cached]
pub fn is_container() -> bool {
    return get_bool_config_with_default(
        ENV_IS_RUNNING_IN_CONTAINER,
        DEFAULT_IS_RUNNING_IN_CONTAINER,
    );
}
