use crate::{Application, Error, SampleRate};

pub trait TEncoder: Sized {
    fn new(sample_rate: SampleRate, channels: u32, application: Application)
        -> Result<Self, Error>;

    fn encode(&self, input: &[i16], output: &mut [u8]) -> Result<usize, Error>;
    fn encode_float(&self, input: &[f32], output: &mut [u8]) -> Result<usize, Error>;

    fn get_sample_rate(&self) -> SampleRate;
    fn set_sample_rate(&mut self, sample_rate: SampleRate) -> Result<(), Error>;

    fn get_channels(&self) -> u32;
    fn set_channels(&mut self, channels: u32) -> Result<(), Error>;

    fn get_application(&self) -> Application;
    fn set_application(&mut self, application: Application) -> Result<(), Error>;

    fn reset(&self);
}
