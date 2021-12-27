use std::env;
use std::path::Path;
use std::process::Command;

fn main() {
    let base_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    println!("Base dir {}", base_dir);
    let profile = env::var("PROFILE").unwrap();
    println!("Profile {}", profile);
    
    let release_dir = Path::new(&base_dir)
    .join("target")
    .join(profile); 
    println!("Release dir {}", release_dir.to_str().unwrap());

    let template_dir = release_dir
        .join("assets");
    println!("Create output dir {}", template_dir.to_str().unwrap());
    // create output directory for build
    Command::new("mkdir")
        .args(&["-p", template_dir.to_str().unwrap()])
        .status()
        .unwrap();

    // copy container assets to output directory
    Command::new("cp")
        .args(&["-r", "assets/templates", template_dir.to_str().unwrap()])
        .status()
        .unwrap();

    // Copy README for package
    Command::new("cp")
        .args(&["package-readme.txt", release_dir.join("README.txt").to_str().unwrap()])
        .status()
        .unwrap();

    // Copy licenses 
    Command::new("cp")
        .args(&["LICENSE-MIT", release_dir.to_str().unwrap()])
        .status()
        .unwrap();

    Command::new("cp")
        .args(&["LICENSE-APACHE", release_dir.to_str().unwrap()])
        .status()
        .unwrap();
}
