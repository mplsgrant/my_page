use askama::Template;
use rand::Rng;

use std::{env::current_dir, fs, path::Path, process::Command, str};

use clap::{arg, command};

fn main() {
    let matches = command!() // requires `cargo` feature
        .arg(arg!([name] "debug or release?").required(true))
        .get_matches();

    // You can check the value provided by positional arguments, or option arguments
    let user_input = matches.get_one::<String>("name").unwrap().as_str();

    let target_dir = match user_input {
        "debug" => "../target/wasm32-unknown-unknown/debug/page.wasm",
        "release" => "../target/wasm32-unknown-unknown/release/page.wasm",
        _ => panic!(),
    };

    #[derive(Template)]
    #[template(path = "index.html", escape = "none")]
    struct IndexTemplate {
        id: u32, // Avoid cache annoyances while developing
        js: String,
        wasm64: String,
    }

    let pwd = current_dir().unwrap();

    // Create a folder for temp assets
    let new_dir = pwd.join("templates").join("temp_assets");
    fs::create_dir(new_dir).expect("make a new dir");

    // Generate wasm and js
    let wasm_bindgen = Command::new("wasm-bindgen")
        .arg("--target")
        .arg("no-modules")
        .arg("--no-typescript")
        .arg("--out-dir")
        .arg("templates/temp_assets/")
        .arg(target_dir)
        .output()
        .expect("wasm-bindgen")
        .status;
    assert!(wasm_bindgen.success());

    // TODO Bring base64 in house
    let wasm64 = Command::new("base64")
        .arg("templates/temp_assets/page_bg.wasm")
        .output()
        .expect("base64")
        .stdout;
    let wasm64 = str::from_utf8(&wasm64).expect("wasm64").into();

    // Grab js
    let template_path = Path::new("templates").join("temp_assets").join("page.js");
    let js = fs::read_to_string(template_path).expect("we opened_page.js");

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

    // Delete temporary assets directory
    Command::new("rm")
        .arg("-rf")
        .arg("templates/temp_assets")
        .output()
        .expect("delete index.html");

    // Create index.html from template
    let index_path = Path::new("www").join("index.html");
    fs::write(index_path, contents).expect("wrote to index.html");
}
