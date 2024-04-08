#[derive(Debug)]
pub enum Error {
    FailedSerialization(serde_json::Error),
    FailedDeserialization(serde_json::Error),
    FailedWriting(std::io::Error),
    FailedReading(std::io::Error),
}
