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
    pub fn decode(&self, mut packet: Packet) {
        match packet.frame_packing {
            crate::frame_packing::FramePacking::OneFrame(size) => {
                packet.data.resize(size, 0);
                let frames = Frame::decode(self, &mut packet).unwrap();
            }
            crate::frame_packing::FramePacking::TowFramesSameSize(size) => {
                packet.data.resize(size * 2, 0);
                let frames = Frame::decode(self, &mut packet).unwrap();
            }
            crate::frame_packing::FramePacking::TowFrames(size1, size2) => {
                packet.data.resize(size1 + size2, 0)
            }
            crate::frame_packing::FramePacking::Arbitrary => unimplemented!(),
        }
    }
}

pub struct Frame {}

impl Frame {
    pub fn decode(decoder: &Decoder, packet: &mut Packet) -> Option<Vec<Self>> {
        match packet.frame_packing {
            crate::frame_packing::FramePacking::OneFrame(size) => {
                let mut data = packet.data.drain(..size).collect::<Vec<u8>>();

                let mut f = [0; u16::MAX as usize];
                for byte in data.iter() {
                    f[*byte as usize] += 1;
                }
                // n -1 this is an non inclusive range so n
                let ft: u16 = (0..size).map(|i| f[i]).sum();
                fn fl(f: &[u16], k: usize) -> u16 {
                    (0..k).map(|i| f[i]).sum::<u16>()
                }
                fn fh(f: &[u16], k: usize) -> u16 {
                    fl(f, k) + f[k]
                };

                let b0 = data.drain(..1).next().unwrap();

                let normalization = b0 & 1 > 0;
                let mut val = 127u32 - (b0 >> 1) as u32;
                let mut rng = 128u32;

                let fs = ft as u32 - (val / ((rng / ft as u32) + 1)).min(ft as u32);
                let val = val - (rng / ft as u32) * (ft - fh(&f, b0 as usize)) as u32;
                let rng = if fl(&f, b0 as usize) > 0 {
                    rng / ft as u32 * (fh(&f, b0 as usize) as u32 - fl(&f, b0 as usize) as u32)
                } else {
                    (rng - (rng / ft as u32)) * ft as u32 - fh(&f, b0 as usize) as u32
                };

                if normalization {
                    if rng < 8388608 {
                        let rng = rng << 8;
                        let sym = data.drain(..1).next().unwrap() as u32;
                        println!("Sym: {sym}");
                        unimplemented!("Read: 4.1.2.1");
                    }
                }

                println!("Val: {val}, rng: {rng}, normalization: {normalization}, fs: {fs}");
            }
            crate::frame_packing::FramePacking::TowFramesSameSize(_) => todo!(),
            crate::frame_packing::FramePacking::TowFrames(_, _) => todo!(),
            crate::frame_packing::FramePacking::Arbitrary => todo!(),
        }

        // decode symbols

        todo!()
    }
}
