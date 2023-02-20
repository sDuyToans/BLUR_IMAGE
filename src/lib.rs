use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::console::log_1 as log;
use base64::{ encode, decode };
use image::load_from_memory;
use image::ImageOutputFormat::Png;

#[wasm_bindgen]
pub fn blur_img(encoded_file: &str) -> String {
    log(&"BLurfn called".into());

    let base64_to_vector = decode(encoded_file).unwrap();
    log(&"Image decoded".into());

    let mut img = load_from_memory(&base64_to_vector).unwrap();
    log(&"Image loaded".into());

    img = img.blur(3.5);
    log(&"Blur effect applied with value 3.5".into());

    let mut buffer = vec![];
    img.write_to(&mut buffer, Png).unwrap();
    log(&"New image written".into());

    let encoded_img = encode(&buffer);
    let data_url = format!(
        "data:image/png;base64, {}",
        encoded_img
    );

    data_url
}   