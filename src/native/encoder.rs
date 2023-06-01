use std::sync::{Arc, Mutex};

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
}

impl Drop for Encoder {
    fn drop(&mut self) {
        unsafe { audiopus_sys::opus_encoder_destroy(self.encoder) }
    }
}

#[cfg(test)]
mod tests {
    use crate::{Decoder, Encoder, TDecoder, TEncoder};

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

    // All the time faild i don't know why the decoding is strange!
    #[test]
    fn encode_decode() {
        let encoder =
            Encoder::new(crate::SampleRate::Hz48000, 1, crate::Application::Audio).unwrap();
        let decoder = Decoder::new(crate::SampleRate::Hz48000, 1).unwrap();

        let input = [
            0.0,
            0.009853362,
            0.01968765,
            0.02950054,
            0.03928942,
            0.049051683,
            0.058784805,
            0.068486266,
            0.078153506,
            0.08778406,
            0.09737537,
            0.10692494,
            0.11643028,
            0.12588987,
            0.13530126,
            0.14466298,
            0.15397258,
            0.16322759,
            0.17242566,
            0.18156432,
            0.19064115,
            0.19965379,
            0.20860077,
            0.21747978,
            0.22628842,
            0.23502441,
            0.24368543,
            0.25226918,
            0.2607733,
            0.26919556,
            0.27753365,
            0.28578535,
            0.29394832,
            0.30202028,
            0.310998,
            0.31987923,
            0.32866174,
            0.33734334,
            0.34592175,
            0.3543948,
            0.36276022,
            0.37101582,
            0.37915942,
            0.38718882,
            0.3951019,
            0.40289646,
            0.41057032,
            0.41812128,
            0.42554718,
            0.43284586,
            0.4400151,
            0.44705275,
            0.4539566,
            0.46072453,
            0.46735436,
            0.4738439,
            0.48019198,
            0.4863964,
            0.492455,
            0.49836564,
        ];

        let mut inpu = [0.0; 960];
        for (i, input) in input.into_iter().enumerate() {
            inpu[i] = input;
        }
        let input = inpu;

        let mut output = [0; 4096];
        let len = encoder.encode_float(&input, &mut output).unwrap();

        let out = &output[..len];
        let mut output = [0.0; 960];

        let len = decoder.decode_float(out, &mut output, false).unwrap();

        let out = &output[..len];

        assert_eq!(input, out);
    }
}
