#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;
mod types;

pub use pallet::*;
#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
use sp_core::crypto::KeyTypeId;
use sp_io::offchain_index;
use sp_runtime::{
	offchain as rt_offchain,
	offchain::{
		storage::StorageValueRef,
		storage_lock::{BlockAndTime, StorageLock},
	},
	transaction_validity::{
		InvalidTransaction, TransactionSource, TransactionValidity, ValidTransaction,
	},
	RuntimeDebug,
};
use sp_std::{collections::vec_deque::VecDeque, prelude::*, str};

use serde::{Deserialize, Deserializer};

pub const KEY_TYPE: KeyTypeId = KeyTypeId(*b"demo");
const NUM_VEC_LEN: usize = 10;
/// The type to sign and send transactions.
const UNSIGNED_TXS_PRIORITY: u64 = 100;

// We are fetching information from the github public API about organization`substrate-developer-hub`.
const HTTP_REMOTE_REQUEST: &str = "https://api.github.com/orgs/substrate-developer-hub";
const HTTP_HEADER_USER_AGENT: &str = "jimmychu0807";

const FETCH_TIMEOUT_PERIOD: u64 = 3000; // in milli-seconds
const LOCK_TIMEOUT_EXPIRATION: u64 = FETCH_TIMEOUT_PERIOD + 1000; // in milli-seconds
const LOCK_BLOCK_EXPIRATION: u32 = 3; // in block number

const ONCHAIN_TX_KEY: &[u8] = b"ocw-demo::storage::tx";
pub mod crypto {
	use crate::KEY_TYPE;
	use sp_core::sr25519::Signature as Sr25519Signature;
	use sp_runtime::app_crypto::{app_crypto, sr25519};
	use sp_runtime::{traits::Verify, MultiSignature, MultiSigner};

	app_crypto!(sr25519, KEY_TYPE);

	pub struct TestAuthId;
	// implemented for ocw-runtime
	impl frame_system::offchain::AppCrypto<MultiSigner, MultiSignature> for TestAuthId {
		type RuntimeAppPublic = Public;
		type GenericSignature = sp_core::sr25519::Signature;
		type GenericPublic = sp_core::sr25519::Public;
	}

	// implemented for mock runtime in test
	impl frame_system::offchain::AppCrypto<<Sr25519Signature as Verify>::Signer, Sr25519Signature>
		for TestAuthId
	{
		type RuntimeAppPublic = Public;
		type GenericSignature = sp_core::sr25519::Signature;
		type GenericPublic = sp_core::sr25519::Public;
	}
}


#[frame_support::pallet]
pub mod pallet {

	use crate::types::Registry;
	use frame_support::{ensure, pallet_prelude::*};
	use frame_system::pallet_prelude::*;
	use parity_scale_codec::{Decode, Encode};

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
	Data get(origin: OriginFor<T>): VecDeque<u64>;
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
		Stored(PrivateDataStorage),
		// Event is emitted when an existing Registry item is updated
		Updated(T::Hash, PrivateDataStorage),
		// Event is emitted when an existing Registry item is deleted
		Deleted(T::Hash, PrivateDataStorage),
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
	// Error returned when not sure which ocw function to executed
		UnknownOffchainMux,

		// Error returned when making signed transactions in off-chain worker
		NoLocalAcctForSigning,
		OffchainSignedTxError,

		// Error returned when making unsigned transactions in off-chain worker
		OffchainUnsignedTxError,

		// Error returned when making unsigned transactions with signed payloads in off-chain worker
		OffchainUnsignedTxSignedPayloadError,

		// Error returned when fetching github info
		HttpFetchingError,
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// An example dispatchable that takes a singles value as a parameter, writes the value to
		/// storage and emits an event. This function must be dispatched by a signed extrinsic.
		#[pallet::weight(0 + T::DbWeight::get().writes(1).ref_time())]
	pub fn submit_data_storage_signed(origin, data_storage: u64) -> DispatchResult {
			let who = ensure_signed(origin)?;
			debug::info!("submit_data_storage_signed: ({}, {:?})", data_storage, who);
			Self::append_or_replace_data_storage(data_storage);

			// Off-chain indexing allowing on-chain extrinsics to write to off-chain storage predictably
			// so it can be read in off-chain worker context. As off-chain indexing is called in on-chain
			// context, if it is agreed upon by the blockchain consensus mechanism, then it is expected
			// to run predicably by all nodes in the network.
			//
			// From an on-chain perspective, this is write-only and cannot be read back.
			//
			// The value is written in byte form, so we need to encode/decode it when writting/reading
			// a data_storage to/from this memory space.
			//
			// Ref: https://substrate.dev/rustdocs/v3.0.0/sp_io/offchain_index/index.html
			let key = Self::derived_key(frame_system::Module::<T>::block_data_storage());
			let data = IndexingData(b"submit_data_storage_signed".to_vec(), data_storage);
			offchain_index::set(&key, &data.encode());

			Self::deposit_event(RawEvent::Newdata_storage(Some(who), data_storage));
			Ok(())
		}

		// Function to update an existing schema
		#[pallet::weight(0 + T::DbWeight::get().writes(1).ref_time())]
		pub fn submit_data_storage_unsigned(origin, data_storage: u64) -> DispatchResult {
			let _ = ensure_none(origin)?;
			debug::info!("submit_data_storage_unsigned: {}", data_storage);
			Self::append_or_replace_data_storage(data_storage);

			// Off-chain indexing write
			let key = Self::derived_key(frame_system::Module::<T>::block_data_storage());
			let data = IndexingData(b"submit_data_storage_unsigned".to_vec(), data_storage);
			offchain_index::set(&key, &data.encode());

			Self::deposit_event(RawEvent::Newdata_storage(None, data_storage));
			Ok(())
		}
		// Function to delete an existing schema
		#[pallet::weight(0 + T::DbWeight::get().reads_writes(1,1).ref_time())]
		pub fn submit_data_storage_unsigned_with_signed_payload(origin, payload: Payload<T::Public>,
			_signature: T::Signature) -> DispatchResult
		{
			let _ = ensure_none(origin)?;
			// we don't need to verify the signature here because it has been verified in
			//   `validate_unsigned` function when sending out the unsigned tx.
			let Payload { data_storage, public } = payload;
			debug::info!("submit_data_storage_unsigned_with_signed_payload: ({}, {:?})", data_storage, public);
			Self::append_or_replace_data_storage(data_storage);

			// Off-chain indexing write
			let key = Self::derived_key(frame_system::Module::<T>::block_number());
			let data = IndexingData(b"submit_data_storage_unsigned_with_signed_payload".to_vec(), number);
			offchain_index::set(&key, &data.encode());

			Self::deposit_event(RawEvent::NewNumber(None, number));
			Ok(())
		}
		fn offchain_worker(block_number: T::BlockNumber) {
			debug::info!("Entering off-chain worker");

			// Here we are showcasing various techniques used when running off-chain workers (ocw)
			// 1. Sending signed transaction from ocw
			// 2. Sending unsigned transaction from ocw
			// 3. Sending unsigned transactions with signed payloads from ocw
			// 4. Fetching JSON via http requests in ocw
			const TRANSACTION_TYPES: usize = 4;
			let result = match block_number.try_into().unwrap_or(0) % TRANSACTION_TYPES	{
				1 => Self::offchain_signed_tx(block_number),
				2 => Self::offchain_unsigned_tx(block_number),
				3 => Self::offchain_unsigned_tx_signed_payload(block_number),
				0 => Self::fetch_github_info(),
				_ => Err(Error::<T>::UnknownOffchainMux),
			};

			if let Err(e) = result {
				debug::error!("offchain_worker error: {:?}", e);
			}

			// Reading back the off-chain indexing value. It is exactly the same as reading from
			// ocw local storage.
			let key = Self::derived_key(block_number);
			let oci_mem = StorageValueRef::persistent(&key);

			if let Some(Some(data)) = oci_mem.get::<IndexingData>() {
				debug::info!("off-chain indexing data: {:?}, {:?}",
					str::from_utf8(&data.0).unwrap_or("error"), data.1);
			} else {
				debug::info!("no off-chain indexing data retrieved.");
			}
		}
	}
}
