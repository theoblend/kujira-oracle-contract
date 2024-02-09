use cosmwasm_std::{CustomQuery, DepsMut};
use mars_owner::{Owner, OwnerError, OwnerInit::SetInitialOwner};

pub fn init_owner<C: CustomQuery>(
    deps: DepsMut<C>,
    owner: String,
    store: Owner,
) -> Result<(), OwnerError> {
    store.initialize(deps.storage, deps.api, SetInitialOwner { owner })
}
