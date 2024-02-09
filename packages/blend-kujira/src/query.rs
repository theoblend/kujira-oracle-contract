use cosmwasm_std::{Deps, QueryRequest, StdResult, Uint128};

use kujira::{
    BankQuery as KujiraBankQuery, Denom, KujiraQuery, SupplyResponse as KujiraSupplyResponse,
};

pub fn get_supply(deps: &Deps<KujiraQuery>, denom: String) -> StdResult<Uint128> {
    let request = QueryRequest::Custom(KujiraQuery::Bank(KujiraBankQuery::Supply {
        denom: Denom::from(denom),
    }));
    let response: KujiraSupplyResponse = deps.querier.query(&request)?;
    Ok(response.amount.amount)
}
