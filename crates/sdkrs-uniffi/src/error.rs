use std::fmt::{Display, Formatter};

use sdkrs_core::dynamic_image::DynamicImageError;

#[derive(uniffi::Error, Debug)]
#[uniffi(flat_error)]
pub enum SdkrsError {
    DE(DynamicImageError),
}

impl From<DynamicImageError> for SdkrsError {
    fn from(e: DynamicImageError) -> Self {
        Self::DE(e)
    }
}

impl Display for SdkrsError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DE(e) => Display::fmt(e, f),
        }
    }
}

impl std::error::Error for SdkrsError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            SdkrsError::DE(e) => Some(e),
        }
    }
}

pub type Result<T, E = SdkrsError> = std::result::Result<T, E>;
