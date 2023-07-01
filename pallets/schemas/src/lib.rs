#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(test)]
mod mock;

pub mod schema;
#[cfg(test)]
mod tests;
pub mod types;

pub use pallet::*;
#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
pub mod weights;

#[frame_support::pallet]
pub mod pallet {
	use crate::schema::{Schema, SchemaInterface};
	use crate::types::*;
	use crate::weights::WeightInfo;
	use codec::HasCompact;
	use frame_support::{
		ensure,
		pallet_prelude::*,
		sp_runtime::traits::{IdentifyAccount, Member, Scale, Verify},
		traits::{IsType, Time},
	};
	use frame_system::pallet_prelude::*;
	use scale_info::{prelude::vec::Vec, StaticTypeInfo};

	#[pallet::pallet]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(_);

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config{
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		type Public: IdentifyAccount<AccountId = Self::AccountId>
			+ Encode
			+ Decode
			+ Member
			+ From<sp_core::sr25519::Public>
			+ Into<sp_core::sr25519::Public>
			+ TypeInfo;
		type Signature: Verify<Signer = Self::Public>
			+ Member
			+ Parameter
			+ Decode
			+ Encode
			+ From<sp_core::sr25519::Signature>
			+ Into<sp_core::sr25519::Signature>
			+ TypeInfo;
		type Moment: Parameter
			+ Default
			+ Scale<Self::BlockNumber, Output = Self::Moment>
			+ Copy
			+ MaxEncodedLen
			+ StaticTypeInfo;
		type Timestamp: Time<Moment = Self::Moment>;
		type WeightInfo: WeightInfo;
		/// Identifier for the schema.
		type SchemaId: Parameter
			+ Default
			+ Copy
			+ HasCompact
			+ MaybeSerializeDeserialize
			+ Ord
			+ PartialOrd
			+ MaxEncodedLen
			+ From<u32>
			+ Into<u32>
			+ TypeInfo;
	}

	// The pallet's runtime schema storage.
	#[pallet::storage]
	#[pallet::getter(fn schema_registry)]
	pub type SchemaStore<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		T::SchemaId,
		(T::Signature, VerifiableCredentialSchema<T::Moment>),
		OptionQuery,
	>;

	#[pallet::storage]
	#[pallet::getter(fn get_nonce)]
	pub(super) type Nonce<T: Config> = StorageValue<_, u64, ValueQuery>;


	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/main-docs/build/events-errors/
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters. [something, who]
		// Event is emitted when a Schema item is created
		SchemaCreated(T::SchemaId, Vec<u8>),
		// Event is emitted when an existing Schema item is updated
		SchemaUpdated(T::SchemaId, Vec<u8>),
		// Event is emitted when an existing Schema item is deleted
		SchemaDeleted(T::SchemaId),
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Error names should be descriptive.
		NoneValue,
		/// Errors emitted when schema already exists.
		SchemaAlreadyExists,
		///Error emitted when schema is unknown
		UnknownSchema,
		///Error emitted when schema id doesn't exist
		SchemaIdDoesNotExist,
		/// Error emitted when signature is invalid
		SignatureVerifyError,
		/// Error emitted when invalid DID is used
		InvalidDID,
		/// Error emitted when the origin and schema creator don't match
		NotSchemaOwner

	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Create a new schema item
		#[pallet::call_index(1)]
		#[pallet::weight(T::WeightInfo::create_schema())]
		pub fn create_schema(
			origin: OriginFor<T>,
			#[pallet::compact] id: T::SchemaId,
			name: Vec<u8>,
			creator: Vec<u8>,
			public: bool,
			mandatory_fields: Vec<Attribute>,
			creation_date: T::Moment,
			expiration_date: Option<T::Moment>,
			issuer_claims: Vec<Claim>,
			subject_claims: Vec<Claim>,
			credential_claims: Vec<Claim>,
			metadata: Vec<u8>,
			signature: T::Signature,
			nonce: u64,
		) -> DispatchResult {
			// Ensure that the caller of the function is signed
			let origin = ensure_signed(origin)?;
			// convert schema creator DID to account id
			let schema_creator = Self::split_publickey_from_did(&creator)?;
			// ensure schema creator and origin are the same
			ensure!(schema_creator == origin, Error::<T>::NotSchemaOwner);
			// Ensure that the Schema does not already exist
			ensure!(!SchemaStore::<T>::contains_key(&id), "Schema already exists");
			// Create a new Schema item
			Self::create_verifiable_schema(
				&id,
				&name,
				&creator,
				&public,
				creation_date,
				expiration_date,
				&mandatory_fields,
				&issuer_claims,
				&subject_claims,
				&credential_claims,
				&metadata,
				&signature,
				&nonce,
			)
		}

		// Function to update an existing schema
		#[pallet::call_index(3)]
		#[pallet::weight(T::WeightInfo::update_schema())]
		pub fn update_schema(
			origin: OriginFor<T>,
			#[pallet::compact] old_schema_key: T::SchemaId,
			new_data: (T::Signature, VerifiableCredentialSchema<T::Moment>),
		) -> DispatchResult {

			let origin = ensure_signed(origin)?;
			// fetch schema from schema store
			let schema_data =
				SchemaStore::<T>::get(&old_schema_key).ok_or(Error::<T>::UnknownSchema)?;
			// ensure schema creator is the one updating the schema
			let schema_creator = Self::split_publickey_from_did(&schema_data.1.creator)?;
			ensure!(schema_creator == origin, Error::<T>::NotSchemaOwner);
			ensure!(schema_data != new_data, Error::<T>::SchemaAlreadyExists);
			// Update the schema data
			Self::update_verifiable_schema(&old_schema_key, &new_data)
		}

		// Function to delete an existing schema
		#[pallet::call_index(5)]
		#[pallet::weight(T::WeightInfo::delete_schema())]
		pub fn delete_schema(
			origin: OriginFor<T>,
			#[pallet::compact] key: T::SchemaId,
		) -> DispatchResult {
			let origin = ensure_signed(origin)?;
			// fetch schema from schema store
			let schema_data =
			SchemaStore::<T>::get(&key).ok_or(Error::<T>::UnknownSchema)?;
			// convert schema DID to account id
			let schema_creator = Self::split_publickey_from_did(&schema_data.1.creator)?;
			// ensure schema creator is the one updating the schema
			ensure!(schema_creator == origin, Error::<T>::NotSchemaOwner);
			Self::delete_verifiable_schema(&key)
		}
	}

	impl<T: Config> Schema<T::AccountId, T::Moment, T::Signature, T::SchemaId> for Pallet<T> {
		// Function to create a new schema
		fn create_verifiable_schema(
			id: &T::SchemaId,
			name: &Vec<u8>,
			creator: &Vec<u8>,
			public: &bool,
			creation_date: T::Moment,
			expiration_date: Option<T::Moment>,
			mandatory_fields: &Vec<Attribute>,
			issuer_claims: &Vec<Claim>,
			subject_claims: &Vec<Claim>,
			credential_claims: &Vec<Claim>,
			metadata: &Vec<u8>,
			signature: &T::Signature,
			nonce: &u64,
		) -> DispatchResult {
			let verifiable_credential_schema = VerifiableCredentialSchema {
				name: name.clone(),
				creator: creator.clone(),
				public: public.clone(),
				creation_date,
				expiration_date,
				mandatory_fields: mandatory_fields.clone(),
				issuer_claims: issuer_claims.clone(),
				subject_claims: subject_claims.clone(),
				credential_claims: credential_claims.clone(),
				metadata: metadata.clone(),
				nonce: nonce.clone(),
			};
			let binding = verifiable_credential_schema.encode();
			let vc_bytes = binding.as_slice();

			let signer = Self::split_publickey_from_did(&verifiable_credential_schema.creator)?;
			Self::is_valid_signer(vc_bytes, signature, &signer)?;
			// Save the Schema data in storage
			SchemaStore::<T>::insert(id, (&signature, &verifiable_credential_schema));
			// Emit an event to indicate that the Schema was created
			Self::deposit_event(Event::SchemaCreated(
				id.clone(),
				verifiable_credential_schema.encode(),
			));
			Ok(())
		}

		// update a schema
		fn update_verifiable_schema(
			old_schema_key: &T::SchemaId,
			new_data: &(T::Signature, VerifiableCredentialSchema<T::Moment>),
		) -> DispatchResult {
			// Update the schema data
			SchemaStore::<T>::insert(old_schema_key, new_data);
			Self::deposit_event(Event::SchemaUpdated(old_schema_key.clone(), new_data.encode()));
			Ok(())
		}

		// delete schema
		fn delete_verifiable_schema(key: &T::SchemaId) -> DispatchResult {
			<SchemaStore<T>>::remove(key);
			Self::deposit_event(Event::SchemaDeleted(key.clone()));
			Ok(())
		}

		fn is_valid_signer(data: &[u8], sig: &T::Signature, from: &T::AccountId) -> DispatchResult {
			ensure!(sig.verify(data, from), <Error<T>>::SignatureVerifyError);
			Ok(())
		}

		fn split_publickey_from_did(did: &Vec<u8>) -> Result<T::AccountId, DispatchError> {
			let did_string = match sp_std::str::from_utf8(did) {
				Ok(did_str) => did_str,
				Err(e) => {
					log::error!("{:?}", e);
					return Err(<Error<T>>::InvalidDID.into());
				},
			};
			let did_vec: Vec<&str> = did_string.split(":").collect();
			let public_key_str = did_vec[2].trim();
			match node_primitives::convert2accountid::convert_string_to_accountid(public_key_str) {
				Ok(account_id) => Ok(account_id),
				Err(e) => Err(e),
			}
		}
	}

	impl<T: Config> SchemaInterface for Pallet<T> {
		type SchemaId = T::SchemaId;
		fn check_schema_id_exists(schema: Self::SchemaId) -> DispatchResult {
			ensure!(<SchemaStore<T>>::contains_key(&schema), Error::<T>::SchemaIdDoesNotExist);
			Ok(())
		}

		fn to_schema_id(schema_id: &u32) -> T::SchemaId {
			let returned_schema_id: T::SchemaId = T::SchemaId::from(*schema_id);
			returned_schema_id
		}
	}
}
