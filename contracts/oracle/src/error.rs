use blend_kujira::oracle::OracleError;
use cosmwasm_std::StdError;
use mars_owner::OwnerError;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("{0}")]
    Owner(#[from] OwnerError),

    #[error("{0}")]
    Oracle(#[from] OracleError),
}
