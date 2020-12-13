use tracing_appender::{non_blocking, rolling::daily};

pub fn start_trace_subscriber<P: AsRef<std::path::Path>>(path: P) {
    let file_appender = daily(path, "MBOT");
    let (non_blocking, _guard) = non_blocking(file_appender);
    tracing_subscriber::fmt().with_writer(non_blocking).init();
}
