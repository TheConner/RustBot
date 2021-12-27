use cached::proc_macro::cached;
use std::fs;
use std::path::Path;

use tracing::info;

/// TODO:
/// - Template caching [DONE / WIP]
/// - Template variable injection

const BASE_PATH: &str = "assets/templates";

fn get_template_path(template_name: &str) -> String {
    return format!("{}/{}.md", BASE_PATH, template_name);
}

/// Reads a template file with name `template_name`
/// Returns the template content as a string
#[cached]
fn read_template(template_name: String) -> String {
    info!("Reading template {}", template_name);
    let path = get_template_path(template_name.as_str());
    let data = fs::read_to_string(path).expect("Unable to read file");
    return data;
}

/// Reads a template file with name `template_name`
/// Returns the template content as a string
pub fn template_reader(template_name: &str) -> Option<String> {
    // note: this function exists as a proxy to the cached version of read_template
    // we can't cache functions with &str values, but we can cache methods with string input
    if Path::new(&get_template_path(template_name)).exists() {
        // Path exists, read template and return
        return Some(read_template(template_name.to_string()));
    } else {
        return None;
    }
}
