use opus_kman::*;
use wasm_bindgen::prelude::*;
use web_sys::{console, window, Element};

#[wasm_bindgen(start)]
fn main() {
    let sample_rate = SampleRate::Hz48000;
    let channels = 2 as u32;
    let needed = ((i32::from(sample_rate.clone()) / 1000) * 20) * channels as i32;
    let mut encoder = Encoder::new(sample_rate, channels, Application::VOIP).unwrap();
    console::info_1(&format!("Needed: {needed}").into());
    let input = vec![0; needed as usize];
    let mut output = [0; 255];

    let len = encoder.encode(&input, &mut output).unwrap();

    console::info_1(&format!("Out: {:?}", &output[0..len]).into());

    console::info_1(&format!("Init").into());
}
