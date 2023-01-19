#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;
mod types;

pub use pallet::*;
#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::{
		pallet_prelude::*, ensure,
		sp_runtime::traits::{Scale, IdentifyAccount, Member, Verify},
		traits::{Time, IsType},
	};
	use frame_system::pallet_prelude::*;
	use scale_info::{prelude::vec::Vec, StaticTypeInfo };
	use crate::types::*;

	#[pallet::pallet]
	#[pallet::without_storage_info]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		type Public: IdentifyAccount<AccountId = Self::AccountId> + Encode + Decode + Member + TypeInfo;
		type Signature: Verify<Signer = Self::Public> + Member + Parameter + Decode + Encode + From<sp_core::sr25519::Signature> + TypeInfo;
		type Moment: Parameter
		+ Default
		+ Scale<Self::BlockNumber, Output = Self::Moment>
		+ Copy
		+ MaxEncodedLen
		+ StaticTypeInfo;
		type Timestamp: Time<Moment=Self::Moment> ;
		
	}

	// The pallet's runtime schema storage.
	#[pallet::storage]
	#[pallet::getter(fn schema_registry)]
	pub type SchemaStore<T: Config> =
		StorageMap<_, Blake2_128Concat, T::Signature, VerifiableCredentialSchema<T::Moment>,  OptionQuery>;

	#[pallet::storage]
	#[pallet::getter(fn credential_registry)]
	pub type CredentialStore<T: Config> =
		StorageMap<_, Blake2_128Concat, T::Signature, VerifiableCredential<T::AccountId, T::Moment>,  OptionQuery>;
	
	#[pallet::storage]
	#[pallet::getter(fn get_nonce)]
	pub(super) type Nonce<T: Config> = StorageValue<_,u64,ValueQuery>;
		
	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/main-docs/build/events-errors/
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters. [something, who]
		// Event is emitted when a Schema item is created
		SchemaCreated(T::Signature, VerifiableCredentialSchema<T::Moment>),
		// Event is emitted when a Credential item is created
		CredentialCreated(T::Signature, VerifiableCredential<T::AccountId, T::Moment>),
		// Event is emitted when an existing Schema item is updated
		SchemaUpdated(T::Signature, VerifiableCredentialSchema<T::Moment>),
		// Event is emitted when an existing credential item is updated
		CredentialUpdated(T::Signature, VerifiableCredential<T::AccountId, T::Moment>),
		// Event is emitted when an existing Schema item is deleted
		SchemaDeleted(T::Signature),
		// Event is emitted when an existing Credential item is deleted
		CredentialDeleted(T::Signature),

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
		/// Errors emitted when credential already exists.
		CredentialAlreadyExists,
		///Error emitted when credential is unknown
		UnknownCredential,
		/// Error emitted when signature is invalid
		SignatureVerifyError,
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Create a new schema item
		#[pallet::weight(10 + T::DbWeight::get().writes(1).ref_time())]
		pub fn create_schema(origin: OriginFor<T>, 
			name: Vec<u8>, 
			creator: Vec<u8>,
			public: bool,
			mandatory_fields: Vec<Attribute>,
			expiration_date: Option<T::Moment>,
			issuer_claims: Vec<Claim>,
			subject_claims: Vec<Claim>,
			credential_claims: Vec<Claim>,			
			metadata: Vec<u8>,
			signature: T::Signature,
			) -> DispatchResult {
			// Ensure that the caller of the function is signed
			let _ = ensure_signed(origin)?;
			let nonce = Self::get_nonce();
			// Create a new Schema item
			let verifiable_credential_schema = VerifiableCredentialSchema {
				name,
				creator,
				public,
				creation_date: T::Timestamp::now(),
				expiration_date,
				mandatory_fields,
				issuer_claims,
				subject_claims,
				credential_claims,
				metadata,
				nonce,
			};
			// sign the schema
			
			let _ = Self::increment_nonce()?;
			// Ensure that the Schema does not already exist
			ensure!(!SchemaStore::<T>::contains_key(&signature), "Schema already exists");
			// Save the Schema data in storage
			SchemaStore::<T>::insert(&signature, &verifiable_credential_schema);
			// Emit an event to indicate that the Schema was created
			Self::deposit_event(Event::SchemaCreated(signature, verifiable_credential_schema));
			Ok(())
		}

		/// Create a new credential item
		#[pallet::weight(10 + T::DbWeight::get().writes(1).ref_time())]
		pub fn create_credential(origin: OriginFor<T>, 
			context: Vec<u8>,
			schema: Vec<u8>,
			issuer: Option<T::AccountId>,
			claim: Vec<Claim>,
			expiration_date: Option<T::Moment>,
			subject: Vec<u8>,
			credential_holder: Vec<u8>,
			signature: T::Signature,
			) -> DispatchResult {

			// Ensure that the caller of the function is signed
			let _ = ensure_signed(origin)?;
			let verifiable_credential = VerifiableCredential {
				context,
				schema,
				issuer,
				claim,
				issuance_date: T::Timestamp::now(),
				expiration_date,
				subject,
				credential_holder,
			};
			let _ = Self::increment_nonce()?;
			// Ensure that the Credential does not already exist
			ensure!(!CredentialStore::<T>::contains_key(&signature), "Credential already exists");
			// Save the Credential data in storage
			CredentialStore::<T>::insert(&signature, &verifiable_credential);
			// Emit an event to indicate that the Credential was created and stored
			Self::deposit_event(Event::CredentialCreated(signature, verifiable_credential));
			Ok(())
		}

		// Function to update an existing schema
		#[pallet::weight(10 + T::DbWeight::get().writes(1).ref_time())]
		pub fn update_schema(origin: OriginFor<T>, old_schema_key:T::Signature, new_data: VerifiableCredentialSchema<T::Moment>) -> DispatchResult {
			let _ = ensure_signed_or_root(origin)?;
			let schema_data = SchemaStore::<T>::get(&old_schema_key).ok_or(Error::<T>::UnknownSchema)?;
			ensure!(schema_data != new_data, Error::<T>::SchemaAlreadyExists);
			// Update the schema data
			SchemaStore::<T>::insert(&old_schema_key, &new_data);
			let _ = Self::increment_nonce()?;
			Self::deposit_event(Event::SchemaUpdated(old_schema_key, new_data));
			Ok(())
		}

		// Function to update an existing credential
		#[pallet::weight(10 + T::DbWeight::get().writes(1).ref_time())]
		pub fn update_credential(origin: OriginFor<T>, old_credential_sig: T::Signature, new_data: VerifiableCredential<T::AccountId, T::Moment>) -> DispatchResult {
			let _ = ensure_signed_or_root(origin)?;
			let credential_data = CredentialStore::<T>::get(&old_credential_sig).ok_or(Error::<T>::UnknownCredential)?;
			ensure!(credential_data != new_data, Error::<T>::CredentialAlreadyExists);
			// Update the credential data
			CredentialStore::<T>::insert(&old_credential_sig, &new_data);
			let _ = Self::increment_nonce()?;
			Self::deposit_event(Event::CredentialUpdated(old_credential_sig, new_data));
			Ok(())
		}

		// Function to delete an existing schema
		#[pallet::weight(10 + T::DbWeight::get().reads_writes(1,1).ref_time())]
        pub fn delete_schema(origin: OriginFor<T>, key: T::Signature) -> DispatchResult {
            let _ = ensure_signed(origin)?;
            ensure!(<SchemaStore<T>>::contains_key(&key), Error::<T>::UnknownSchema);
            <SchemaStore<T>>::remove(&key);
            Self::deposit_event(Event::SchemaDeleted(key));
            Ok(())
        }

		// Function to delete an existing credential
		#[pallet::weight(10 + T::DbWeight::get().reads_writes(1,1).ref_time())]
        pub fn delete_credential(origin: OriginFor<T>, key: T::Signature) -> DispatchResult {
            let _ = ensure_signed(origin)?;
            ensure!(<CredentialStore<T>>::contains_key(&key), Error::<T>::UnknownCredential);
            <CredentialStore<T>>::remove(&key);
            Self::deposit_event(Event::CredentialDeleted(key));
            Ok(())
        }

		// Function to verify signature on data
		#[pallet::weight(10 + T::DbWeight::get().reads_writes(1,1).ref_time())]
		pub fn verify_sig(origin: OriginFor<T>, data: Vec<u8>, sig: T::Signature, from: T::Public) -> DispatchResult{
			let _ = ensure_signed(origin)?;
			let ok = sig.verify(&data[..], &from.into_account());
			// `ok` is a bool. Use in an `if` or `ensure!`.
			ensure!(ok, <Error<T>>::SignatureVerifyError);
			Ok(())
		}

	}

	impl<T: Config> Pallet<T>{
		// Function to sign data with account public key
		fn increment_nonce() -> DispatchResult {
			<Nonce<T>>::try_mutate(|nonce| {
				let next = nonce.checked_add(1).ok_or("Overflow")?;
				*nonce = next;
				
				Ok(().into())
			})
		}
	}


}