#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

fn main() {
    // 1. Init tracing subscriber — reads RUST_LOG env var.
    //    Default: debug for our crate, info for everything else.
    //    Examples:
    //      RUST_LOG=debug          → everything at debug+
    //      RUST_LOG=messenger_desktop=trace → just our crate at trace
    //      RUST_LOG=warn           → only warnings/errors
    use tracing_subscriber::{fmt, EnvFilter};
    fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| EnvFilter::new("messenger_desktop=debug,info")),
        )
        .with_target(true)
        .with_thread_ids(true)
        .with_line_number(true)
        .pretty()
        .init();

    // 2. Bridge `log` crate macros (log::info!, log::warn!, etc.) into tracing.
    //    This makes all deps that use `log` appear in our tracing output.
    tracing_log::LogTracer::init().ok();

    tracing::info!("Messenger Desktop starting up");

    messenger_desktop::run();
}
