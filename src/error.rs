use rusoto_core::RusotoError;
use rusoto_dynamodb::{
    BatchWriteItemError, DeleteItemError, GetItemError, PutItemError, QueryError,
};
#[derive(Debug)]
pub enum DshmError {
    Unknown,
    DynamoError(String),
    BrokenContent(serde_json::error::Error),
    NoItem,
}

impl std::fmt::Display for DshmError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for DshmError {}

impl From<RusotoError<PutItemError>> for DshmError {
    fn from(e: RusotoError<PutItemError>) -> Self {
        Self::DynamoError(e.to_string())
    }
}

impl From<RusotoError<GetItemError>> for DshmError {
    fn from(e: RusotoError<GetItemError>) -> Self {
        Self::DynamoError(e.to_string())
    }
}

impl From<RusotoError<DeleteItemError>> for DshmError {
    fn from(e: RusotoError<DeleteItemError>) -> Self {
        Self::DynamoError(e.to_string())
    }
}

impl From<RusotoError<BatchWriteItemError>> for DshmError {
    fn from(e: RusotoError<BatchWriteItemError>) -> Self {
        Self::DynamoError(e.to_string())
    }
}

impl From<RusotoError<QueryError>> for DshmError {
    fn from(e: RusotoError<QueryError>) -> Self {
        Self::DynamoError(e.to_string())
    }
}

impl From<serde_json::error::Error> for DshmError {
    fn from(e: serde_json::error::Error) -> Self {
        Self::BrokenContent(e)
    }
}
