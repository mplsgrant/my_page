use bitcoin::{secp256k1::Secp256k1, KeyPair, Network, PrivateKey};
use wasm_bindgen::prelude::*;

// Called by our JS entry point to run the example
#[wasm_bindgen(start)]
pub fn run() -> Result<(), JsValue> {
    // Use `web_sys`'s global `window` function to get a handle on the global
    // window object.
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");

    // Manufacture the element we're gonna append
    let val = document.create_element("p")?;
    val.set_text_content(Some("Hello from Rust!"));

    body.append_child(&val)?;

    // Write to console.log
    let a: JsValue = "hello from run()".into();
    web_sys::console::log_1(&a);

    // Bitcoin key pair
    let secp = Secp256k1::new();
    let s = [1u8; 32]; // 🤦
    let priv_k = PrivateKey::from_slice(&s, Network::Regtest).expect("priv_k");
    let pub_k = priv_k.public_key(&secp);
    let a: JsValue = format!("{}", pub_k).as_str().into();
    web_sys::console::log_1(&a);

    let p_public_key = document.create_element("p")?;
    val.set_text_content(Some(format!("Have you seen me? {}", pub_k).as_str()));

    Ok(())
}

#[wasm_bindgen]
pub fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}
