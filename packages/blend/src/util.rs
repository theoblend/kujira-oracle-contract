use cosmwasm_std::StdError;
use cw2::ContractVersion;

pub fn vec_to_str<T: ToString>(v: &[T]) -> String {
    v.iter()
        .map(|t| t.to_string())
        .collect::<Vec<String>>()
        .join(", ")
}

pub fn non_migratable_err(contract: &str, version: ContractVersion) -> StdError {
    StdError::generic_err(format!(
        "Contract {}({:?}) is currently not migratable",
        contract, version
    ))
}
