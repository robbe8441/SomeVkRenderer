pub use log::*;
pub use eyre::Result;
pub use thiserror::Error;


pub fn init() {
    std::env::set_var("RUST_LOG", "trace");

    env_logger::builder()
        .format_timestamp(None)
        .write_style(env_logger::WriteStyle::Always)
        .filter_level(log::LevelFilter::max())
        .init();

    debug!("Setting up logger");
}
