use thiserror::Error;



#[derive(Error, Debug, PartialEq)]
pub enum OxydeError {
    #[error("Deserialization Error - {err}")]
    DeserializationError{err: String},

    #[error("Serialization Error - {err}")]
    SerializationError{err: String}
}