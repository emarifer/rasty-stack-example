use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::to_value;
use wasm_bindgen::prelude::{wasm_bindgen, JsValue};
use wasm_bindgen_futures::spawn_local;
use yew::KeyboardEvent;

// use crate::model::Task;

#[wasm_bindgen]
extern "C" {
    // #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "http"], catch)]
    // async fn fetch(url: &str, args: JsValue) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "dialog"])]
    async fn message(content: &str, args: JsValue) -> JsValue;

    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "window.appWindow"])]
    async fn close() -> JsValue;

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

}

// #[derive(Serialize, Deserialize)]
// struct FetchArgs<'a> {
//     method: &'a str,
//     timeout: u64,
// }
//
// pub fn display_data(url: String) {
//     spawn_local(async move {
//         let todos = fetch(
//             url.as_str(),
//             to_value(&FetchArgs {
//                 method: "GET",
//                 timeout: 30,
//             })
//             .unwrap(),
//         )
//         .await;
//
//         match todos {
//             Ok(result_ok) => {
//                 let res = result_ok.clone();
//                 // let res2 = result_ok.clone();
//                 log(&format!("{:?}", from_value::<Vec<Task>>(res).unwrap()));
//                 // data.set(Some(from_value::<Todo>(res2).unwrap()));
//             }
//             Err(result_err) => {
//                 log(&result_err.as_string().unwrap());
//                 // show_message(&result_err.as_string().unwrap(), "error");
//             }
//         }
//     });
// }

#[derive(Serialize, Deserialize)]
struct MessageArgs<'a> {
    title: &'a str,
    #[serde(rename = "type")]
    kind: &'a str,
}

pub fn onclose(e: KeyboardEvent) {
    if e.ctrl_key() && e.key() == "q".to_string() {
        spawn_local(async {
            close().await;
        })
    }
}

pub fn show_message(content: String, kind: &str) {
    // let content = content.to_string();
    let kind = kind.to_string();
    spawn_local(async move {
        message(
            &content,
            to_value(&MessageArgs {
                title: "Todo Tauri Desktop",
                kind: &kind,
            })
            .unwrap(),
        )
        .await;
    });
}
