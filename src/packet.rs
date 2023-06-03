use bytes_kman::TBytes;

use crate::{bandwidth::Bandwidth, frame_packing::FramePacking, mode::Mode};

#[derive(Clone, Debug)]
pub struct Packet {
    mode: Mode,
    bandwidth: Bandwidth,
    stereo: bool,
    frame_packing: FramePacking,
}

impl TBytes for Packet {
    fn size(&self) -> usize {
        1
    }

    fn to_bytes(&self) -> Vec<u8> {
        vec![]
    }

    fn from_bytes(buffer: &mut bytes_kman::TBuffer) -> Option<Self>
    where
        Self: Sized,
    {
        let Some(byte) = buffer.drain(..1).next() else{return None};

        println!("Config: {byte:08b}");

        let config = byte >> 3 & 31;
        let stereo = (byte >> 2 & 1) > 0;
        let Ok(frame_packing) = FramePacking::try_from(byte & 3) else {
            log::error!("Invalid packet frame_packing");
            buffer.insert(0, byte);
            return None;
        };

        let (mode, bandwidth) = match config {
            0..=3 => (Mode::SILK, Bandwidth::NB),
            4..=7 => (Mode::SILK, Bandwidth::MB),
            8..=11 => (Mode::SILK, Bandwidth::WB),
            12..=13 => (Mode::Hybrid, Bandwidth::SWB),
            14..=15 => (Mode::Hybrid, Bandwidth::FB),
            16..=19 => (Mode::CELT, Bandwidth::NB),
            20..=23 => (Mode::CELT, Bandwidth::WB),
            24..=27 => (Mode::CELT, Bandwidth::SWB),
            28..=31 => (Mode::CELT, Bandwidth::FB),
            _ => {
                log::error!("Invalid packet config!");
                buffer.insert(0, byte);
                return None;
            }
        };

        let Some(byte2) = buffer.drain(..1).next() else{
            log::error!("No seccond byte!");
            buffer.insert(0, byte);
            return None};

        let size = match byte2 {
            0 => 0,
            1..=251 => byte2 as usize,
            252..=255 => {
                let Some(byte3) = buffer.drain(0..).next() else{
                    log::error!("Need one more byte for the frame length");
                    buffer.insert(0, byte2);
                    buffer.insert(0, byte);
                    return None
                };

                (byte3 as usize * 4) + byte2 as usize
            }
        };

        if size == 0 {
            panic!("Size 0");
        }

        let frames = match frame_packing {
            FramePacking::OneFrame => {
                let size = size - 1;
                println!("Frame size: {size}");
            }
            FramePacking::TowFramesSameSize => {
                let size = (size - 1) / 2;
                println!("Frame size: {size}");
            }
            FramePacking::TowFrames => {
                let Some(byte3) = buffer.drain(..1).next() else{
                log::error!("No seccond byte!");
                buffer.insert(0, byte);
                return None};

                let mut bytes = 2;

                let size1 = match byte3 {
                    0 => 0,
                    1..=251 => byte3 as usize,
                    252..=255 => {
                        let Some(byte4) = buffer.drain(0..).next() else{
                            log::error!("Need one more byte for the frame length");
                            buffer.insert(0, byte3);
                            buffer.insert(0, byte2);
                            buffer.insert(0, byte);
                        return None};
                        bytes += 1;
                        (byte3 as usize * 4) + byte2 as usize
                    }
                };

                let size2 = (size - size1) - bytes;

                println!("Size1: {size1}");
                println!("Size2: {size2}");

                todo!()
            }
            FramePacking::Arbitrary => {
                let Some(config) = buffer.drain(..1).next() else{
                    buffer.insert(0, byte2);
                    buffer.insert(0, byte);
                    return None
                };
                todo!()
            }
        };

        Some(Self {
            mode,
            bandwidth,
            stereo,
            frame_packing,
        })
    }
}
