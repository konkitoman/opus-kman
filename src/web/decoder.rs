use crate::{Error, TDecoder};
use js_sys::Function;
use wasm_bindgen::prelude::*;
use web_sys::*;

pub struct Decoder {
    decoder: Option<AudioDecoder>,
    fn_output: Function,
    fn_error: Function,
    receiver: std::sync::mpsc::Receiver<Pak>,
}

enum Pak {
    BufferI16(Vec<i16>),
    BufferF32(Vec<f32>),
    Error,
}

impl TDecoder for Decoder {
    fn new(sample_rate: crate::SampleRate, channels: u32) -> Result<Self, Error> {
        let (sender, receiver) = std::sync::mpsc::channel();
        let sender_err = sender.clone();
        let mut s = Self {
            decoder: None,
            fn_output: Closure::<dyn FnMut(AudioData)>::new(move |output: AudioData| {
                console::log_2(&"Output: ".into(), &output);
                if let Some(format) = output.format() {
                    match format {
                        AudioSampleFormat::S16 => {
                            let mut buffer = js_sys::Int16Array::new_with_length(4096);
                            output.copy_to_with_buffer_source(
                                &buffer,
                                &AudioDataCopyToOptions::new(0),
                            );
                            sender.send(Pak::BufferI16(buffer.to_vec()));
                        }
                        AudioSampleFormat::F32 => {
                            let mut buffer = js_sys::Float32Array::new_with_length(4096);
                            output.copy_to_with_buffer_source(
                                &buffer,
                                &AudioDataCopyToOptions::new(0),
                            );
                            sender.send(Pak::BufferF32(buffer.to_vec()));
                        }
                        _ => {}
                    }
                }
            })
            .into_js_value()
            .into(),
            fn_error: Closure::<dyn FnMut(JsValue)>::new(move |error| {
                console::log_2(&"Error: ".into(), &error);
                sender_err.send(Pak::Error);
            })
            .into_js_value()
            .into(),
            receiver,
        };

        let decoder = AudioDecoder::new(&AudioDecoderInit::new(&s.fn_error, &s.fn_output)).unwrap();
        decoder.configure(&AudioDecoderConfig::new(
            "opus",
            channels,
            i32::from(sample_rate.clone()) as u32,
        ));
        s.decoder = Some(decoder);

        Ok(s)
    }

    fn decode(&self, input: &[u8], output: &mut [i16], fec: bool) -> Result<usize, Error> {
        if !input.is_empty() {
            self.decoder.as_ref().unwrap().decode(
                &EncodedAudioChunk::new(&EncodedAudioChunkInit::new(
                    &js_sys::Uint8Array::from(input).into(),
                    js_sys::Date::now(),
                    EncodedAudioChunkType::Key,
                ))
                .unwrap(),
            );
        }

        if let Ok(pak) = self.receiver.try_recv() {
            match pak {
                Pak::BufferI16(mut buffer) => {
                    if buffer.len() >= output.len() {
                        return Err(Error::BufferToSmall);
                    }
                    for (i, byte) in buffer.drain(..).enumerate() {
                        output[i] = byte;
                    }
                }
                Pak::BufferF32(mut buffer) => {
                    if buffer.len() >= output.len() {
                        return Err(Error::BufferToSmall);
                    }
                    let scale = std::i16::MAX as f32;
                    for (i, byte) in buffer.drain(..).enumerate() {
                        output[i] = (byte * scale).max(scale).min(std::i16::MIN as f32) as i16;
                    }
                }
                Pak::Error => return Err(Error::Unknown),
            }
        }
        Err(Error::CannotEncodeBufferToSmallWaitingForMore)
    }

    fn decode_float(&self, input: &[u8], output: &mut [f32], fec: bool) -> Result<usize, Error> {
        if !input.is_empty() {
            self.decoder.as_ref().unwrap().decode(
                &EncodedAudioChunk::new(&EncodedAudioChunkInit::new(
                    &js_sys::Uint8Array::from(input).into(),
                    js_sys::Date::now(),
                    EncodedAudioChunkType::Key,
                ))
                .unwrap(),
            );
        }

        if let Ok(pak) = self.receiver.try_recv() {
            match pak {
                Pak::BufferI16(mut buffer) => {
                    if buffer.len() >= output.len() {
                        return Err(Error::BufferToSmall);
                    }
                    for (i, byte) in buffer.drain(..).enumerate() {
                        let scale = 1.0 / std::u16::MAX as f32;
                        output[i] = byte as f32 * scale;
                    }
                }
                Pak::BufferF32(mut buffer) => {
                    if buffer.len() >= output.len() {
                        return Err(Error::BufferToSmall);
                    }
                    let scale = std::i16::MAX as f32;
                    for (i, byte) in buffer.drain(..).enumerate() {
                        output[i] = byte;
                    }
                }
                Pak::Error => return Err(Error::Unknown),
            }
        }
        Err(Error::CannotEncodeBufferToSmallWaitingForMore)
    }
}
