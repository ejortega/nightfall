use serde::Serialize;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, Box<NightfallError>>;

#[derive(Clone, Debug, Error, Serialize)]
pub enum NightfallError {
    #[error("The requested session doesnt exist")]
    SessionDoesntExist,
    #[error("Chunk requested is not ready yet")]
    ChunkNotDone,
    #[error("Request aborted")]
    Aborted,
    #[error("Session manager died")]
    SessionManagerDied,
    #[error("Failed to patch segment {}", 0)]
    SegmentPatchError(String),
    #[error("Io Error")]
    IoError,
    #[error("Box missing in segment.")]
    MissingSegmentBox,
    #[error("Profile not supported {}", 0)]
    ProfileNotSupported(String),
    #[error("Profile chain exhausted.")]
    ProfileChainExhausted,
    #[error("Parsed a partial segment.")]
    #[serde(skip_serializing)]
    PartialSegment(crate::patch::segment::Segment),
}

impl From<mp4::Error> for NightfallError {
    fn from(e: mp4::Error) -> Self {
        Self::SegmentPatchError(e.to_string())
    }
}

impl From<mp4::Error> for Box<NightfallError> {
    fn from(e: mp4::Error) -> Self {
        Box::new(NightfallError::from(e))
    }
}

impl From<std::io::Error> for NightfallError {
    fn from(_: std::io::Error) -> Self {
        Self::IoError
    }
}

impl From<std::io::Error> for Box<NightfallError> {
    fn from(e: std::io::Error) -> Self {
        Box::new(NightfallError::from(e))
    }
}
