use std::fs;
use cached::proc_macro::cached;

/// TODO:
/// - Template caching [DONE / WIP]
/// - Template variable injection

/// Reads a template file with name `template_name`
/// Returns the template content as a string
#[cached]
fn read_template(template_name: String) -> String {
    println!("Reading template {}", template_name);
    let path = format!("assets/templates/{}.md",template_name);
    let data = fs::read_to_string(path).expect("Unable to read file");
    return data;
}

pub fn template_reader(template_name: &str) -> String {
    return read_template(template_name.to_string());
}