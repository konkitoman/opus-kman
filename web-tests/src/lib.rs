use opus_kman::*;
use wasm_bindgen::prelude::*;
use web_sys::{console, window, Element};

#[wasm_bindgen(start)]
fn main() {
    let mut encoder = Encoder::new(SampleRate::Hz48000, 1, Application::VOIP).unwrap();
    let input = [0; 240 * 2];
    let mut output = [0; 255];

    let len = encoder.encode(&input, &mut output).unwrap();

    console::info_1(&format!("Out: {:?}", &output[0..len]).into());

    console::info_1(&format!("Init").into());
}
