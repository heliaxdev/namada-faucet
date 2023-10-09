use thiserror::Error;

#[derive(Error, Debug)]
pub enum NamadaError {
    #[error("Invalid denomination/amount: {0}")]
    DenominationInvalid(String),
    #[error("Invalid transaction building: {0}")]
    TxBuildingInvalid(String),
    #[error("Error while broadcasting tx: {0}")]
    TxBroadcastingInvalid(String),
    #[error("Invalid signing data: {0}")]
    SigningDataInvalid(String),
    #[error("Invalid conversion: {0}")]
    ConversionInvalid(String),
    #[error("Can't find secret key")]
    InvalidSecretKey,
}
