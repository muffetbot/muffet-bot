use anyhow::Result;
use flexi_logger::{writers::FileLogWriter, Age, Cleanup, Criterion, Naming};

use std::path::PathBuf;
use once_cell::sync::Lazy;
static LOG_PATH: Lazy<PathBuf> = Lazy::new(|| PathBuf::from("~/"));

pub fn crate_logger() -> Result<FileLogWriter> {
    let _logger = FileLogWriter::builder()
        .directory(&(*LOG_PATH))
        .discriminant("MBOT")
        .rotate(
            Criterion::Age(Age::Day),
            Naming::Timestamps,
            Cleanup::KeepLogFiles(30),
        )
        .print_message()
        .try_build()?;
    Ok(_logger)
}
