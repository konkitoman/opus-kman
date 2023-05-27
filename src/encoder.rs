use crate::{Application, Error, SampleRate};

pub trait TEncoder: Sized {
    fn new(sample_rate: SampleRate, channels: u32, application: Application)
        -> Result<Self, Error>;

    fn encode(&self, input: &[i16], output: &mut [u8]) -> Result<usize, Error>;
    fn encode_float(&self, input: &[f32], output: &mut [u8]) -> Result<usize, Error>;
}
