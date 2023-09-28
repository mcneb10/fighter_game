//! Executor with your game connected to it as a plugin.
use fyrox::engine::executor::Executor;
use client::GameConstructor;
use fyrox::core::wasm_bindgen::{self, prelude::*};
use web_sys::{Storage, Window, WebSocket};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn error(msg: String);

    type Error;

    #[wasm_bindgen(constructor)]
    fn new() -> Error;

    #[wasm_bindgen(structural, method, getter)]
    fn stack(error: &Error) -> String;
}

fn custom_panic_hook(info: &std::panic::PanicInfo) {
    let mut msg = info.to_string();
    msg.push_str("\n\nStack:\n\n");
    let e = Error::new();
    let stack = e.stack();
    msg.push_str(&stack);
    msg.push_str("\n\n");
    error(msg);
}

#[inline]
pub fn set_panic_hook() {
    use std::sync::Once;
    static SET_HOOK: Once = Once::new();
    SET_HOOK.call_once(|| {
        std::panic::set_hook(Box::new(custom_panic_hook));
    });
}

#[wasm_bindgen]
pub fn main() {
    set_panic_hook();
    // Make JS objects
    let mut storage: Storage = web_sys::window().unwrap().local_storage().unwrap().unwrap();
    let mut url: String = storage.get_item("url").unwrap().unwrap();
    let mut ws: WebSocket = WebSocket::new(&url).unwrap();

    ws.send_with_str("{\"hello\": \"world\"}");

    let mut executor = Executor::new();
    executor.add_plugin_constructor(GameConstructor);
    executor.run();
}