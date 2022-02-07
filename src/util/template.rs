use crate::constants::{ENV_BOT_TOKEN, TEMPLATE_BASE_PATH};
use crate::util::configuration::get_str_config_with_default;
use cached::proc_macro::cached;
use regex::Regex;

use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

use tracing::debug;

/// TODO:
/// - Template caching [DONE / WIP]
/// - Template variable injection

/// Builds a full template path from a template name
fn get_template_path(template_name: &str) -> PathBuf {
    Path::new(TEMPLATE_BASE_PATH).join(format!("{}.md", template_name))
}

/// Checks if an interpolation key is allowed, right now it will only return
/// false if you try to use the bot token variable as a template
/// interpolation, which is not allowed as why would you want to leak the bot
/// secret to users?
///
/// In the future this should have a list of allowed values
fn is_interpolation_key_allowed(key: &str) -> bool {
    if key.eq(ENV_BOT_TOKEN) {
        // Cannot leak bot secret in template
        return false;
    }
    true
}

/// Gets a vector of interpolation keywords found in the template
/// For example, if template is
///     Hello {{NAME}} would you like a {{ITEM}}
/// This function will return ['{{NAME}}','{{ITEM}}']
fn get_interpolations(template: &str) -> HashMap<String, String> {
    let re = Regex::new(r"\{\{(.*)\}\}").unwrap();
    let mut interpolations: HashMap<String, String> = HashMap::new();

    // Check to see if we should bother with the rest of this
    if !re.is_match(template) {
        debug!("No interpolations in template, returning");
        return interpolations;
    }

    for cap in re.captures_iter(template) {
        // there must be a less nasty way to do this...
        let template_sub: &str = cap.get(1).map(|m| m.as_str()).unwrap();
        if !interpolations.contains_key(template_sub) && is_interpolation_key_allowed(template_sub)
        {
            interpolations.insert(
                // What a stupid escape system
                // two {{ = one escaped {
                // so if I want to format `TEST` as `{{TEST}}` it looks
                // like this `{{{{{}}}}}`
                format!("{{{{{}}}}}", template_sub),
                get_str_config_with_default(template_sub),
            );
        }
    }

    interpolations
}

fn do_interpolations(template: String, interpolations: HashMap<String, String>) -> String {
    interpolations
        .iter()
        .fold(template, |s, (from, to)| s.replace(from, to))
}

/// Reads a template file with name `template_name`
/// Returns the template content as a string
#[cached]
fn read_template(template_name: String) -> String {
    debug!("Reading template {}", template_name);
    let path = get_template_path(template_name.as_str());
    let mut template = fs::read_to_string(path).expect("Unable to read file");
    let interpolations = get_interpolations(&template);
    if !interpolations.is_empty() {
        debug!("Found interpolations");
        template = do_interpolations(template, interpolations);
    }
    template
}

/// Reads a template file with name `template_name`
/// Returns the template content as a string
pub fn template_reader(template_name: &str) -> Option<String> {
    // note: this function exists as a proxy to the cached version of read_template
    // we can't cache functions with &str values, but we can cache methods with string input
    let template_file = get_template_path(template_name);
    if Path::new(&template_file).exists() {
        debug!("Found template file");
        // Path exists, read template and return
        Some(read_template(template_name.to_string()))
    } else {
        debug!(
            "No template file {}",
            template_file.into_os_string().into_string().unwrap()
        );
        None
    }
}
