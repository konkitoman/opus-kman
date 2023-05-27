use crate::{Application, Error, SampleRate, TEncoder};

pub struct Encoder {
    encoder: *mut audiopus_sys::OpusEncoder,
    sample_rate: SampleRate,
    channels: u32,
    application: Application,
}

impl TEncoder for Encoder {
    /// max channels 2 min 1
    fn new(
        sample_rate: SampleRate,
        channels: u32,
        application: Application,
    ) -> Result<Self, Error> {
        let mut error = 0;
        let encoder = unsafe {
            audiopus_sys::opus_encoder_create(
                sample_rate.clone().into(),
                channels as i32,
                application.clone().into(),
                &mut error,
            )
        };
        if error == audiopus_sys::OPUS_OK {
            Ok(Self {
                encoder,
                sample_rate,
                channels,
                application,
            })
        } else {
            Err(error.into())
        }
    }

    /// input len should be min 120 if has only one channel if has 2 channels min is 240
    fn encode(&self, input: &[i16], output: &mut [u8]) -> Result<usize, Error> {
        let res = unsafe {
            audiopus_sys::opus_encode(
                self.encoder,
                input.as_ptr(),
                (input.len() / self.channels as usize) as i32,
                output.as_mut_ptr(),
                output.len() as i32,
            )
        };

        let error: Error = res.into();
        if let Error::Unknown = error {
            Ok(res as usize)
        } else {
            Err(error)
        }
    }

    fn encode_float(&self, input: &[f32], output: &mut [u8]) -> Result<usize, Error> {
        let res = unsafe {
            audiopus_sys::opus_encode_float(
                self.encoder,
                input.as_ptr(),
                (input.len() / self.channels as usize) as i32,
                output.as_mut_ptr(),
                output.len() as i32,
            )
        };

        let error: Error = res.into();
        if let Error::Unknown = error {
            Ok(res as usize)
        } else {
            Err(error)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{Encoder, TEncoder};

    #[test]
    fn create_encoder() {
        let encoder =
            Encoder::new(crate::SampleRate::Hz48000, 2, crate::Application::Audio).unwrap();
    }

    #[test]
    fn encode_i16() {
        let encoder =
            Encoder::new(crate::SampleRate::Hz48000, 1, crate::Application::Audio).unwrap();
        let input = [0; 120];
        let mut output = [0; 1024];
        let len = encoder.encode(&input, &mut output).unwrap();
    }
}
