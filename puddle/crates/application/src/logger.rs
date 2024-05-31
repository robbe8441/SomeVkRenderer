pub use log::*;

pub fn init() {
    std::env::set_var("RUST_LOG", "trace");

    env_logger::builder()
        .format_timestamp(None)
        .write_style(env_logger::WriteStyle::Always)
        .filter_level(log::LevelFilter::Trace)
        .init();
}
