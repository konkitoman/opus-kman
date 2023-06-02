use crate::{Error, SampleRate, TDecoder};

pub struct Decoder {
    decoder: *mut audiopus_sys::OpusDecoder,
    sample_rate: SampleRate,
    channels: u32,
}

impl TDecoder for Decoder {
    /// max channels 2 min 1
    fn new(sample_rate: SampleRate, channels: u32) -> Result<Self, Error> {
        let mut error = 0;
        let decoder = unsafe {
            audiopus_sys::opus_decoder_create(
                sample_rate.clone().into(),
                channels as i32,
                &mut error,
            )
        };

        if error == audiopus_sys::OPUS_OK {
            Ok(Self {
                decoder,
                sample_rate,
                channels,
            })
        } else {
            Err(error.into())
        }
    }

    fn decode(&self, input: &[u8], output: &mut [i16], fec: bool) -> Result<usize, Error> {
        let ptr = match input.len() {
            0 => std::ptr::null(),
            _ => input.as_ptr(),
        };
        let res = unsafe {
            audiopus_sys::opus_decode(
                self.decoder,
                ptr,
                input.len() as i32,
                output.as_mut_ptr(),
                (output.len()
                    - (output.len()
                        % (i32::from(self.sample_rate.clone()) as usize / 1000
                            * 20
                            * self.channels as usize))) as i32,
                fec as i32,
            )
        };

        let error: Error = res.into();
        if let Error::Unknown = error {
            Ok(res as usize)
        } else {
            Err(error)
        }
    }

    fn decode_float(&self, input: &[u8], output: &mut [f32], fec: bool) -> Result<usize, Error> {
        let ptr = match input.len() {
            0 => std::ptr::null(),
            _ => input.as_ptr(),
        };
        let res = unsafe {
            audiopus_sys::opus_decode_float(
                self.decoder,
                ptr,
                input.len() as i32,
                output.as_mut_ptr(),
                (output.len()
                    - (output.len()
                        % (i32::from(self.sample_rate.clone()) as usize / 1000
                            * 20
                            * self.channels as usize))) as i32,
                fec as i32,
            )
        };

        let error: Error = res.into();
        if let Error::Unknown = error {
            Ok(res as usize)
        } else {
            Err(error)
        }
    }

    fn get_sample_rate(&self) -> SampleRate {
        self.sample_rate.clone()
    }

    fn set_sample_rate(&mut self, sample_rate: SampleRate) -> Result<(), Error> {
        let res = unsafe {
            audiopus_sys::opus_decoder_init(
                self.decoder,
                sample_rate.clone().into(),
                self.channels as i32,
            )
        };

        if res == audiopus_sys::OPUS_OK {
            self.sample_rate = sample_rate;
            Ok(())
        } else {
            Err(res.into())
        }
    }

    fn get_channels(&self) -> u32 {
        self.channels
    }

    fn set_channels(&mut self, channels: u32) -> Result<(), Error> {
        let res = unsafe {
            audiopus_sys::opus_decoder_init(
                self.decoder,
                self.sample_rate.clone().into(),
                channels as i32,
            )
        };

        if res == audiopus_sys::OPUS_OK {
            self.channels = channels;
            Ok(())
        } else {
            Err(res.into())
        }
    }

    fn reset(&self) {
        unsafe {
            audiopus_sys::opus_decoder_ctl(self.decoder, audiopus_sys::OPUS_RESET_STATE as i32);
        }
    }
}

impl Drop for Decoder {
    fn drop(&mut self) {
        unsafe {
            audiopus_sys::opus_decoder_destroy(self.decoder);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{Decoder, Encoder, TDecoder, TEncoder};

    #[test]
    fn create_decoder() {
        let _ = Decoder::new(crate::SampleRate::Hz48000, 2).unwrap();
    }

    #[test]
    fn encode_decode() {
        let encoder =
            Encoder::new(crate::SampleRate::Hz48000, 1, crate::Application::Audio).unwrap();
        let decoder = Decoder::new(crate::SampleRate::Hz48000, 1).unwrap();

        let input = [0; 48000 / 1000 * 20];
        let mut output = [0; 1024];

        let len = encoder.encode(&input, &mut output).unwrap();
        let out = &output[..len];

        println!("Out: {out:?}");

        let mut output = [0; 4096];

        let len = decoder.decode(out, &mut output, false).unwrap();
        let out = &output[..len];

        println!("Out: {}", out.len());
        println!("Input: {}", input.len());

        assert_eq!(out, input);
    }

    #[test]
    fn encode_decode_float() {
        let encoder =
            Encoder::new(crate::SampleRate::Hz48000, 1, crate::Application::Audio).unwrap();
        let decoder = Decoder::new(crate::SampleRate::Hz48000, 1).unwrap();

        let input = [0.0; 48000 / 1000 * 20];
        let mut output = [0; 4096];

        let len = encoder.encode_float(&input, &mut output).unwrap();
        let out = &output[..len];

        println!("Out: {out:?}");

        let mut output = [0.0; 48000 / 1000 * 20];

        let len = decoder.decode_float(out, &mut output, true).unwrap();
        let out = &output[..len];

        assert_eq!(out, input);
    }
}
