uniffi::setup_scaffolding!();
use log::trace;
pub use sdkrs_core::nephele_image::NepheleImage;
pub mod error;

pub fn init_logger() {
    #[cfg(not(any(target_os = "android", target_os = "ios")))]
    let _ = env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info"))
        .try_init();

    #[cfg(target_os = "ios")]
    let _ = oslog::OsLogger::new("com.ctrip.sdkrs")
        .level_filter(log::LevelFilter::Info)
        .init();

    #[cfg(target_os = "android")]
    android_logger::init_once(
        android_logger::Config::default().with_max_level(uniffi::deps::log::LevelFilter::Info),
    );

    trace!("\n\n INIT LOGGER ON RUST");
}

/// 处理通过Nephele图片服务上传的图片地址,
/// 通过在URL上拼写命令动态进行缩略、打水印和格式变换等操作。
/// 因Nephele各环境之间是相互隔离, 故提供统一方法为图片添加命令。
#[uniffi::export]
pub fn process_nephele_image_url(d: NepheleImage) -> error::Result<String> {
    init_logger();
    Ok(d.format()?)
}
