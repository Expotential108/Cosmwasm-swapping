use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},
    // Add any other custom errors you like here.
    #[error("InvalidParam")]
    InvalidParam {},
    // for internal usage only !
    #[error("NotAnError")]
    NotAnError {},
}
