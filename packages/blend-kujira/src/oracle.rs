use blend::oracle::msg::OracleMeta;
use cosmwasm_schema::cw_serde;
use cosmwasm_std::{QuerierWrapper, StdError};
use kujira::{KujiraQuerier, KujiraQuery, NormalizedPrice};
use std::collections::HashMap;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum OracleError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Denom {0} is not configured")]
    UnconfiguredDenom(String),
}

#[cw_serde]
pub struct Oracle {
    pub data: HashMap<String, OracleMeta>,
}

impl Oracle {
    pub fn new(v: Vec<(String, OracleMeta)>) -> Oracle {
        Oracle {
            data: v
                .into_iter()
                .fold(HashMap::new(), |mut map, (denom, meta)| {
                    map.insert(denom, meta);
                    map
                }),
        }
    }

    pub fn query_price(
        &self,
        querier: &QuerierWrapper<'_, KujiraQuery>,
        denom: &str,
    ) -> Result<NormalizedPrice, OracleError> {
        if !self.data.contains_key(denom) {
            return Err(OracleError::UnconfiguredDenom(denom.to_string()));
        }
        let OracleMeta {
            oracle_denom,
            decimals,
        } = self.data[denom].clone();
        Ok(NormalizedPrice::from_oracle(
            &KujiraQuerier::new(querier),
            oracle_denom,
            decimals,
        )?)
    }
}
