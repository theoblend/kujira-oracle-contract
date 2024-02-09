use blend::storage_key::as_str;
use blend_kujira::oracle::Oracle;
use cw_storage_plus::Item;
use mars_owner::Owner;

#[repr(u8)]
pub enum StorageKey {
    Owner = b'a',
    Oracle = b'o',
}

pub const OWNER: Owner = Owner::new(as_str(&StorageKey::Owner));
pub const ORACLE: Item<Oracle> = Item::new(as_str(&StorageKey::Oracle));
