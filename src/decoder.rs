use crate::{Error, SampleRate};

pub trait TDecoder: Sized {
    fn new(sample_rate: SampleRate, channels: u32) -> Result<Self, Error>;
    fn decode(&self, input: &[u8], output: &mut [i16], fec: bool) -> Result<usize, Error>;
    fn decode_float(&self, input: &[u8], output: &mut [f32], fec: bool) -> Result<usize, Error>;
}
