#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;
mod types;
pub mod schema;

pub use pallet::*;
#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
pub mod weights;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::{
		pallet_prelude::*, ensure,
		sp_runtime::traits::{Scale, IdentifyAccount, Member, Verify},
		traits::{Time, IsType},
	};
	use frame_system::pallet_prelude::*;
	use scale_info::{prelude::vec::Vec, StaticTypeInfo };
	use crate::weights::WeightInfo;
	use crate::types::*;
	use crate::schema::Schema;

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
		type WeightInfo: WeightInfo;
		
	}

	// The pallet's runtime schema storage.
	#[pallet::storage]
	#[pallet::getter(fn schema_registry)]
	pub type SchemaStore<T: Config> =
		StorageMap<_, Blake2_128Concat, T::Hash, (T::Signature, VerifiableCredentialSchema<T::AccountId, T::Moment>),  OptionQuery>;

	#[pallet::storage]
	#[pallet::getter(fn credential_registry)]
	pub type CredentialStore<T: Config> =
		StorageMap<_, Blake2_128Concat, T::Hash, (T::Signature, VerifiableCredential<T::AccountId, T::Moment>),  OptionQuery>;

	#[pallet::storage]
	#[pallet::getter(fn get_nonce)]
	pub(super) type Nonce<T: Config> = StorageValue< _, u64, ValueQuery>;
	
		
	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/main-docs/build/events-errors/
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters. [something, who]
		// Event is emitted when a Schema item is created
		SchemaCreated(T::Hash, Vec<u8>),
		// Event is emitted when a Credential item is created
		CredentialCreated(T::Hash, Vec<u8>),
		// Event is emitted when an existing Schema item is updated
		SchemaUpdated(T::Hash, Vec<u8>),
		// Event is emitted when an existing credential item is updated
		CredentialUpdated(T::Hash, Vec<u8>),
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
		/// Error emitted when signature is invalid
		SignatureVerifyError,
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Create a new schema item
		#[pallet::call_index(1)]
		#[pallet::weight(T::WeightInfo::create_schema())]
		pub fn create_schema(origin: OriginFor<T>, 
			name: Vec<u8>, 
			creator: T::AccountId, 
			public: bool,
			mandatory_fields: Vec<Attribute>,
			expiration_date: Option<T::Moment>,
			issuer_claims: Vec<Claim>,
			subject_claims: Vec<Claim>,
			credential_claims: Vec<Claim>,			
			metadata: Vec<u8>,
			signature: T::Signature,
			random_hash: T::Hash,
			nonce: u64, 
			) -> DispatchResult {
			// Ensure that the caller of the function is signed
			let _ = ensure_signed(origin)?;
			let creation_date = T::Timestamp::now();
			// Ensure that the Schema does not already exist
			ensure!(!SchemaStore::<T>::contains_key(&random_hash), "Schema already exists");
			// Create a new Schema item
			Self::create_verifiable_schema(&name, &creator, &public, creation_date, expiration_date, &mandatory_fields, &issuer_claims, &subject_claims, &credential_claims, &metadata, &signature, &nonce, &random_hash)
		}

		/// Create a new credential item
		#[pallet::call_index(2)]
		#[pallet::weight(T::WeightInfo::create_credential())]
		pub fn create_credential(origin: OriginFor<T>, 
			context: Vec<u8>,
			schema: Vec<u8>,
			issuer: T::AccountId,
			expiration_date: Option<T::Moment>,
			subject: Subject,
			credential_holder: Vec<u8>,
			signature: T::Signature,
			nonce: u64,
			random_hash: T::Hash,
			) -> DispatchResult {

			// Ensure that the caller of the function is signed
			let _ = ensure_signed(origin)?;
			let issuance_date = Some(T::Timestamp::now());
			// Ensure that the Credential does not already exist
			ensure!(!CredentialStore::<T>::contains_key(&random_hash), "Credential already exists");
			Self::create_verifiable_credential(&context, &schema, &issuer, issuance_date, expiration_date, &subject, &credential_holder, &signature, &nonce, &random_hash)
		}

		// Function to update an existing schema
		#[pallet::call_index(3)]
		#[pallet::weight(T::WeightInfo::update_schema())]
		pub fn update_schema(origin: OriginFor<T>, old_schema_key: T::Hash, new_data: (T::Signature, VerifiableCredentialSchema<T::AccountId, T::Moment>)) -> DispatchResult {
			let _ = ensure_signed_or_root(origin)?;
			let schema_data = SchemaStore::<T>::get(&old_schema_key).ok_or(Error::<T>::UnknownSchema)?;
			ensure!(schema_data != new_data, Error::<T>::SchemaAlreadyExists);
			// Update the schema data
			Self::update_verifiable_schema(&old_schema_key, &new_data)
		}

		// Function to update an existing credential
		#[pallet::call_index(4)]
		#[pallet::weight(T::WeightInfo::update_credential())]
		pub fn update_credential(origin: OriginFor<T>, old_credential_sig: T::Hash, new_data: (T::Signature, VerifiableCredential<T::AccountId, T::Moment>)) -> DispatchResult {
			let _ = ensure_signed_or_root(origin)?;
			let credential_data = CredentialStore::<T>::get(&old_credential_sig).ok_or(Error::<T>::UnknownCredential)?;
			ensure!(credential_data != new_data, Error::<T>::CredentialAlreadyExists);
			// Update the credential data
			Self::update_verifiable_credential(&old_credential_sig, &new_data)
		}

		// Function to delete an existing schema
		#[pallet::call_index(5)]
		#[pallet::weight(T::WeightInfo::delete_schema())]
        pub fn delete_schema(origin: OriginFor<T>, key: T::Hash) -> DispatchResult {
            let _ = ensure_signed(origin)?;
            ensure!(<SchemaStore<T>>::contains_key(&key), Error::<T>::UnknownSchema);
            Self::delete_verifiable_schema(&key)
        }

		// Function to delete an existing credential
		#[pallet::call_index(6)]
		#[pallet::weight(T::WeightInfo::delete_credential())]
        pub fn delete_credential(origin: OriginFor<T>, key: T::Hash) -> DispatchResult {
            let _ = ensure_signed(origin)?;
            ensure!(<CredentialStore<T>>::contains_key(&key), Error::<T>::UnknownCredential);
            Self::delete_verifiable_credential(&key)
        }

	}
	impl<T: Config>
	Schema<T::AccountId, T::Moment, T::Signature, T::Hash>
	for Pallet<T>
	{
		// Function to create a new schema
		fn create_verifiable_schema(
			name: &Vec<u8>, 
			creator: &T::AccountId,
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
			random_hash: &T::Hash,
		) -> DispatchResult{
			let verifiable_credential_schema = VerifiableCredentialSchema {
				name: name.to_owned(),
				creator: creator.to_owned(),
				public: public.to_owned(),
				creation_date,
				expiration_date,
				mandatory_fields: mandatory_fields.to_owned(),
				issuer_claims: issuer_claims.to_owned(),
				subject_claims: subject_claims.to_owned(),
				credential_claims: credential_claims.to_owned(),
				metadata: metadata.to_owned(),
				nonce: nonce.to_owned(),
			};
			let binding = verifiable_credential_schema.encode();
   			let vc_bytes = binding.as_slice();
			let signer = &verifiable_credential_schema.creator;
			Self::is_valid_signer(vc_bytes, signature, signer)?;
			// Save the Schema data in storage
			SchemaStore::<T>::insert(random_hash, (&signature, &verifiable_credential_schema));
			// Emit an event to indicate that the Schema was created
			Self::deposit_event(Event::SchemaCreated(random_hash.to_owned(), verifiable_credential_schema.encode()));
			Ok(())
		}
		// create a new credential
		fn create_verifiable_credential(
			context: &Vec<u8>,
			schema: &Vec<u8>,
			issuer: &T::AccountId,
			issuance_date: Option<T::Moment>,
			expiration_date: Option<T::Moment>,
			subject: &Subject,
			credential_holder: &Vec<u8>,
			signature: &T::Signature,
			nonce: &u64,
			random_hash: &T::Hash,
		) -> DispatchResult{
			let verifiable_credential = VerifiableCredential {
				context: context.to_owned(),
				schema: schema.to_owned(),
				issuer: issuer.to_owned(),
				issuance_date,
				expiration_date,
				subject: subject.to_owned(),
				credential_holder: credential_holder.to_owned(),
				nonce: nonce.to_owned(),
			};
			let binding = verifiable_credential.encode();
   			let vc_bytes = binding.as_slice();
			let signer = &verifiable_credential.issuer;
			Self::is_valid_signer(vc_bytes, signature, signer)?;
			// Save the Schema data in storage
			CredentialStore::<T>::insert(&random_hash, (&signature, &verifiable_credential));
			// Emit an event to indicate that the Credential was created and stored
			Self::deposit_event(Event::CredentialCreated(random_hash.to_owned(), verifiable_credential.encode()));
			Ok(())
		}
		// update a schema
		fn update_verifiable_schema(
			old_schema_key: &T::Hash, 
			new_data: &(T::Signature, VerifiableCredentialSchema<T::AccountId, T::Moment>)) -> DispatchResult{
			// Update the schema data
			SchemaStore::<T>::insert(old_schema_key, new_data);
			Self::deposit_event(Event::SchemaUpdated(old_schema_key.to_owned(), new_data.encode()));
			Ok(())
		}
		// update a credential
		fn update_verifiable_credential(old_credential_sig: &T::Hash, new_data: &(T::Signature, VerifiableCredential<T::AccountId, T::Moment>)) -> DispatchResult{
			// Update the credential data
			CredentialStore::<T>::insert(old_credential_sig, new_data);
			Self::deposit_event(Event::CredentialUpdated(old_credential_sig.to_owned(), new_data.encode()));
			Ok(())
		}
		// delete schema
		fn delete_verifiable_schema(key: &T::Hash) -> DispatchResult{
            <SchemaStore<T>>::remove(key);
            Self::deposit_event(Event::SchemaDeleted(key.to_owned()));
			Ok(())
		}
		// delete a credential
		fn delete_verifiable_credential(key: &T::Hash) -> DispatchResult{
            <CredentialStore<T>>::remove(key);
            Self::deposit_event(Event::CredentialDeleted(key.to_owned()));
			Ok(())
		}

		fn is_valid_signer(data: &[u8], sig: &T::Signature, from: &T::AccountId) -> DispatchResult{
			ensure!(sig.verify(data, from), <Error<T>>::SignatureVerifyError);
			Ok(())
		}
	}

}