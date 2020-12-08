use anyhow::Result;
use flexi_logger::writers::FileLogWriter;
use log::*;

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
