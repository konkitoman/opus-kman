use crate::{Error, TDecoder};
use wasm_bindgen::prelude::*;
use web_sys::*;

pub struct Decoder {
    decoder: Option<AudioDecoder>,
    fn_output: Option<Closure<dyn FnMut(JsValue)>>,
    fn_error: Option<Closure<dyn FnMut(JsValue)>>,
}

impl TDecoder for Decoder {
    fn new(sample_rate: crate::SampleRate, channels: u32) -> Result<Self, Error> {
        let mut s = Self {
            decoder: None,
            fn_output: Some(Closure::new(|output| {
                console::log_2(&"Output: ".into(), &output);
            })),
            fn_error: Some(Closure::new(|error| {
                console::log_2(&"Error: ".into(), &error);
            })),
        };

        let Some(fn_output) = &s.fn_output else {return Err(Error::Unknown)};
        let Some(fn_error) = &s.fn_error else {return Err(Error::Unknown)};
        let decoder = AudioDecoder::new(&AudioDecoderInit::new(
            fn_output.as_ref().unchecked_ref(),
            fn_error.as_ref().unchecked_ref(),
        ))
        .unwrap();
        decoder.configure(&AudioDecoderConfig::new(
            "opus",
            channels,
            i32::from(sample_rate.clone()) as u32,
        ));
        s.decoder = Some(decoder);

        Ok(s)
    }

    fn decode(&self, input: &[u8], output: &mut [i16], fec: bool) -> Result<usize, Error> {
        todo!()
    }

    fn decode_float(&self, input: &[u8], output: &mut [f32], fec: bool) -> Result<usize, Error> {
        todo!()
    }
}
