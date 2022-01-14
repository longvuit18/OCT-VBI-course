#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/v3/runtime/frame>
pub use pallet::*;

#[cfg(test)]
mod tests;

#[cfg(test)]
mod mock;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;
	use scale_info::TypeInfo;
	use sp_std::vec::Vec;

	#[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo, Default)]
	#[scale_info(skip_type_params(T))]
	pub struct Dog<T> {
		pub dog_id: Vec<T>,
		pub name: Vec<T>,
		pub age: u32,
		pub color: Vec<T>,
	}

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
	}
	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/v3/runtime/events-and-errors
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters. [something, who]
		ClaimCreated(T::AccountId, Dog<u8>),
		ClaimRevoked(T::AccountId, Dog<u8>),
		ClaimTransferred(T::AccountId, Dog<u8>),
	}

	#[pallet::error]
	pub enum Error<T> {
		/// Error attempt to the claim that has been claimed.
		ProofAlreadyClaimed,
		// Error attempt to revoke the non-existent proof.
		NoSuchProof,
		/// Error attempt to revoke proof that has been claimed by another account.
		NotProofOwner,
		/// Error attempt to transfer proof to itself.
		NotTransferToItself,
	}
	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::storage]
	pub(super) type Proofs<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		Vec<u8>, // dog id
		(T::AccountId, T::BlockNumber, Dog<u8>), // luu struct dog vao day
		ValueQuery,
	>;

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(10_000)]
		pub fn create_claim(
			origin: OriginFor<T>,
			name: Vec<u8>,
			color: Vec<u8>,
			age: u32,
			dog_id: Vec<u8>,
		) -> DispatchResult {
			let sender = ensure_signed(origin)?;

			ensure!(!Proofs::<T>::contains_key(&dog_id), Error::<T>::ProofAlreadyClaimed);

			let current_block = <frame_system::Pallet<T>>::block_number();
			let dog = Dog::<u8> { name, color, age, dog_id };

			Proofs::<T>::insert(&dog.dog_id, (&sender, current_block, &dog));

			Self::deposit_event(Event::ClaimCreated(sender, dog));

			Ok(())
		}

		#[pallet::weight(10_000)]
		pub fn revoke_claim(origin: OriginFor<T>, dog_id: Vec<u8>) -> DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			// https://docs.substrate.io/v3/runtime/origins
			let sender = ensure_signed(origin)?;

			// Verify that the specified proof has been claimed.
			ensure!(Proofs::<T>::contains_key(&dog_id), Error::<T>::NoSuchProof);

			// Get owner of the claim.
			let (owner, _, dog) = Proofs::<T>::get(&dog_id);

			// Verify that sender of the current call is the claim owner.
			ensure!(sender == owner, Error::<T>::NotProofOwner);

			// Remove claim from storage.
			Proofs::<T>::remove(&dog_id);

			// Emit an event that the claim was erased.
			Self::deposit_event(Event::ClaimRevoked(sender, dog));
			Ok(())
		}

		#[pallet::weight(10_000)]
		pub fn transfer_claim(
			origin: OriginFor<T>,
			dog_id: Vec<u8>,
			address_to: T::AccountId,
		) -> DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			// https://docs.substrate.io/v3/runtime/origins
			let sender = ensure_signed(origin)?;
			let current_block = <frame_system::Pallet<T>>::block_number();
			// Verify that the specified proof has been claimed.
			ensure!(Proofs::<T>::contains_key(&dog_id), Error::<T>::NoSuchProof);
			ensure!(sender != address_to, Error::<T>::NotTransferToItself);

			// Get owner of the claim.
			let (_, _, dog) = Proofs::<T>::get(&dog_id);

			let new_dog = Dog::<u8> { name: dog.name, color: dog.color, age: dog.age, dog_id };

			// delete old identity
			Proofs::<T>::remove(&dog.dog_id);

			Proofs::<T>::insert(&dog.dog_id, (&address_to, current_block, &new_dog));

			Self::deposit_event(Event::ClaimTransferred(address_to, new_dog));
			Ok(())
		}
	}
}
