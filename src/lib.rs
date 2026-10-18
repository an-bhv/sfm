use wasm_bindgen::prelude::*;
use web_sys::console;

#[wasm_bindgen]
extern "C" {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}

#[wasm_bindgen]
pub struct SfmPipeline {
    images: Vec<Vec<u8>>, // Stores all images in Rust RAM
}

#[wasm_bindgen]
impl SfmPipeline {
    #[wasm_bindgen(constructor)]
    pub fn new() -> SfmPipeline {
        SfmPipeline { images: Vec::new() }
    }

    pub fn add_image(&mut self, bytes: &[u8]) {
        self.images.push(bytes.to_vec()); // Copy bytes into Rust memory
        console::log_1(&format!("Total images loaded: {}", self.images.len()).into());
    }

    pub fn run_pipeline(&self) {
        console::log_1(&format!("Starting SfM on {} images...", self.images.len()).into());
        // Feature extraction, matching, etc. goes here
    }
}

#[wasm_bindgen]
pub fn load_image_bytes(bytes: &[u8]){
    console::log_1(&format!("Rust received {} bytes successfully!", bytes.len()).into());
}