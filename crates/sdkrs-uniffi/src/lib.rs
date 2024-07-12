uniffi::setup_scaffolding!();
use log::trace;
pub use sdkrs_core::dynamic_image::DynamicImageBuilder;

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
