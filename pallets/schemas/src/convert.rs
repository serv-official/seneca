use codec::{Decode, Encode};
use sp_core::crypto::AccountId32;
pub use sp_std::{str, vec, vec::Vec};
use frame_support::pallet_prelude::DispatchError;


pub fn convert_string_to_accountid<AccountId>(account_str: &str) -> Result<AccountId, DispatchError>
where
	AccountId: Encode + ?Sized + Decode,
{
	let mut output = vec![0xFF; 35];
	bs58::decode(account_str).into(&mut output).unwrap();
	let cut_address_vec: Vec<u8> = output.drain(1..33).collect();
	let mut array = [0; 32];
	let bytes = &cut_address_vec[..array.len()];
	array.copy_from_slice(bytes);
	let account32: AccountId32 = array.into();
	let mut to32 = AccountId32::as_ref(&account32);
	let to_address = match AccountId::decode(&mut to32){
		Ok(a) => a,
		Err(e) => {
			log::error!("{:?}", e);
			return Err(DispatchError::Other("Error converting string to AccountId"))
		},
	};
	Ok(to_address)
}