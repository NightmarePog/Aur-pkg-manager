use tracing_subscriber::fmt;

pub fn init_logging() {
    fmt()
        .without_time()
        .with_target(false)
        .with_level(true)
        .init();
}
