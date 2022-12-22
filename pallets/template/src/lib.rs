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

	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;
	use crate::types::DIDData;
	use sp_core::H256;
	use scale_info::prelude::vec::Vec;

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
	// https://docs.substrate.io/main-docs/build/runtime-storage/
	#[pallet::storage]
	#[pallet::getter(fn something)]
	// Learn more about declaring storage items:
	// https://docs.substrate.io/main-docs/build/runtime-storage/#declaring-storage-items
	pub type Something<T> = StorageValue<_, u32>;

	#[pallet::storage]
	#[pallet::getter(fn store_did)]
	pub type DIDs<T: Config> =
		StorageMap<_, Blake2_128Concat, Vec<u8>, DIDData, OptionQuery>;

	#[pallet::storage]
	#[pallet::getter(fn did_to_key)]
	pub type DidToKey<T: Config> =
		StorageMap<_, Blake2_128Concat, H256, Vec<u8>, OptionQuery>;

	#[pallet::storage]
	#[pallet::getter(fn did_to_owner)]
	pub type DidToOwner<T: Config> =
		StorageMap<_, Blake2_128Concat, H256, T::AccountId, OptionQuery>;

	#[pallet::storage]
	#[pallet::getter(fn did_to_metadata)]
	pub type DidToMetadata<T: Config> =
		StorageMap<_, Blake2_128Concat, H256, Vec<u8>, OptionQuery>;

	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/main-docs/build/events-errors/
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters. [something, who]
		SomethingStored {
			something: u32,
			who: T::AccountId,
		},
		RuntimeUpgraded(T::AccountId),
		Created(T::AccountId, Vec<u8>),
		// Event is emitted when an existing DID is updated
		Updated(T::AccountId, Vec<u8>),
		// Event is emitted when an existing DID is deleted
		Deleted(T::AccountId, Vec<u8>),
		DidCreated(T::AccountId, H256),
        DidUpdated(T::AccountId, H256),
        DidDeleted(T::AccountId, H256),
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Error names should be descriptive.
		NoneValue,
		/// Errors should have helpful documentation associated with them.
		StorageOverflow,
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// An example dispatchable that takes a singles value as a parameter, writes the value to
		/// storage and emits an event. This function must be dispatched by a signed extrinsic.
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn create(origin: OriginFor<T>, did: Vec<u8>, data: DIDData) -> DispatchResult {
			// Ensure that the caller of the function is signed
			let sender = ensure_signed(origin)?;
			// Ensure that the DID does not already exist
			ensure!(!DIDs::<T>::contains_key(&did), "DID already exists");
			// Save the DID data in storage
			DIDs::<T>::insert(&did, &data);
			// Emit an event to indicate that the DID was created
			Self::deposit_event(Event::Created(sender, did));
			Ok(())
		}
		// Function to update an existing DID
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn update(origin: OriginFor<T>, did: Vec<u8>, data: DIDData) -> DispatchResult {
			// Ensure that the caller of the function is signed
			let sender = ensure_signed(origin)?;
			// Ensure that the DID already exists
			ensure!(DIDs::<T>::contains_key(&did), "DID does not exist");
			// Update the DID data in storage
			DIDs::<T>::insert(&did, &data);
			// Emit an event to indicate that the DID was updated
			Self::deposit_event(Event::Updated(sender, did));
			Ok(())
		}

		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1).ref_time())]
        pub fn create_did(origin: OriginFor<T>, key: Vec<u8>, metadata: Vec<u8>) -> DispatchResult {
            let sender = ensure_signed(origin)?;
            let did = H256::from_slice(&metadata);
            <DidToKey<T>>::insert(&did, &key);
            <DidToOwner<T>>::insert(&did, &sender);
            <DidToMetadata<T>>::insert(&did, &metadata);
            Self::deposit_event(Event::DidCreated(sender, did));
            Ok(())
        }

		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1).ref_time())]
        pub fn update_did(origin: OriginFor<T>, did: H256, key: Vec<u8>, metadata: Vec<u8>) -> DispatchResult {
            let sender = ensure_signed(origin)?;
            ensure!(<DidToOwner<T>>::contains_key(&did), "Did does not exist");
            <DidToKey<T>>::insert(&did, &key);
            <DidToMetadata<T>>::insert(&did, &metadata);
            Self::deposit_event(Event::DidUpdated(sender, did));
            Ok(())
        }

		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1).ref_time())]
        pub fn delete_did(origin: OriginFor<T>, did: H256) -> DispatchResult {
            let sender = ensure_signed(origin)?;
            ensure!(<DidToOwner<T>>::contains_key(&did), "Did does not exist");
            <DidToKey<T>>::remove(&did);
            <DidToOwner<T>>::remove(&did);
            <DidToMetadata<T>>::remove(&did);
            Self::deposit_event(Event::DidDeleted(sender, did));
            Ok(())
        }
	}
}
