use std::sync::Mutex;

use crate::{Application, Error, SampleRate, TEncoder};

pub struct Encoder {
    encoder: *mut audiopus_sys::OpusEncoder,
    sample_rate: SampleRate,
    channels: u32,
    application: Application,
    buffer_i16: Mutex<Vec<i16>>,
    buffer_f32: Mutex<Vec<f32>>,
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
                buffer_i16: Mutex::default(),
                buffer_f32: Mutex::default(),
            })
        } else {
            Err(error.into())
        }
    }

    fn encode(&self, input: &[i16], output: &mut [u8]) -> Result<usize, Error> {
        let needed = ((i32::from(self.sample_rate.clone()) / 1000) * 20) * self.channels as i32;
        let mut buffer = self.buffer_i16.lock().unwrap();
        buffer.append(&mut input.to_vec());

        let len = buffer.len();
        let len = len - (len % needed as usize);

        if len >= needed as usize {
            let buffer = buffer.drain(..len).collect::<Vec<i16>>();
            println!("Len: {}", buffer.len());
            let res = unsafe {
                audiopus_sys::opus_encode(
                    self.encoder,
                    buffer.as_ptr(),
                    (buffer.len() / self.channels as usize) as i32,
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
        } else {
            Err(Error::CannotEncodeBufferToSmallWaitingForMore)
        }
    }

    fn encode_float(&self, input: &[f32], output: &mut [u8]) -> Result<usize, Error> {
        let needed = ((i32::from(self.sample_rate.clone()) / 1000) * 20) * self.channels as i32;
        let mut buffer = self.buffer_f32.lock().unwrap();
        buffer.append(&mut input.to_vec());

        let len = buffer.len();
        let len = len - (len % needed as usize);

        if len >= needed as usize {
            let buffer = buffer.drain(..len).collect::<Vec<f32>>();
            let res = unsafe {
                audiopus_sys::opus_encode_float(
                    self.encoder,
                    buffer.as_ptr(),
                    (buffer.len() / self.channels as usize) as i32,
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
        } else {
            Err(Error::CannotEncodeBufferToSmallWaitingForMore)
        }
    }

    fn get_sample_rate(&self) -> SampleRate {
        self.sample_rate.clone()
    }

    fn set_sample_rate(&mut self, sample_rate: SampleRate) -> Result<(), Error> {
        let res = unsafe {
            audiopus_sys::opus_encoder_init(
                self.encoder,
                sample_rate.clone().into(),
                self.channels as i32,
                self.application.clone().into(),
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
            audiopus_sys::opus_encoder_init(
                self.encoder,
                self.sample_rate.clone().into(),
                channels as i32,
                self.application.clone().into(),
            )
        };

        if res == audiopus_sys::OPUS_OK {
            self.channels = channels;
            Ok(())
        } else {
            Err(res.into())
        }
    }

    fn get_application(&self) -> Application {
        self.application.clone()
    }

    fn set_application(&mut self, application: Application) -> Result<(), Error> {
        let res = unsafe {
            audiopus_sys::opus_encoder_init(
                self.encoder,
                self.sample_rate.clone().into(),
                self.channels as i32,
                application.clone().into(),
            )
        };

        if res == audiopus_sys::OPUS_OK {
            self.application = application;
            Ok(())
        } else {
            Err(res.into())
        }
    }

    fn reset(&self) {
        unsafe {
            audiopus_sys::opus_encoder_ctl(self.encoder, audiopus_sys::OPUS_RESET_STATE);
        }
    }
}

impl Drop for Encoder {
    fn drop(&mut self) {
        unsafe { audiopus_sys::opus_encoder_destroy(self.encoder) }
    }
}

#[cfg(test)]
mod tests {

    use crate::{Encoder, TEncoder};

    #[test]
    fn create_encoder() {
        let _ = Encoder::new(crate::SampleRate::Hz48000, 2, crate::Application::Audio).unwrap();
    }

    #[test]
    fn encode_i16() {
        let encoder =
            Encoder::new(crate::SampleRate::Hz48000, 1, crate::Application::Audio).unwrap();
        let input = [0; 120 * 10];
        let mut output = [0; 4096];
        let len = encoder.encode(&input, &mut output).unwrap();
        let out = &output[..len];

        debug_assert_eq!(out, &[248, 255, 254])
    }
}
