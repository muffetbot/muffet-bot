use anyhow::Result;
use flexi_logger::{writers::FileLogWriter, Age, Cleanup, Criterion, Naming};

pub fn crate_logger<P: Into<std::path::PathBuf>>(path: P) -> Result<FileLogWriter> {
    let logger = FileLogWriter::builder()
        .directory(path)
        .rotate(
            Criterion::Age(Age::Day),
            Naming::Timestamps,
            Cleanup::KeepLogFiles(30),
        )
        .print_message()
        .try_build()?;
    Ok(logger)
}
