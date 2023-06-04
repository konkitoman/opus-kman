use bytes_kman::TBytes;

use crate::{bandwidth::Bandwidth, frame_packing::FramePacking, mode::Mode};

#[derive(Clone, Debug)]
pub struct Packet {
    mode: Mode,
    bandwidth: Bandwidth,
    stereo: bool,
    frame_packing: FramePacking,
    data: Vec<u8>,
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
        buffer.reverse();
        let Some(byte) = buffer.pop() else{return None};

        println!("Config: {byte:08b}");

        let config = byte >> 3u8 & 31;
        let stereo = (byte >> 2u8 & 1) > 0;
        let Some(frame_packing) = FramePacking::new(buffer, byte & 3) else {
            log::error!("Invalid packet frame_packing");
            buffer.push(byte);
            buffer.reverse();
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

        let mut data = Vec::with_capacity(buffer.len());
        while let Some(byte) = buffer.pop() {
            data.push(byte)
        }

        Some(Self {
            mode,
            bandwidth,
            stereo,
            frame_packing,
            data,
        })
    }
}
// // n -1 this is an non inclusive range so n
//                 let ft: u16 = (0..n).map(|i| data[i] as u16).sum();
//                 fn fl(f: &[u8], k: usize) -> u16 {
//                     (0..k).map(|i| f[i] as u16).sum::<u16>()
//                 }
//                 fn fh(f: &[u8], k: usize) -> u16 {
//                     fl(f, k) + f[k] as u16
//                 };
