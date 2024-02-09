use cosmwasm_schema::write_api;

use blend::oracle::msg::{OracleExecuteMsg, OracleInstantiateMsg, OracleQueryMsg};

fn main() {
    write_api! {
        instantiate: OracleInstantiateMsg,
        execute: OracleExecuteMsg,
        query: OracleQueryMsg,
    }
}
