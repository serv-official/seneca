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
		pallet_prelude::*, ensure,traits::{Time, IsType},
		sp_runtime::traits::{Scale, IdentifyAccount, Member, Verify},
	};
	use uuid::Uuid;
	use scale_info::StaticTypeInfo;
	use frame_system::pallet_prelude::*;
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
		type Public: IdentifyAccount<AccountId = Self::AccountId>;
		type Signature: Verify<Signer = Self::Public> + Member + Decode + Encode + TypeInfo;
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
		StorageMap<_, Blake2_128Concat, T::Hash, VerifiableCredentialSchema<T::Moment, T::Signature>,  OptionQuery>;

	#[pallet::storage]
	#[pallet::getter(fn credential_registry)]
	pub type CredentialStore<T: Config> =
		StorageMap<_, Blake2_128Concat, T::Hash, VerifiableCredential<T::AccountId, T::Moment, T::Signature>,  OptionQuery>;
	
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
		SchemaCreated(T::Hash, VerifiableCredentialSchema<T::Moment, T::Signature>),
		// Event is emitted when a Credential item is created
		CredentialCreated(T::Hash, VerifiableCredential<T::AccountId, T::Moment, T::Signature>),
		// Event is emitted when an existing Schema item is updated
		SchemaUpdated(T::Hash, VerifiableCredentialSchema<T::Moment, T::Signature>),
		// Event is emitted when an existing credential item is updated
		CredentialUpdated(T::Hash, VerifiableCredential<T::AccountId, T::Moment, T::Signature>),
		// Event is emitted when an existing Schema item is deleted
		SchemaDeleted(T::Hash),
		// Event is emitted when an existing Credential item is deleted
		CredentialDeleted(T::Hash),

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
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Create a new schema item
		#[pallet::weight(0 + T::DbWeight::get().writes(1).ref_time())]
		pub fn create_schema(origin: OriginFor<T>,
			key: T::Hash, 
			name: String, 
			creator: Vec<u8>,
			mandatory_fields: Attribute,
			expiration_date: Option<T::Moment>,
			issuer_claims: Claim,
			subject_claims: Claim,
			credential_claims: Claim,
			signature: T::Signature,
			) -> DispatchResult {
			// Ensure that the caller of the function is signed
			let _ = ensure_signed_or_root(origin)?;
			let nonce = Self::get_nonce();
			let id = Uuid::new_v4();

			let verifiable_credential_schema = VerifiableCredentialSchema {
				id: id.as_u128(),
				name,
				creator,
				creation_date: T::Timestamp::now(),
				expiration_date,
				mandatory_fields,
				issuer_claims,
				subject_claims,
				credential_claims,
				signature,
				nonce,
			};
			let _ = Self::increment_nonce()?;
			// Ensure that the Schema does not already exist
			ensure!(!SchemaStore::<T>::contains_key(&key), "Schema already exists");
			// Save the Schema data in storage
			SchemaStore::<T>::insert(&key, &verifiable_credential_schema);
			// Emit an event to indicate that the Schema was created
			Self::deposit_event(Event::SchemaCreated(key, verifiable_credential_schema));
			Ok(())
		}

		/// Create a new credential item
		#[pallet::weight(0 + T::DbWeight::get().writes(1).ref_time())]
		pub fn create_credential(origin: OriginFor<T>,
			key: T::Hash, 
			context: Vec<u8>,
			schema: VerifiableCredentialSchema<T::Moment, T::Signature>,
			issuer: Option<T::AccountId>,
			expiration_date: Option<T::Moment>,
			subject: String,
			credential_holder: Vec<u8>,
			signature: T::Signature,
			) -> DispatchResult {

			// Ensure that the caller of the function is signed
			let _ = ensure_signed_or_root(origin)?;
			let id = Uuid::new_v4();
			let verifiable_credential = VerifiableCredential {
				id: id.as_u128(),
				context,
				schema,
				issuer,
				issuance_date: T::Timestamp::now(),
				expiration_date,
				subject,
				credential_holder,
				signature,
			};
			let _ = Self::increment_nonce()?;
			// Ensure that the Credential does not already exist
			ensure!(!CredentialStore::<T>::contains_key(&key), "Credential already exists");
			// Save the Credential data in storage
			CredentialStore::<T>::insert(&key, &verifiable_credential);
			// Emit an event to indicate that the Credential was created and stored
			Self::deposit_event(Event::CredentialCreated(key, verifiable_credential));
			Ok(())
		}

		// Function to update an existing schema
		#[pallet::weight(0 + T::DbWeight::get().writes(1).ref_time())]
		pub fn update_schema(origin: OriginFor<T>, key: T::Hash, new_data: VerifiableCredentialSchema<T::Moment, T::Signature>) -> DispatchResult {
			let _ = ensure_signed_or_root(origin)?;
			let schema_data = SchemaStore::<T>::get(key.clone()).ok_or(Error::<T>::UnknownSchema)?;
			ensure!(schema_data != new_data, Error::<T>::SchemaAlreadyExists);
			// Update the schema data
			SchemaStore::<T>::insert(&key, &new_data);
			let _ = Self::increment_nonce()?;
			Self::deposit_event(Event::SchemaUpdated(key, new_data));
			Ok(())
		}

		// Function to update an existing credential
		#[pallet::weight(0 + T::DbWeight::get().writes(1).ref_time())]
		pub fn update_credential(origin: OriginFor<T>, key: T::Hash, new_data: VerifiableCredential<T::AccountId, T::Moment, T::Signature>) -> DispatchResult {
			let _ = ensure_signed_or_root(origin)?;
			let credential_data = CredentialStore::<T>::get(key.clone()).ok_or(Error::<T>::UnknownCredential)?;
			ensure!(credential_data != new_data, Error::<T>::CredentialAlreadyExists);
			// Update the credential data
			CredentialStore::<T>::insert(&key, &new_data);
			let _ = Self::increment_nonce()?;
			Self::deposit_event(Event::CredentialUpdated(key, new_data));
			Ok(())
		}

		// Function to delete an existing schema
		#[pallet::weight(0 + T::DbWeight::get().reads_writes(1,1).ref_time())]
        pub fn delete_schema(origin: OriginFor<T>, key: T::Hash) -> DispatchResult {
            let _ = ensure_root(origin)?;
            ensure!(<SchemaStore<T>>::contains_key(&key), Error::<T>::UnknownSchema);
            <SchemaStore<T>>::remove(&key);
            Self::deposit_event(Event::SchemaDeleted(key));
            Ok(())
        }

		// Function to delete an existing credential
		#[pallet::weight(0 + T::DbWeight::get().reads_writes(1,1).ref_time())]
        pub fn delete_credential(origin: OriginFor<T>, key: T::Hash) -> DispatchResult {
            let _ = ensure_root(origin)?;
            ensure!(<CredentialStore<T>>::contains_key(&key), Error::<T>::UnknownCredential);
            <CredentialStore<T>>::remove(&key);
            Self::deposit_event(Event::CredentialDeleted(key));
            Ok(())
        }
	}

	impl<T: Config> Pallet<T>{
		fn increment_nonce() -> DispatchResult {
			<Nonce<T>>::try_mutate(|nonce| {
				let next = nonce.checked_add(1).ok_or("Overflow")?;
				*nonce = next;
				
				Ok(().into())
			})
		}
	}


}