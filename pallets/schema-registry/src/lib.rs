#![cfg_attr(not(feature = "std"), no_std)]
#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;
mod types;

use serde::{Deserialize, Serialize};
use serde_json::{from_str, to_string};
use std::time::{SystemTime, UNIX_EPOCH, Duration};
use uuid::Uuid;
pub use pallet::*;
use sp_core::sr25519::Signature;
use std::time::{SystemTime, UNIX_EPOCH};
#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

#[derive(PartialEq, Eq, TypeInfo, Clone, Encode, Decode, RuntimeDebug)]
pub struct VerifiableCredential {
	id: u64,
	context: String,
	schema: String,
	issuer: String,
	issuance_date: SystemTime,
	expiration_date: SystemTime,
	subject: String,
	credential_holder: String,
	signature: Signature,
}
pub struct FinalVerifiableCredential {
	id: u64,
	context: String,
	schema: String,
	issuer: String,
	issuance_date: SystemTime,
	expiration_date: SystemTime,
	subject: String,
	credential_holder: String,
	signature: Signature,
}
#[frame_support::pallet]
pub mod pallet {

	
	struct VerifiableCredential {
		id: u64,
		context: String,
		schema: String,
		issuer: String,
		issuance_date: SystemTime,
		expiration_date: SystemTime,
		subject: String,
		credential_holder: String,
		signature: String,
	}
	use crate::types::Registry;
	use frame_support::{ensure, pallet_prelude::*};
	use frame_system::pallet_prelude::*;

	#[pallet::pallet]
	#[pallet::without_storage_info]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
	}

	// The pallet's runtime storage items.
	#[pallet::storage]
	#[pallet::getter(fn store_regisrty)]
	pub type RegistryStore<T: Config> =
		StorageMap<_, Blake2_128Concat, T::Hash, Registry, OptionQuery>;

	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/main-docs/build/events-errors/
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters. [something, who]
		// Event is emitted when an existing Registry item is created
		Created(Registry),
		// Event is emitted when an existing Registry item is updated
		Updated(T::Hash, Registry),
		// Event is emitted when an existing Registry item is deleted
		Deleted(T::Hash),
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Error names should be descriptive.
		NoneValue,
		/// Errors emitted when schema already exists.
		SchemaAlreadyExists,
		/// Error emitted when bounded vec creation fails
		BoundedVecCreationFailed,
		///Error emitted when schema is unknown
		UnknownSchema,
		///Error emitted when schema is unknown
		SchemaNotFound,
		///Error emitted when schema values do not fit format
		SchemaFormatNotFollowed,
	}

	fn process_json(json_string: String) -> Vec<u8> {
    let mut credential: VerifiableCredential = match from_str(&json_string) {
        Ok(c) => c,
        Err(_) => return Vec::new(),
    };

    // Generate a new ID
		credential.id = Uuid::new_v4().as_u128() as u64;

		// Set the issuance date to the current time
		credential.issuance_date = SystemTime::now();

		// Set the expiration date to January 1, 3030
		let january_1_3030 = UNIX_EPOCH + Duration::from_secs(2524608000);
		credential.expiration_date = january_1_3030;

		// Serialize the modified struct into JSON data
		let modified_json = to_string(&credential).unwrap();

		// Turn the JSON data into a vector of bytes
		modified_json.as_bytes().to_vec()
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// An example dispatchable that takes a singles value as a parameter, writes the value to
		/// storage and emits an event. This function must be dispatched by a signed extrinsic.
		#[pallet::weight(0 + T::DbWeight::get().writes(1).ref_time())]
		pub fn create_schema(
			origin: OriginFor<T>,
			key: T::Hash,
			credential: String,
		) -> DispatchResult {
			// Ensure that the caller of the function is signed
			let _ = ensure_root(origin)?;
			// Ensure that the DID does not already exist
			ensure!(!RegistryStore::<T>::contains_key(&key), "Schema already exists");
			
			let VerifiedCredential = 
			// Save the DID data in storage
			credential.id = Uuid::new_v4();
			credential.issuance_date = SystemTime::now();

			let json_vec: Vec<u8> = serde_json::to_vec(&credential).unwrap();

			RegistryStore::<T>::insert(&key, &json_vec);
			// Emit an event to indicate that the DID was created
			Self::deposit_event(Event::Created(json_vec));
			Ok(())
		}
		// Function to update an existing schema
		#[pallet::weight(0 + T::DbWeight::get().writes(1).ref_time())]
		pub fn update_schema(
			origin: OriginFor<T>,
			key: T::Hash,
			new_data: Registry,
		) -> DispatchResult {
			let _ = ensure_root(origin)?;
			let schema_data =
				RegistryStore::<T>::get(key.clone()).ok_or(Error::<T>::UnknownSchema)?;
			ensure!(schema_data != new_data, Error::<T>::SchemaAlreadyExists);
			// Update the schema data
			RegistryStore::<T>::insert(&key, &new_data);
			Self::deposit_event(Event::Updated(key, new_data));
			Ok(())
		}

		// Function to delete an existing schema
		#[pallet::weight(0 + T::DbWeight::get().reads_writes(1,1).ref_time())]
		pub fn delete_schema(origin: OriginFor<T>, key: T::Hash) -> DispatchResult {
			let _ = ensure_root(origin)?;
			ensure!(<RegistryStore<T>>::contains_key(&key), Error::<T>::SchemaNotFound);
			<RegistryStore<T>>::remove(&key);
			Self::deposit_event(Event::Deleted(key));
			Ok(())
		}
	}
}
