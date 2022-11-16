use std::string::FromUtf8Error;
use thiserror::Error;

use cosmwasm_std::StdError;
use cw_utils::PaymentError;

/// Never is a placeholder to ensure we don't return any errors
#[derive(Error, Debug)]
pub enum Never {}

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("{0}")]
    Payment(#[from] PaymentError),

    #[error("Didn't send any funds")]
    NoFunds {},

    #[error("Only admin can do this")]
    Unauthorized,

    #[error("Recipient format is invalid")]
    InvalidFormat{recipient: String},

    #[error("Contract balance is empty")]
    EmptyBalance
}

impl From<FromUtf8Error> for ContractError {
    fn from(_: FromUtf8Error) -> Self {
        ContractError::Std(StdError::invalid_utf8("parsing denom key"))
    }
}

// impl From<TryFromIntError> for ContractError {
//     fn from(_: TryFromIntError) -> Self {
//         ContractError::AmountOverflow {}
//     }
// }
