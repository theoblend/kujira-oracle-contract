use crate::error::ContractError;
use crate::state::{ORACLE, OWNER};
use blend::oracle::msg::{
    MigrateMsg, OracleConfigResponse, OracleExecuteMsg, OracleInstantiateMsg, OracleMeta,
    OracleQueryMsg, OracleUpdate, PriceResponse,
};
use blend::owner_util::init_owner;
use blend::util::non_migratable_err;
use blend_kujira::oracle::Oracle;
use cosmwasm_std::{
    entry_point, to_binary, Binary, Coin, Deps, DepsMut, Env, MessageInfo, Response,
};
use cw2::set_contract_version;
use kujira::KujiraQuery;

pub const CONTRACT_NAME: &str = "crates.io:blend-kujira-oracle";
pub const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

pub type ContractResult<T> = core::result::Result<T, ContractError>;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(deps: DepsMut, env: Env, _: MigrateMsg) -> ContractResult<Response> {
    let version = cw2::get_contract_version(deps.storage)?;
    Err(ContractError::Std(non_migratable_err(
        env.contract.address.as_ref(),
        version,
    )))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: OracleInstantiateMsg,
) -> ContractResult<Response> {
    ORACLE.save(deps.storage, &Oracle::new(msg.oracle_config))?;
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    init_owner(deps, msg.owner, OWNER)?;
    Ok(Response::new())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: OracleExecuteMsg,
) -> ContractResult<Response> {
    match msg {
        OracleExecuteMsg::UpdateOwner(update) => Ok(OWNER.update(deps, info, update)?),
        OracleExecuteMsg::Update(OracleUpdate { oracle_config }) => {
            OWNER.assert_owner(deps.storage, &info.sender)?;
            if let Some(oracle_config) = oracle_config {
                ORACLE.save(deps.storage, &Oracle::new(oracle_config))?;
            }
            Ok(Response::default())
        }
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(
    deps: Deps<KujiraQuery>,
    _env: Env,
    msg: OracleQueryMsg,
) -> Result<Binary, ContractError> {
    let oracle = ORACLE.load(deps.storage)?;
    match msg {
        OracleQueryMsg::Price { coin } => Ok(to_binary(&PriceResponse {
            price: Coin {
                amount: coin.amount * oracle.query_price(&deps.querier, &coin.denom)?,
                denom: "USD".to_string(),
            },
        })?),
        OracleQueryMsg::Config {} => Ok(to_binary(&OracleConfigResponse {
            config: ORACLE
                .load(deps.storage)?
                .data
                .into_iter()
                .collect::<Vec<(String, OracleMeta)>>(),
        })?),
    }
}
