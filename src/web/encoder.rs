use crate::{Error, TDecoder};
use crate::{SampleRate, TEncoder};
use js_sys::{Function, Object};
use wasm_bindgen::prelude::*;
use web_sys::*;

pub struct Encoder {
    encoder: Option<AudioEncoder>,
    fn_output: Function,
    fn_error: Function,

    sample_rate: SampleRate,
    channels: u32,
    receiver: std::sync::mpsc::Receiver<Pak>,
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
            fn_output: Closure::<dyn FnMut(EncodedAudioChunk)>::new(
                move |output: EncodedAudioChunk| {
                    let mut buffer = vec![0; output.byte_length() as usize];
                    output.copy_to_with_u8_array(&mut buffer);
                    console::log_1(&"Was sent: ".into());
                    sender.send(Pak::Buffer(buffer));
                },
            )
            .into_js_value()
            .into(),
            fn_error: Closure::<dyn FnMut(JsValue)>::new(|error| {
                console::log_2(&"Error: ".into(), &error);
            })
            .into_js_value()
            .into(),
            sample_rate: sample_rate.clone(),
            channels,
            receiver: receiver,
        };

        let encoder = AudioEncoder::new(&AudioEncoderInit::new(&s.fn_error, &s.fn_output)).unwrap();
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
        // let needed = ((i32::from(self.sample_rate.clone()) / 1000) * 20) * self.channels as i32;

        if !input.is_empty() {
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
        }

        if let Ok(pak) = self.receiver.try_recv() {
            match pak {
                Pak::Buffer(mut buffer) => {
                    if output.len() <= buffer.len() {
                        return Err(Error::BufferToSmall);
                    }
                    let len = buffer.len();
                    for (i, byte) in buffer.drain(..).enumerate() {
                        output[i] = byte;
                    }
                    return Ok(len);
                }
                Pak::Error => {
                    console::error_1(&"Opus encoder Error".into());
                }
            }
        }
        Err(Error::CannotEncodeBufferToSmallWaitingForMore)
    }

    fn encode_float(&self, input: &[f32], output: &mut [u8]) -> Result<usize, Error> {
        todo!()
    }
}
