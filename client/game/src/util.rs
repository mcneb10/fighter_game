use std::future::Future;
use std::net::TcpStream;
use wasm_bindgen::JsValue;
use web_sys::{Storage, Window};
/*use websocket::ClientBuilder;
use websocket::sync::Client;*/

pub async fn get_local_storage() -> Storage {
    let w: Window = web_sys::window().unwrap();
    let s: Result<Option<Storage>, JsValue> = w.local_storage();
    s.unwrap().unwrap()
}

/*
pub async fn make_ws_stuff(storage: impl Future<Output = Storage>) -> Client<TcpStream> {
    return ClientBuilder::new(&*((storage.await.get_item("url").unwrap().unwrap())))
        .unwrap()
        .connect_insecure()
        .unwrap();
}*/