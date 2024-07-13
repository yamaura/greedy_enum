pub use greedy_enum_derive::FromStr;

/// Custom error type for parsing strings into enum variants.
#[derive(Debug, thiserror::Error)]
#[error("No valid variant found for string: '{span}'")]
pub struct ParseError {
    pub span: String,
}
