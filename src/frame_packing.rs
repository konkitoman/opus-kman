use bytes_kman::TBytes;

#[derive(Clone, Debug)]
pub enum FramePacking {
    /// 1 frame in the packet
    OneFrame(usize),
    /// 2 frames in the packet, each with equal compressed size
    TowFramesSameSize(usize),
    /// 2 frames in the packet, with different compressed sizes
    TowFrames(usize, usize),
    /// an arbitrary number of frames in the packet  
    Arbitrary,
}

impl FramePacking {
    pub fn new(buffer: &mut Vec<u8>, frame_packing: u8) -> Option<Self> {
        let Some(byte2) = buffer.pop() else{
            log::error!("No seccond byte!");
            return None};

        let mut byte3 = None;
        let n = match byte2 {
            0 => 0,
            1..=251 => byte2 as usize,
            252..=255 => {
                let Some(tmp_byte3) = buffer.pop() else{
                    log::error!("Need one more byte for the frame length");
                    buffer.push(byte2);
                    return None
                };
                byte3 = Some(tmp_byte3);

                (tmp_byte3 as usize * 4) + byte2 as usize
            }
        };
        match frame_packing {
            0 => Some(Self::OneFrame(n - 1)),
            1 => Some(Self::TowFramesSameSize(n - 1)),
            2 => {
                let Some(byte4) = buffer.pop() else{
                    log::error!("No seccond byte!");
                    if let Some(byte3) = byte3{
                        buffer.push(byte3);
                    }
                    buffer.push(byte2);
                return None};

                let mut bytes = 2;

                let size1 = match byte4 {
                    0 => 0,
                    1..=251 => byte4 as usize,
                    252..=255 => {
                        let Some(byte5) = buffer.pop() else{
                            log::error!("Need one more byte for the frame length");
                            buffer.push(byte4);
                            if let Some(byte3) = byte3{
                                buffer.push(byte3);
                            }
                            buffer.push(byte2);
                        return None};
                        bytes += 1;
                        (byte5 as usize * 4) + byte4 as usize
                    }
                };

                let size2 = (n - size1) - bytes;
                Some(Self::TowFrames(size1, size2))
            }
            3 => unimplemented!("FramePaking Arbitrary!"),
            _ => None,
        }
    }
}
