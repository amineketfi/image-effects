use wasm_bindgen::prelude::*;
use web_sys::console::log_1 as log;
use base64::{decode, encode};
use image::{load_from_memory};
use image::ImageOutputFormat::Png;

use std::io::Cursor;



#[wasm_bindgen]
pub fn grayscale(encoded_file: &str) -> String {
    // into function is smart enough to detect the type we want to convert to
    log(&"Grayscale called".into());

    let base64_to_vector = decode(encoded_file).unwrap();
    log(&"Image decoded".into());

    let mut img = load_from_memory(&base64_to_vector).unwrap();
    log(&"Image loaded".into());

    img = img.grayscale();
    log(&"Grayscale effect applied".into());

    let mut buffer: Cursor<Vec<u8>> = Cursor::new(Vec::new());

    img.write_to(&mut buffer, Png).unwrap();
    log(&"New image written".into());

    let buffer_ref = buffer.get_ref();


    let encoded_img = encode(&buffer_ref);
    let data_url = format!(
        "data:image/png;base64,{}",
        encoded_img
    );

    data_url
}