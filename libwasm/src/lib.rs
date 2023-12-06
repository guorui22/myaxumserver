use std::collections::HashMap;

use image::{DynamicImage, ImageBuffer, imageops};
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::Error;
use wasm_bindgen::Clamped;
use wasm_bindgen::prelude::*;
use web_sys::{HtmlCanvasElement, ImageData};

mod utils;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, libwasm!");
}

/// 返回字符串
#[wasm_bindgen]
pub fn get_str() -> String {
    "Hello, libwasm!".to_string()
}

/// 返回字符串格式的 JSON
#[wasm_bindgen]
pub fn get_json() -> String {
    r#"{
        "status": 0,
        "result": "token"
    }"#.to_string()
}

/// 支持反序列化为 JSON 对象的 Rust 结构体
#[derive(Serialize, Deserialize, Debug)]
pub struct Example {
    pub field1: HashMap<u32, String>,
    pub field2: Vec<Vec<f32>>,
    pub field3: [f32; 4],
}

/// 返回 JSON 对象
#[wasm_bindgen]
pub fn get_jval() -> Result<JsValue, Error> {
    let mut field1 = HashMap::new();
    field1.insert(0, String::from("ex"));

    let example = Example {
        field1,
        field2: vec![vec![1., 2.], vec![3., 4.]],
        field3: [1., 2., 3., 4.]
    };
    serde_wasm_bindgen::to_value(&example)
}

/// 输入 JSON 对象
#[wasm_bindgen]
pub fn set_jval(val: JsValue) -> String {

    let random_string:Result<Example, Error> = serde_wasm_bindgen::from_value(val);

    format!("对象结构：{:?}", random_string)

}

fn to_image(width: u32, height: u32, vec: Vec<u8>) -> DynamicImage {
    let img_buffer = ImageBuffer::from_vec(width, height, vec).unwrap();
    DynamicImage::ImageRgba8(img_buffer)
}

fn to_canvas_image(width: u32, height: u32, vec: Vec<u8>) -> ImageData {
    ImageData::new_with_u8_clamped_array_and_sh(Clamped(&mut vec.clone()), width, height).unwrap()
}

#[wasm_bindgen]
pub fn highlight_canvas(canvas_elem: &HtmlCanvasElement) {
    let ctx = canvas_elem
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();

    // 绘制一个红色边框
    ctx.set_stroke_style(&"red".into());
    ctx.rect(0f64, 0f64, 50f64, 50f64);
    ctx.stroke();
    // 获取上述区域的数据
    let image_data = ctx
        .get_image_data(0f64, 0f64, 50f64, 50f64)
        .unwrap()
        .data()
        .to_vec();
    // 将上述获取的 canvas 数据转成 `image` crate 所需的格式
    let photo_image = to_image(50, 50, image_data);
    // 返回高亮后的信息
    let filtered_image = imageops::brighten(&photo_image, 100).to_vec();
    // 将 rust 数据转回 canvas 支持的数据
    let new_canvas_image = to_canvas_image(50, 50, filtered_image);
    ctx.put_image_data(&new_canvas_image, 0f64, 0f64).unwrap();
}
