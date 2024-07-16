#[cfg(feature = "builder")]
use derive_builder::Builder;
use lazy_static::lazy_static;
use regex_lite::Regex;
mod options;

pub use options::DynamicImageError;

use self::options::{CropMode, DimensionLimit, ImageFormat, WaterOpacity};

type Result<T, E = DynamicImageError> = std::result::Result<T, E>;

lazy_static! {
    static ref URL_RE: Regex = Regex::new(r"https?://(.+?)((_M)([a-zA-Z0-9]+)(_[1-9])?)?((_[RCZWDYX]_)([0-9]+)_([0-9]+))?((_R)([0-9]+))?((_Q)([0-9]+))?((_M)([a-zA-Z0-9]+)(_[1-9])?)?((_P)([a-zA-Z0-9]+))?(_D)?((\.)(jpg|jpeg|gif|png|Jpg|JPG|webp))(_\.webp)?(_D)?(\?.*)?$").unwrap();

    static ref OLD_URL_RE: Regex = Regex::new(r"https?://(.+?[^/]{30})((_M)([a-zA-Z0-9]+)(_[1-9])?)?((_[RCZWDYX]_)([0-9]+)_([0-9]+))?((_R)([0-9]+))?((_Q)([0-9]+))?((_M)([a-zA-Z0-9]+)(_[1-9])?)?((_P)([a-zA-Z0-9]+))?(_D)?((\.)(jpg|jpeg|gif|png|Jpg|JPG|webp))(_\.webp)?(_D)?(\?.*)?$").unwrap();

    static ref HOST_RE1: Regex = Regex::new(r"https?://((images4|youimg1|(dimg\d{2,})).c-ctrip|(youimg1|ak-d).tripcdn|nephele.(trip|ctrip)).com").unwrap();

    static ref HOST_RE2: Regex = Regex::new(r"https?://(dimg\d{0,}).(uat|fws).qa.nt.(ctripcorp|tripqate).com").unwrap();

    static ref PRESET_LIMITS: Vec<DimensionLimit> = vec![
        DimensionLimit {
            width: 640,
            height: 0,
            quality: Some(90),
            limit: 500,
        },
        DimensionLimit {
            width: 640,
            height: 0,
            quality: Some(80),
            limit: 350,
        },
        DimensionLimit {
            width: 460,
            height: 0,
            quality: Some(80),
            limit: 250,
        },
        DimensionLimit {
            width: 320,
            height: 0,
            quality: Some(80),
            limit: 0,
        },
    ];
}

/// 动态图片配置
#[derive(Clone, PartialEq, Default, Debug)]
#[cfg_attr(
    feature = "builder",
    derive(Builder),
    builder(
        build_fn(validate = "Self::validate", error = "DynamicImageError"),
        setter(into)
    )
)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Record))]
pub struct DynamicImage {
    /// 原始图片链接
    pub raw_url: String,

    /// 需要配置的图片宽度
    #[cfg_attr(feature = "builder", builder(setter(strip_option), default = "None"))]
    pub width: Option<u32>,

    /// 需要配置的图片高度
    #[cfg_attr(feature = "builder", builder(setter(strip_option), default = "None"))]
    pub height: Option<u32>,

    /// 切图模式, 默认为等比缩略
    #[cfg_attr(feature = "builder", builder(default = "Some(CropMode::default())"))]
    pub crop_mode: Option<CropMode>,

    /// 是否重置为 png (会丢失透明通道)
    #[cfg_attr(feature = "builder", builder(default = "None"))]
    pub reset_png: Option<bool>,

    /// 水印透明度指令
    #[cfg_attr(feature = "builder", builder(setter(strip_option), default = "None"))]
    pub water_opacity: Option<WaterOpacity>,

    /// 图片格式, 如果设置了`reset_png`则被忽略
    #[cfg_attr(feature = "builder", builder(setter(strip_option), default = "None"))]
    pub format: Option<ImageFormat>,

    /// 最终生成链接的查询参数
    #[cfg_attr(feature = "builder", builder(setter(strip_option), default = "None"))]
    pub query: Option<String>,

    /// 图片尺寸限制, 不传使用内部预设值
    #[cfg_attr(feature = "builder", builder(setter(strip_option), default = "None"))]
    pub limits: Option<Vec<DimensionLimit>>,

    /// 图片质量
    #[cfg_attr(feature = "builder", builder(setter(strip_option), default = "None"))]
    pub quality: Option<u8>,
}

impl DynamicImage {
    /// 根据配置项, 生成新的图片链接
    pub fn format(&self) -> Result<String> {
        if cfg!(not(feature = "builder")) {
            validate(
                Some(self.raw_url.clone()),
                Some(self.width),
                Some(self.height),
                Some(self.quality),
            )?;
        }

        let caps = if self.raw_url.contains("/fd/") {
            // 老的 /target/fd/ 打头的这种URL，文件名长度30，URL可能匹配_Q629 的文件名。eg：https://images4.c-ctrip.com/target/fd/hotel/g1/M0A/6B/AA/CghzflViwNqAIHWzAAbl8S7rW_Q629.jpg
            OLD_URL_RE
                .captures(&self.raw_url)
                .ok_or(DynamicImageError::InvalidUrl)?
        } else {
            URL_RE
                .captures(&self.raw_url)
                .ok_or(DynamicImageError::InvalidUrl)?
        };

        // 根据宽高的最大值, 获取对应的预设
        let dimension_limit = {
            // 宽高中的最大值
            let max_side = self.width.unwrap_or(0).max(self.height.unwrap_or(0));

            // 预设配置可从外部传入, 否则使用内部预设
            let limits = self.limits.as_ref().unwrap_or(&PRESET_LIMITS);

            // 根据宽高的最大值, 获取对应的预设
            limits
                .iter()
                .find(|preset| preset.limit < max_side)
                .ok_or(DynamicImageError::InvalidDimension)
        }?;

        // 图片原始地址 (不包含任何配置、扩展名)
        let prefix = caps.get(1).map_or("", |s| s.as_str());

        // 水印透明度配置
        let water_opacity = match &self.water_opacity {
            Some(op) => format!("_{}", op),
            None => caps.get(10).map_or("", |s| s.as_str()).to_string(),
        };

        // 图片质量配置, 优先根据外部传入获取, 负责从预设配置中获取, 最后从链接中获取
        let quality = if let Some(q) = self.quality {
            format!("_Q{}", q)
        } else {
            match dimension_limit.quality {
                Some(q) => format!("_Q{}", q),
                None => caps.get(13).map_or("", |s| s.as_str()).to_string(), // 其次从链接中获取
            }
        };

        // 文件扩展名, 如果设置了`reset_png`则强制为 jpg
        let ext = if let Some(true) = self.reset_png {
            ".jpg"
        } else {
            self.format.map_or_else(
                || caps.get(24).map_or(".jpg", |s| s.as_str()),
                |f| {
                    f.extensions_str()
                        .first()
                        .unwrap_or(&caps.get(24).map_or(".jpg", |s| s.as_str()))
                },
            )
        };

        // 链接中初始查询参数
        let query = caps.get(29).map_or("", |s| s.as_str());

        // 额外的查询参数
        let extra_query = self
            .query
            .as_ref()
            .map_or(String::default(), |f| f.to_owned());

        // 最终的查询参数
        let query = if query.is_empty() {
            extra_query
        } else {
            format!(
                "{}{}{}",
                query,
                if query.contains("?") { "&" } else { "?" },
                extra_query,
            )
        };

        Ok(format!(
            "https://{}_{}_{}_{}{}{}{}{}",
            prefix,
            self.crop_mode.map_or(CropMode::default(), |v| v),
            dimension_limit.width,
            dimension_limit.height,
            water_opacity,
            quality,
            ext,
            query
        ))
    }
}

fn validate(
    raw_url: Option<String>,
    width: Option<Option<u32>>,
    height: Option<Option<u32>>,
    quality: Option<Option<u8>>,
) -> Result<()> {
    if let Some(ref raw_url) = raw_url {
        if raw_url.is_empty() {
            return Err(DynamicImageError::EmptyUrl);
        }

        if !(HOST_RE1.is_match(raw_url) || HOST_RE2.is_match(raw_url)) {
            return Err(DynamicImageError::InvalidUrl);
        }
    } else {
        return Err(DynamicImageError::EmptyUrl);
    }

    if width.is_none() && height.is_none() {
        return Err(DynamicImageError::EmptyDimension);
    }

    if let Some(width) = width {
        if let Some(width) = width {
            if width < 1 {
                return Err(DynamicImageError::InvalidWidth);
            }
        } else if let Some(height) = height {
            if height.is_none() {
                return Err(DynamicImageError::InvalidHeight);
            } else if let Some(height) = height {
                if height < 1 {
                    return Err(DynamicImageError::InvalidHeight);
                }
            }
        }
    }

    if let Some(Some(height)) = height {
        if height < 1 {
            return Err(DynamicImageError::InvalidHeight);
        }
    }

    if let Some(Some(quality)) = quality {
        if !(1..=100).contains(&quality) {
            return Err(DynamicImageError::InvalidQuality);
        }
    }

    Ok(())
}

#[cfg(feature = "builder")]
impl DynamicImageBuilder {
    fn validate(&self) -> Result<()> {
        validate(self.raw_url.clone(), self.width, self.height, self.quality)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const OLD_URLS: [&str; 2] = [
        "https://youimg1.c-ctrip.com/target/fd/tg/g3/M07/01/AD/CggYGVXdMjCAYC76AE_wQZoMows816.jpg",
        "http://youimg1.c-ctrip.com/target/fd/tg/g3/M07/01/AD/CggYGVXdMjCAYC76AE_wQZoMows816.jpg",
    ];

    #[test]
    fn test_url_regex() {
        for url in OLD_URLS.iter() {
            assert!(OLD_URL_RE.is_match(url));
        }
    }

    #[test]
    #[cfg(feature = "builder")]
    fn test_builder() {
        // url不能为空
        assert!(DynamicImageBuilder::default().build().is_err());

        // 宽高不能同时为空
        assert!(DynamicImageBuilder::default()
            .raw_url(OLD_URLS.get(0).unwrap().to_string())
            .build()
            .is_err());

        // 宽必须 > 0
        assert!(DynamicImageBuilder::default()
            .raw_url(OLD_URLS.get(0).unwrap().to_string())
            .width(0u32)
            .build()
            .is_err());

        assert!(DynamicImageBuilder::default()
            .raw_url(OLD_URLS.get(0).unwrap().to_string())
            .width(240u32)
            .build()
            .is_ok());
    }

    #[test]
    #[cfg(feature = "builder")]
    fn test_get_url() {
        let output = DynamicImageBuilder::default()
            .raw_url(OLD_URLS.get(0).unwrap().to_string())
            .width(240u32)
            .build()
            .unwrap()
            .format()
            .unwrap();

        println!("{:?}", output);

        assert_eq!(
            output,
            "https://youimg1.c-ctrip.com/target/fd/tg/g3/M07/01/AD/CggYGVXdMjCAYC76AE_wQZoMows816_W_320_0_Q80.jpg");
    }
}
