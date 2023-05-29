use std::sync::atomic::AtomicUsize;

use crate::{Error, TDecoder};
use crate::{SampleRate, TEncoder};
use js_sys::{Function, Object};
use wasm_bindgen::prelude::*;
use web_sys::*;

pub struct Encoder {
    encoder: Option<AudioEncoder>,
    fn_output: Option<Function>,
    fn_error: Option<Function>,

    sample_rate: SampleRate,
    channels: u32,
    receiver: std::sync::mpsc::Receiver<Pak>,
    count: std::cell::Cell<usize>,
}

pub enum Pak {
    Buffer(Vec<u8>),
    Error,
}

impl TEncoder for Encoder {
    fn new(
        sample_rate: crate::SampleRate,
        channels: u32,
        application: crate::Application,
    ) -> Result<Self, crate::Error> {
        let (sender, receiver) = std::sync::mpsc::channel();
        let mut s = Self {
            encoder: None,
            fn_output: Some(
                Closure::<dyn FnMut(EncodedAudioChunk)>::new(move |output: EncodedAudioChunk| {
                    let mut buffer = vec![0; output.byte_length() as usize];
                    output.copy_to_with_u8_array(&mut buffer);
                    sender.send(Pak::Buffer(buffer));
                    console::log_1(&"Was sent: ".into());
                })
                .into_js_value()
                .into(),
            ),
            fn_error: Some(
                Closure::<dyn FnMut(JsValue)>::new(|error| {
                    console::log_2(&"Error: ".into(), &error);
                })
                .into_js_value()
                .into(),
            ),
            sample_rate: sample_rate.clone(),
            channels,
            receiver,
            count: std::cell::Cell::new(0),
        };

        let Some(fn_output) = &s.fn_output else {return Err(Error::Unknown)};
        let Some(fn_error) = &s.fn_error else {return Err(Error::Unknown)};
        let encoder = AudioEncoder::new(&AudioEncoderInit::new(fn_error, fn_output)).unwrap();
        encoder.configure(
            &AudioEncoderConfig::new("opus")
                .sample_rate(i32::from(sample_rate) as u32)
                .number_of_channels(channels),
        );
        console::info_1(&format!("Test: {}", encoder.encode_queue_size()).into());
        s.encoder = Some(encoder);

        Ok(s)
    }

    fn encode(&self, input: &[i16], output: &mut [u8]) -> Result<usize, Error> {
        self.count.set(self.count.get() + input.len());
        let needed = ((i32::from(self.sample_rate.clone()) / 1000) * 20) * self.channels as i32;

        let timestamp = js_sys::Date::now();
        let obj = js_sys::Int16Array::from(input).into();
        let data = AudioData::new(&AudioDataInit::new(
            &obj,
            AudioSampleFormat::S16,
            self.channels,
            input.len() as u32 / self.channels,
            48000.0,
            timestamp,
        ))
        .unwrap();

        self.encoder.as_ref().unwrap().encode(&data);
        let count = self.count.get();
        let wait = count >= needed as usize;
        while true {
            if let Ok(pak) = self.receiver.try_recv() {
                self.count.set(self.count.get() - needed as usize);
                match pak {
                    Pak::Buffer(buffer) => {
                        if output.len() <= buffer.len() {
                            return Err(Error::BufferToSmall);
                        }
                        output.copy_from_slice(&buffer);
                        return Ok(buffer.len());
                    }
                    Pak::Error => {
                        console::error_1(&"Opus encoder Error".into());
                        break;
                    }
                }
            }
            if !wait {
                break;
            }
        }
        Ok(0)
    }

    fn encode_float(&self, input: &[f32], output: &mut [u8]) -> Result<usize, Error> {
        todo!()
    }
}
