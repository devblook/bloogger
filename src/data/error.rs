// Allow unused fields, which are used for Debug output
#[allow(dead_code)]
#[derive(Debug)]
pub enum Error {
    Serialization(serde_json::Error),
    Deserialization(serde_json::Error),
    Writing(std::io::Error),
    Reading(std::io::Error),
}
