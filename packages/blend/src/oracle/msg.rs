use cosmwasm_schema::cw_serde;
#[cfg(not(target_arch = "wasm32"))]
use cosmwasm_schema::QueryResponses;
use cosmwasm_std::{Coin, Uint128};
use mars_owner::OwnerUpdate;

use crate::constants::USD;

#[cw_serde]
pub struct MigrateMsg {}

#[cw_serde]
pub struct OracleMeta {
    pub oracle_denom: String,
    pub decimals: u8,
}

#[cw_serde]
pub struct OracleInstantiateMsg {
    pub owner: String,
    pub oracle_config: Vec<(String, OracleMeta)>,
}

#[cw_serde]
pub struct OracleUpdate {
    pub oracle_config: Option<Vec<(String, OracleMeta)>>,
}

#[cw_serde]
pub enum OracleExecuteMsg {
    UpdateOwner(OwnerUpdate),
    Update(OracleUpdate),
}

#[cw_serde]
#[cfg_attr(not(target_arch = "wasm32"), derive(QueryResponses))]
pub enum OracleQueryMsg {
    #[cfg_attr(not(target_arch = "wasm32"), returns(PriceResponse))]
    Price { coin: Coin },
    #[cfg_attr(not(target_arch = "wasm32"), returns(OracleConfigResponse))]
    Config {},
}

#[cw_serde]
pub struct PriceResponse {
    pub price: Coin,
}

impl Default for PriceResponse {
    fn default() -> Self {
        Self {
            price: Coin {
                denom: USD.to_string(),
                amount: Uint128::zero(),
            },
        }
    }
}

#[cw_serde]
pub struct OracleConfigResponse {
    pub config: Vec<(String, OracleMeta)>,
}
