use std::fmt::Display;

#[cfg(feature = "builder")]
use derive_builder::UninitializedFieldError;

/// 支持的图片格式, 数据来源于: [格式转换](https://docs.fx.ctripcorp.com/docs/nephele/how-to/image/process/format)
#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Enum))]
pub enum ImageFormat {
    /// An Image in PNG Format
    Png,

    /// An Image in JPEG Format
    Jpeg,

    /// An Image in GIF Format
    Gif,

    /// An Image in WEBP Format
    WebP,
}

impl ImageFormat {
    /// Return a list of applicable extensions for this format.
    pub fn extensions_str(self) -> &'static [&'static str] {
        match self {
            ImageFormat::Png => &["png"],
            ImageFormat::Jpeg => &["jpg", "jpeg"],
            ImageFormat::Gif => &["gif"],
            ImageFormat::WebP => &["webp"],
        }
    }
}

/// 预设的图片尺寸、质量、宽度限制
#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
pub struct DimensionLimit {
    pub width: u32,
    pub height: u32,
    pub limit: u32,
    pub quality: Option<u8>,
}

/// 水印透明度
#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Enum))]
pub enum WaterOpacity {
    R5,
    R10,
    R20,
    R30,
    R40,
    R50,
}

impl Display for WaterOpacity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WaterOpacity::R5 => write!(f, "R5"),
            WaterOpacity::R10 => write!(f, "R10"),
            WaterOpacity::R20 => write!(f, "R20"),
            WaterOpacity::R30 => write!(f, "R30"),
            WaterOpacity::R40 => write!(f, "R40"),
            WaterOpacity::R50 => write!(f, "R50"),
        }
    }
}

/// 切图类型, 参考[文档](https://docs.fx.ctripcorp.com/docs/nephele/how-to/image/process/resize/)
/// - W 等比缩略 (默认)
/// - Z 等比缩略
/// - R 等比缩略
/// - D | C 指定宽高缩放
/// - Y 指定宽高缩放
/// - X 居中裁剪
#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Enum))]
pub enum CropMode {
    W,
    Z,
    R,
    D,
    C,
    Y,
    X,
}

impl Display for CropMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CropMode::W => write!(f, "W"),
            CropMode::Z => write!(f, "Z"),
            CropMode::R => write!(f, "R"),
            CropMode::D => write!(f, "D"),
            CropMode::C => write!(f, "C"),
            CropMode::Y => write!(f, "Y"),
            CropMode::X => write!(f, "X"),
        }
    }
}

impl Default for CropMode {
    fn default() -> Self {
        CropMode::W
    }
}

#[derive(Debug, PartialEq, thiserror::Error)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Error))]
pub enum DynamicImageError {
    #[error("url不正确")]
    InvalidUrl,

    #[error("图片尺寸不正确")]
    InvalidDimension,

    #[error("字段未初始化: {0}")]
    UninitializedField(String),

    #[error("图片地址不能为空")]
    EmptyUrl,

    #[error("宽和高不能同时为空")]
    EmptyDimension,

    #[error("宽度必须 > 1")]
    InvalidWidth,

    #[error("高度必须 > 1")]
    InvalidHeight,

    #[error("图片质量必须在 1-100 之间")]
    InvalidQuality,
}

#[cfg(feature = "builder")]
impl From<UninitializedFieldError> for DynamicImageError {
    fn from(e: UninitializedFieldError) -> Self {
        Self::UninitializedField(e.field_name().into())
    }
}
