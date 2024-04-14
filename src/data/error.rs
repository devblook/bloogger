#[derive(Debug)]
pub enum Error {
    Serialization(serde_json::Error),
    Deserialization(serde_json::Error),
    Writing(std::io::Error),
    Reading(std::io::Error),
}
