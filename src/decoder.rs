use crate::{Error, SampleRate};

pub trait TDecoder: Sized {
    fn new(sample_rate: SampleRate, channels: u32) -> Result<Self, Error>;
}
