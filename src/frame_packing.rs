#[derive(Clone, Debug)]
pub enum FramePacking {
    /// 1 frame in the packet
    OneFrame = 0,
    /// 2 frames in the packet, each with equal compressed size
    TowFramesSameSize = 1,
    /// 2 frames in the packet, with different compressed sizes
    TowFrames = 2,
    /// an arbitrary number of frames in the packet  
    Arbitrary = 3,
}

impl TryFrom<u8> for FramePacking {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Ok(match value {
            0 => Self::OneFrame,
            1 => Self::TowFramesSameSize,
            2 => Self::TowFrames,
            3 => Self::Arbitrary,
            _ => return Err(()),
        })
    }
}
