use std::process::Command;

fn main() {
    Command::new("wasm-bindgen")
        .arg("--target")
        .arg("no-modules")
        .arg("--no-typescript")
        .arg("--out-dir")
        .arg("www/scripts")
        .arg(
            "target/wasm32-unknown-u\
nknown/debug/my_page.wasm",
        )
        .output()
        .unwrap();
}
