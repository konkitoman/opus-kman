mod decoder;
mod encoder;

#[cfg(not(target_arch = "wasm32"))]
mod native;

#[cfg(target_arch = "wasm32")]
mod web;

pub use decoder::TDecoder;
pub use encoder::TEncoder;

#[cfg(not(target_arch = "wasm32"))]
pub use native::{Decoder, Encoder};

#[cfg(target_arch = "wasm32")]
pub use web::{Decoder, Encoder};

#[derive(Clone, Default)]
pub enum SampleRate {
    Hz8000,
    Hz12000,
    Hz16000,
    Hz24000,
    #[default]
    Hz48000,
}

impl From<SampleRate> for i32 {
    fn from(value: SampleRate) -> Self {
        match value {
            SampleRate::Hz8000 => 8000,
            SampleRate::Hz12000 => 12000,
            SampleRate::Hz16000 => 16000,
            SampleRate::Hz24000 => 24000,
            SampleRate::Hz48000 => 48000,
        }
    }
}

#[derive(Clone, Debug)]
pub enum Error {
    ///One or more invalid/out of range arguments.
    BadArg,
    ///Not enough bytes allocated in the buffer.
    BufferToSmall,
    ///An internal error was detected.
    InternalError,
    ///The compressed data passed is corrupted.
    InvalidPacket,
    ///Invalid/unsupported request number.
    UnImplemented,
    ///An encoder or decoder structure is invalid or already freed.
    InvalidState,
    ///Memory allocation has failed.
    AllocFail,

    Unknown,

    CannotEncodeBufferToSmallWaitingForMore,
    /// Whould not be recalled in a loop because browser thread will be blocked and nothing will happend
    NeedAReCall,
}

#[cfg(target_arch = "x86_64")]
impl From<i32> for Error {
    fn from(value: i32) -> Self {
        use audiopus_sys::{
            OPUS_ALLOC_FAIL, OPUS_BAD_ARG, OPUS_BUFFER_TOO_SMALL, OPUS_INTERNAL_ERROR,
            OPUS_INVALID_PACKET, OPUS_INVALID_STATE, OPUS_UNIMPLEMENTED,
        };
        match value {
            OPUS_BAD_ARG => Self::BadArg,
            OPUS_ALLOC_FAIL => Self::AllocFail,
            OPUS_BUFFER_TOO_SMALL => Self::BufferToSmall,
            OPUS_INTERNAL_ERROR => Self::InternalError,
            OPUS_INVALID_PACKET => Self::InvalidPacket,
            OPUS_INVALID_STATE => Self::InvalidState,
            OPUS_UNIMPLEMENTED => Self::UnImplemented,
            _ => Self::Unknown,
        }
    }
}

#[derive(Default, Clone, Debug)]
pub enum Application {
    ///Best for broadcast/high-fidelity application where the decoded audio should be as close as possible to the input.
    Audio,
    ///Only use when lowest-achievable latency is what matters most.
    ///Voice-optimized modes cannot be used.
    RestrictedLowdelay,
    ///Best for most VoIP/videoconference applications where listening quality and intelligibility matter most.
    #[default]
    VOIP,
}

#[cfg(target_arch = "x86_64")]
impl From<Application> for i32 {
    fn from(value: Application) -> Self {
        use audiopus_sys::{
            OPUS_APPLICATION_AUDIO, OPUS_APPLICATION_RESTRICTED_LOWDELAY, OPUS_APPLICATION_VOIP,
        };
        match value {
            Application::Audio => OPUS_APPLICATION_AUDIO,
            Application::RestrictedLowdelay => OPUS_APPLICATION_RESTRICTED_LOWDELAY,
            Application::VOIP => OPUS_APPLICATION_VOIP,
        }
    }
}
