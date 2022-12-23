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
use sp_core::H256;
	use crate::types::Registry;

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
		StorageMap<_, Blake2_128Concat, H256, Registry, OptionQuery>;


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
		Updated(Registry),
		// Event is emitted when an existing Registry item is deleted
		Deleted(H256),

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
		SchemaNotFound
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// An example dispatchable that takes a singles value as a parameter, writes the value to
		/// storage and emits an event. This function must be dispatched by a signed extrinsic.
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn create(origin: OriginFor<T>, key: H256, data: Registry) -> DispatchResult {
			// Ensure that the caller of the function is signed
			let _ = ensure_root(origin)?;
			// Ensure that the DID does not already exist
			ensure!(!RegistryStore::<T>::contains_key(&key), "Schema already exists");
			// Save the DID data in storage
			RegistryStore::<T>::insert(&key, &data);
			// Emit an event to indicate that the DID was created
			Self::deposit_event(Event::Created(data));
			Ok(())
		}
		// Function to update an existing schema
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn update(origin: OriginFor<T>, key: H256, old_data: Registry, new_data: Registry) -> DispatchResult {
			let _ = ensure_root(origin)?;
			let mut onchain_data = RegistryStore::<T>::get(key.clone()).ok_or(Error::<T>::UnknownSchema)?;
			ensure!(&old_data == &onchain_data, Error::<T>::SchemaNotFound);
			onchain_data = new_data.clone();
			// Save the updated schema vector in the storage
			RegistryStore::<T>::insert(&key, onchain_data);
			Self::deposit_event(Event::Updated(new_data));
			Ok(())
		}

		// Function to delete an existing schema
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1).ref_time())]
        pub fn delete(origin: OriginFor<T>, key: H256) -> DispatchResult {
            let _ = ensure_root(origin)?;
            ensure!(<RegistryStore<T>>::contains_key(&key), Error::<T>::SchemaNotFound);
            <RegistryStore<T>>::remove(&key);
            Self::deposit_event(Event::Deleted(key));
            Ok(())
        }
	}
}