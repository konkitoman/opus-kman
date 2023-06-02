use crate::{Application, Error, SampleRate};

pub trait TDecoder: Sized {
    fn new(sample_rate: SampleRate, channels: u32) -> Result<Self, Error>;
    fn decode(&self, input: &[u8], output: &mut [i16], fec: bool) -> Result<usize, Error>;
    fn decode_float(&self, input: &[u8], output: &mut [f32], fec: bool) -> Result<usize, Error>;

    fn get_sample_rate(&self) -> SampleRate;
    fn set_sample_rate(&mut self, sample_rate: SampleRate) -> Result<(), Error>;

    fn get_channels(&self) -> u32;
    fn set_channels(&mut self, channels: u32) -> Result<(), Error>;

    fn reset(&self);
}
