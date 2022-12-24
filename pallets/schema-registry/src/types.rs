use codec::{Decode, Encode};
use scale_info::TypeInfo;
use frame_support::RuntimeDebug;
use sp_core::sr25519;
use scale_info::prelude::vec::Vec;

/// schema registry structure
#[derive(PartialEq, Eq, TypeInfo, Clone, Encode, Decode, RuntimeDebug)]
pub struct Registry {
	pub id: u64,
    pub address: sr25519::Public,
    pub registry_type: RegistryType,
	pub data: Vec<u8>,
}

#[derive(PartialEq, Eq, TypeInfo, Clone, Encode, Decode, RuntimeDebug)]
pub enum RegistryType{
    CredentialRegsitry,
    DidRegistry,
    SchemaRegistry,
}