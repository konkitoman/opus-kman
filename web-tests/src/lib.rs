use opus_kman::*;
use wasm_bindgen::prelude::*;
use web_sys::console;

#[wasm_bindgen(start)]
fn main() {
    std::panic::set_hook(Box::new(|panic_info| {
        console::log_1(&panic_info.to_string().into());
    }));

    console::info_1(&"Spawning thread!".into());

    let sample_rate = SampleRate::Hz48000;
    let channels = 2u32;
    let needed = ((i32::from(sample_rate.clone()) / 1000) * 20) * channels as i32;
    let encoder = Encoder::new(sample_rate, channels, Application::VOIP).unwrap();
    console::info_1(&format!("Needed: {needed}").into());
    let input = vec![0; needed as usize];
    let mut output = [0; 1024];

    let _ = encoder.encode(&input, &mut output);

    let _ = web_sys::window()
        .unwrap()
        .set_timeout_with_callback_and_timeout_and_arguments(
            &Closure::<dyn FnMut()>::new(move || {
                let res = encoder.encode(&[], &mut output);
                if let Ok(len) = res {
                    let decoder = Decoder::new(SampleRate::Hz48000, 2).unwrap();
                    console::info_1(&format!("Out: {:?}", &output[0..len]).into());
                    let out = &output[..len];
                    let mut output = [0; 4096];
                    let res = decoder.decode(out, &mut output, false);
                    console::log_1(&format!("Res: {res:?}").into());
                }
            })
            .into_js_value()
            .into(),
            10,
            &js_sys::Array::new(),
        );

    console::info_1(&"Init".to_string().into());
}
