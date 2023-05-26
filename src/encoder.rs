use crate::{Application, Error, SampleRate};

pub trait TEncoder: Sized {
    fn new(sample_rate: SampleRate, channels: u32, application: Application)
        -> Result<Self, Error>;

    fn encode(&mut self, input: &[i16], output: &mut [u8]) -> Result<usize, Error>;
}
