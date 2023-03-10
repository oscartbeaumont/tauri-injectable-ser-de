// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::time::SystemTime;

use tauri::{
    plugin::{Builder, TauriPlugin},
    Runtime,
};
use tauri_injectable_ser_de::*;

pub struct MyStruct {
    name: String,
    data: Vec<u8>,
    time: u128,
}

// TODO: Do via derive macro
impl IntoBytes for MyStruct {
    fn len_hint(&self) -> u64 {
        self.name.len_hint() + self.data.len_hint() + self.time.len_hint()
    }

    fn bytes_into_buf(self, buf: &mut Vec<u8>) {
        self.name.bytes_into_buf(buf);
        self.data.bytes_into_buf(buf);
        self.time.bytes_into_buf(buf);
    }

    fn decode_impl() -> String {
        format!(
            "let offset = [0]; // Array is so JS is pass by value not pass by reference - Why can't JS just be Rust lmao
            {}", Self::decode_impl_offset()
        )
    }

    fn decode_impl_offset() -> String {
        format!(
            r#"return {{
                "name": {},
                "data": {},
                "time": {},
            }};"#,
            String::decode_impl_offset(),
            Vec::<u8>::decode_impl_offset(),
            u128::decode_impl_offset(),
        )
    }
}

fn my_func() -> MyStruct {
    MyStruct {
        name: "Hello".to_string(),
        data: vec![1, 2, 3, 4, 5],
        time: SystemTime::UNIX_EPOCH.elapsed().unwrap().as_millis(),
    }
}

// Designed to replicate the serialize system within Tauri itself
#[tauri::command]
fn my_func_wrapper() -> Vec<u8> {
    my_func().bytes()
}

// This is an external plugin so it would need a method to collect the used types. This could be done by Specta or through official Tauri integration.
pub fn do_the_thing<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("I did a thing")
        .js_init_script(format!(
            r#"window.INJECTED_DECODE_TABLE = {{
    "my_func_wrapper": (data) => {{ {} }}
}};"#,
            MyStruct::decode_impl()
        ))
        .build()
}

fn main() {
    tauri::Builder::default()
        .plugin(do_the_thing())
        .invoke_handler(tauri::generate_handler![my_func_wrapper])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
