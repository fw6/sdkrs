uniffi::setup_scaffolding!();
use log::trace;
pub use sdkrs_core::dynamic_image::DynamicImage;
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

#[uniffi::export]
pub fn get_dynamic_image_url(d: DynamicImage) -> error::Result<String> {
    init_logger();
    Ok(d.format()?)
}
