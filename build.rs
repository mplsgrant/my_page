use askama::Template;
use rand::Rng;
use std::{env, fs, path::Path, process::Command, str};

fn main() {
    #[derive(Template)]
    #[template(path = "index.html", escape = "none")]
    struct IndexTemplate {
        id: u32, // Avoid cache annoyances while developing
        js: String,
        wasm64: String,
    }

    let out_dir = env::var_os("OUT_DIR").unwrap();
    println!("cargo:warning={:?}", out_dir);

    // Generate wasm and js
    Command::new("wasm-bindgen")
        .arg("--target")
        .arg("no-modules")
        .arg("--no-typescript")
        .arg("--out-dir")
        .arg("templates/temp_assets/")
        .arg("target/wasm32-unknown-unknown/debug/my_page.wasm")
        .output()
        .expect("wasm-bindgen");

    // TODO Bring base64 in house
    let wasm64 = Command::new("base64")
        .arg("templates/temp_assets/my_page_bg.wasm")
        .output()
        .expect("base64")
        .stdout;
    let wasm64 = str::from_utf8(&wasm64).expect("wasm64").into();

    // Grab js
    let template_path = Path::new("templates")
        .join("temp_assets")
        .join("my_page.js");
    let js = fs::read_to_string(template_path).expect("we opened my_page.js");

    // Pack wasm & js into template
    let mut rnd = rand::thread_rng();
    let id: u32 = rnd.gen();
    let index_template = IndexTemplate { id, js, wasm64 };
    let contents = index_template.render().unwrap();

    // Delete old index.html
    Command::new("rm")
        .arg("www/index.html")
        .output()
        .expect("delete index.html");

    // Create index.html from template
    let index_path = Path::new("www").join("index.html");
    fs::write(index_path, contents).expect("wrote to index.html");
}
