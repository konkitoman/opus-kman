use crate::{packet::Packet, Error, SampleRate};

pub struct Decoder {
    sample_rate: u32,
    channels: u8,
}

impl Decoder {
    pub fn new(sample_rate: u32, channels: u8) -> Self {
        Self {
            sample_rate,
            channels,
        }
    }
    pub fn decode(&self, packet: Packet) {}
}
