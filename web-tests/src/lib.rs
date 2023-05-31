use opus_kman::*;
use wasm_bindgen::prelude::*;
use web_sys::{console, window, Element};

#[wasm_bindgen(start)]
fn main() {
    std::panic::set_hook(Box::new(|panic_info| {
        console::log_1(&panic_info.to_string().into());
    }));

    console::info_1(&"Spawning thread!".into());

    let d = js_sys::Promise::new(&mut |d, e| {
        let sample_rate = SampleRate::Hz48000;
        let channels = 2 as u32;
        let needed = ((i32::from(sample_rate.clone()) / 1000) * 20) * channels as i32;
        let mut encoder = Encoder::new(sample_rate, channels, Application::VOIP).unwrap();
        console::info_1(&format!("Needed: {needed}").into());
        let input = vec![0; needed as usize];
        let mut output = [0; 1024];

        let mut res = encoder.encode(&input, &mut output);

        web_sys::window()
            .unwrap()
            .set_timeout_with_callback_and_timeout_and_arguments(
                &Closure::<dyn FnMut()>::new(move || {
                    res = encoder.encode(&[], &mut output);
                    if let Ok(len) = res {
                        console::info_1(&format!("Out: {:?}", &output[0..len]).into());
                    }
                })
                .into_js_value()
                .into(),
                10,
                &js_sys::Array::new(),
            );
    });

    console::info_1(&format!("Init").into());
}
