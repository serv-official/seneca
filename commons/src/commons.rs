// This file is part of Substrate.

// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use codec::{Decode, Encode};
use sp_core::crypto::AccountId32;
use sp_runtime::{DispatchError, traits::Verify};
use sp_std::{str, vec, vec::Vec};
use sp_core::sr25519::Signature;
use sp_core::sr25519::Public;

/// function converts string to accountid
pub fn convert_string_to_accountid<AccountId>(account_str: &str) -> Result<AccountId, DispatchError>
where
	AccountId: Encode + ?Sized + Decode,
{
	let mut output = vec![0xFF; 35];
	match bs58::decode(account_str).into(&mut output) {
		Ok(_) => {}
		Err(err) => {
			// Error: Handle the error here or propagate it further
			log::error!("Error decoding: {}", err);
			// Additional error handling logic can be added here
			return Err(DispatchError::Other("Error decoding string"));
		}
	}
	let cut_address_vec: Vec<u8> = output.drain(1..33).collect();
	let mut array = [0; 32];
	let bytes = &cut_address_vec[..array.len()];
	array.copy_from_slice(bytes);
	let account32: AccountId32 = array.into();
	let mut to32 = AccountId32::as_ref(&account32);
	let to_address = match AccountId::decode(&mut to32) {
		Ok(a) => a,
		Err(e) => {
			log::error!("Error decoding: {}", e);
			return Err(DispatchError::Other("Error converting string to AccountId"));
		},
	};
	Ok(to_address)
}

pub fn is_valid_signer(data: &[u8], sig: &Signature, from: &Public) -> bool{
	sig.verify(data, from)
}

pub fn split_publickey_from_did(did: &Vec<u8>) -> Result<AccountId32, DispatchError> {
	let did_string = match sp_std::str::from_utf8(did) {
		Ok(did_str) => did_str,
		Err(e) => {
			log::error!("{:?}", e);
			return Err(DispatchError::Other("Invalid DID"));
		},
	};
	let did_vec: Vec<&str> = did_string.split(":").collect();
	let public_key_str = did_vec[2].trim();
	match convert_string_to_accountid(public_key_str) {
		Ok(account_id) => Ok(account_id),
		Err(e) => Err(e),
	}
}