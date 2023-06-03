#[derive(Clone, Debug)]
pub enum Bandwidth {
    /// 8-12 kbit/s
    NB,
    /// 16-20 kbit/s
    MB,
    /// 28-40 kbit/s
    WB,
    /// 48-64 kbit/s
    SWB,
    /// 64-128 kbit/s
    FB,
}
