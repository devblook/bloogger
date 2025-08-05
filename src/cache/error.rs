// Allow unused fields, which are used for Debug output
#[allow(dead_code)]
#[derive(Debug)]
pub enum Error {
    SaveError(SaveError),
    LoadError(LoadError),
}

impl From<SaveError> for Error {
    fn from(value: SaveError) -> Self {
        Self::SaveError(value)
    }
}

impl From<LoadError> for Error {
    fn from(value: LoadError) -> Self {
        Self::LoadError(value)
    }
}

// Allow unused fields, which are used for Debug output
#[allow(dead_code)]
#[derive(Debug)]
pub enum SaveError {
    FailedSerialization(serde_json::Error),
    FailedWriting(std::io::Error),
}

impl From<serde_json::Error> for SaveError {
    fn from(value: serde_json::Error) -> Self {
        Self::FailedSerialization(value)
    }
}

impl From<std::io::Error> for SaveError {
    fn from(value: std::io::Error) -> Self {
        Self::FailedWriting(value)
    }
}

// Allow unused fields, which are used for Debug output
#[allow(dead_code)]
#[derive(Debug)]
pub enum LoadError {
    ConfigNotFound(u64),
    FailedDeserialization(serde_json::Error),
    FailedReading(std::io::Error),
}

impl From<u64> for LoadError {
    fn from(id: u64) -> Self {
        Self::ConfigNotFound(id)
    }
}

impl From<serde_json::Error> for LoadError {
    fn from(value: serde_json::Error) -> Self {
        Self::FailedDeserialization(value)
    }
}

impl From<std::io::Error> for LoadError {
    fn from(value: std::io::Error) -> Self {
        Self::FailedReading(value)
    }
}
