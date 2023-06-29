#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(test)]
mod mock;

pub mod credential;
#[cfg(test)]
mod tests;
mod types;

pub use pallet::*;
#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
pub mod weights;

#[frame_support::pallet]
pub mod pallet {
	use crate::credential::Credential;
	use crate::types::*;
	use crate::weights::WeightInfo;
	use codec::HasCompact;
	use frame_support::{
		ensure,
		pallet_prelude::*,
		sp_runtime::traits::{IdentifyAccount, Member, Verify},
		traits::IsType
	};
	use frame_system::pallet_prelude::*;
	use pallet_schemas::schema::SchemaInterface;
	use scale_info::prelude::vec::Vec;

	#[pallet::pallet]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(_);

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config + pallet_schemas::Config{
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		type Public: IdentifyAccount<AccountId = Self::AccountId>
			+ Encode
			+ Decode
			+ Member
			+ From<sp_core::sr25519::Public>
			+ TypeInfo;
		type CredentialsWeightInfo: WeightInfo;
		/// Identifier for the class of credential.
		type CredentialId: Member
			+ Parameter
			+ Default
			+ Copy
			+ HasCompact
			+ MaybeSerializeDeserialize
			+ Ord
			+ PartialOrd
			+ MaxEncodedLen
			+ TypeInfo;
		type SchemaCheck: SchemaInterface;
	}

	#[pallet::storage]
	#[pallet::getter(fn credential_registry)]
	pub type CredentialStore<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		T::CredentialId,
		(T::Signature, VerifiableCredential<T::Moment>),
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
		// Event is emitted when a Credential item is created
		CredentialCreated(T::CredentialId, Vec<u8>),
		// Event is emitted when an existing credential item is updated
		CredentialUpdated(T::CredentialId, Vec<u8>),
		// Event is emitted when an existing Credential item is deleted
		CredentialDeleted(T::CredentialId),
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Error names should be descriptive.
		NoneValue,
		/// Errors emitted when credential already exists.
		CredentialAlreadyExists,
		///Error emitted when credential is unknown
		UnknownCredential,
		/// Error emitted when signature is invalid
		SignatureVerifyError,
		/// Error emitted when invalid DID is used
		InvalidDID,
		/// Error emitted when credential issuer and origin don't match
		NotCredentialOwner
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Create a new credential item
		#[pallet::call_index(2)]
		#[pallet::weight(T::CredentialsWeightInfo::create_credential())]
		pub fn create_credential(
			origin: OriginFor<T>,
			#[pallet::compact] id: T::CredentialId,
			context: Vec<u8>,
			schema: u32,
			issuer: Vec<u8>,
			issuance_date: Option<T::Moment>,
			expiration_date: Option<T::Moment>,
			subject: Subject,
			credential_holder: Vec<u8>,
			signature: T::Signature,
			nonce: u64,
		) -> DispatchResult {
			// Ensure that the caller of the function is signed
			let origin = ensure_signed(origin)?;
			// ensure credential issuer is the same as the origin of the extrinsic
			let credential_creator = Self::split_publickey_from_did(&issuer)?;
			ensure!(credential_creator == origin, Error::<T>::NotCredentialOwner);
			let schema_id = T::SchemaCheck::to_schema_id(&schema);
			//Ensure schema id exists
			ensure!(
				T::SchemaCheck::check_schema_id_exists(schema_id).is_ok(),
				"Schema does not exist"
			);
			// Ensure that the Credential does not already exist
			ensure!(!CredentialStore::<T>::contains_key(&id), "Credential already exists");
			Self::create_verifiable_credential(
				&id,
				&context,
				&schema,
				&issuer,
				issuance_date,
				expiration_date,
				&subject,
				&credential_holder,
				&signature,
				&nonce,
			)
		}

		// Function to update an existing credential
		#[pallet::call_index(4)]
		#[pallet::weight(T::CredentialsWeightInfo::update_credential())]
		pub fn update_credential(
			origin: OriginFor<T>,
			#[pallet::compact] old_credential_key: T::CredentialId,
			new_data: (T::Signature, VerifiableCredential<T::Moment>),
		) -> DispatchResult {
			let origin = ensure_signed(origin)?;
			// fetch credential from credential store
			let credential_data =
				CredentialStore::<T>::get(&old_credential_key).ok_or(Error::<T>::UnknownCredential)?;
			// ensure credential creator is the one updating the credential
			let credential_creator = Self::split_publickey_from_did(&credential_data.1.issuer)?;
			ensure!(credential_creator == origin, Error::<T>::NotCredentialOwner);
			ensure!(credential_data != new_data, Error::<T>::CredentialAlreadyExists);
			// Update the credential data
			Self::update_verifiable_credential(&old_credential_key, &new_data)
		}

		// Function to delete an existing credential
		#[pallet::call_index(6)]
		#[pallet::weight(T::CredentialsWeightInfo::delete_credential())]
		pub fn delete_credential(
			origin: OriginFor<T>,
			#[pallet::compact] key: T::CredentialId,
		) -> DispatchResult {
			let origin = ensure_signed(origin)?;
			// fetch credential from credential store
			let credential_data =
				CredentialStore::<T>::get(&key).ok_or(Error::<T>::UnknownCredential)?;
			// ensure credential creator is the one updating the credential
			let credential_creator = Self::split_publickey_from_did(&credential_data.1.issuer)?;
			ensure!(credential_creator == origin, Error::<T>::NotCredentialOwner);
			Self::delete_verifiable_credential(&key)
		}
	}

	impl<T: Config> Credential<T::AccountId, T::Moment, T::Signature, T::CredentialId> for Pallet<T> {
		// create a new credential
		fn create_verifiable_credential(
			id: &T::CredentialId,
			context: &Vec<u8>,
			schema: &u32,
			issuer: &Vec<u8>,
			issuance_date: Option<T::Moment>,
			expiration_date: Option<T::Moment>,
			subject: &Subject,
			credential_holder: &Vec<u8>,
			signature: &T::Signature,
			nonce: &u64,
		) -> DispatchResult {
			let verifiable_credential = VerifiableCredential {
				context: context.clone(),
				schema: schema.clone(),
				issuer: issuer.clone(),
				issuance_date,
				expiration_date,
				subject: subject.clone(),
				credential_holder: credential_holder.clone(),
				nonce: nonce.clone(),
			};
			let binding = verifiable_credential.encode();
			let vc_bytes = binding.as_slice();
			let signer = Self::split_publickey_from_did(&verifiable_credential.issuer)?;
			Self::is_valid_signer(vc_bytes, signature, &signer)?;
			// Save the Schema data in storage
			CredentialStore::<T>::insert(&id, (&signature, &verifiable_credential));
			// Emit an event to indicate that the Credential was created and stored
			Self::deposit_event(Event::CredentialCreated(
				id.clone(),
				verifiable_credential.encode(),
			));
			Ok(())
		}
		// update a credential
		fn update_verifiable_credential(
			old_credential_key: &T::CredentialId,
			new_data: &(T::Signature, VerifiableCredential<T::Moment>),
		) -> DispatchResult {
			// Update the credential data
			CredentialStore::<T>::insert(old_credential_key, new_data);
			Self::deposit_event(Event::CredentialUpdated(
				old_credential_key.clone(),
				new_data.encode(),
			));
			Ok(())
		}
		// delete a credential
		fn delete_verifiable_credential(key: &T::CredentialId) -> DispatchResult {
			<CredentialStore<T>>::remove(key);
			Self::deposit_event(Event::CredentialDeleted(key.clone()));
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

		// Fetch credentials by schemaid
		fn get_credentials_by_schemaid(
			schema_id: &u32,
		) -> Vec<(T::CredentialId, VerifiableCredential<T::Moment>)> {
			let mut credentials: Vec<(T::CredentialId, VerifiableCredential<T::Moment>)> =
				Vec::new();
			for (key, value) in CredentialStore::<T>::iter() {
				if value.1.schema == *schema_id {
					credentials.push((key, value.1.clone()));
				}
			}
			credentials
		}

		// Fetch credentials by subject
		fn get_credentials_by_subject(
			subject: &Subject,
		) -> Vec<(T::CredentialId, VerifiableCredential<T::Moment>)> {
			let mut credentials: Vec<(T::CredentialId, VerifiableCredential<T::Moment>)> =
				Vec::new();
			for (key, value) in CredentialStore::<T>::iter() {
				if value.1.subject == *subject {
					credentials.push((key, value.1.clone()));
				}
			}
			credentials
		}

		// Fetch credentials by holder
		fn get_credentials_by_holder(
			holder: &Vec<u8>,
		) -> Vec<(T::CredentialId, VerifiableCredential<T::Moment>)> {
			let mut credentials: Vec<(T::CredentialId, VerifiableCredential<T::Moment>)> =
				Vec::new();
			for (key, value) in CredentialStore::<T>::iter() {
				if value.1.credential_holder == *holder {
					credentials.push((key, value.1.clone()));
				}
			}
			credentials
		}

		// Fetch credentials by creator
		fn get_credentials_by_creator(
			creator: &Vec<u8>,
		) -> Vec<(T::CredentialId, VerifiableCredential<T::Moment>)> {
			let mut credentials: Vec<(T::CredentialId, VerifiableCredential<T::Moment>)> =
				Vec::new();
			for (key, value) in CredentialStore::<T>::iter() {
				if value.1.issuer == *creator {
					credentials.push((key, value.1.clone()));
				}
			}
			credentials
		}
	}
}
