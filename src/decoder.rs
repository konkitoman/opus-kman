use crate::{Error, SampleRate};

pub struct Decoder {
    sample_rate: u32,
    channels: u8,
}
