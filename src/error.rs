use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized: You don't have permission to perform this action")]
    Unauthorized {},

    #[error("Token ID '{0}' is already claimed")]
    Claimed {},

    #[error("Cannot set approval that is already expired")]
    Expired {},

    #[error("Source NFT '{0}' not found")]
    SourceNotFound(String),
}
