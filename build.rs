use askama::Template;
use std::{fs, path::Path, process::Command};

fn main() {
    #[derive(Template)]
    #[template(path = "index.html", escape = "none")]
    struct IndexTemplate {
        js: String,
    }

    // Generate wasm and js
    Command::new("wasm-bindgen")
        .arg("--target")
        .arg("no-modules")
        .arg("--no-typescript")
        .arg("--out-dir")
        .arg("www/scripts")
        .arg("target/wasm32-unknown-unknown/debug/my_page.wasm")
        .output()
        .unwrap();

    // Grab js and store it in a template
    let template_path = Path::new("www").join("scripts").join("my_page.js");
    let js = fs::read_to_string(template_path).expect("we opened my_page.js");
    let index_template = IndexTemplate { js };
    let contents = index_template.render().unwrap();

    let index_path = Path::new("www").join("index.html");
    fs::write(index_path, contents).expect("wrote to index.html");
}
