use std::fmt::Display;

#[derive(Debug)]
pub enum MetadataError {
    UnsupportedBigEndian,
    StringParseError(String),
    IntParseError(String),
}

impl Display for MetadataError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MetadataError::UnsupportedBigEndian => write!(f, "Unsupported big endian format"),
            MetadataError::StringParseError(msg) => write!(f, "String parse error: {}", msg),
            MetadataError::IntParseError(msg) => write!(f, "Integer parse error: {}", msg),
        }
    }
}
