
#![cfg_attr(not(feature = "std"), no_std)]

// Re-export pallet items so that they can be accessed from the crate namespace.
pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
  use frame_support::pallet_prelude::*;
  use frame_system::pallet_prelude::*;

  #[pallet::pallet]
  #[pallet::generate_store(pub(super) trait Store)]
  pub struct Pallet<T>(_);

 /// Configure the pallet by specifying the parameters and types on which it depends.
#[pallet::config]
pub trait Config: frame_system::Config {
  /// Because this pallet emits events, it depends on the runtime's definition of an event.
  type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
}


#[pallet::event]
#[pallet::generate_deposit(pub(super) fn deposit_event)]
pub enum Event<T: Config> {
  /// Event emitted when a claim has been created.
  ClaimCreated { who: T::AccountId, claim: T::Hash },
  /// Event emitted when a claim is revoked by the owner.
  ClaimRevoked { who: T::AccountId, claim: T::Hash },
}

#[pallet::error]
pub enum Error<T> {

  AlreadyClaimed,

  NoSuchClaim,

  NotClaimOwner,
}

#[pallet::storage]
pub(super) type Claims<T: Config> = StorageMap<_, Blake2_128Concat, T::Hash, (T::AccountId, T::BlockNumber)>;

#[pallet::call]
impl<T: Config> Pallet<T> {
  #[pallet::weight(0)]
  #[pallet::call_index(1)]
  pub fn create_claim(origin: OriginFor<T>, claim: T::Hash) -> DispatchResult {

    let sender = ensure_signed(origin)?;

    // Verify that the specified claim has not already been stored.
    ensure!(!Claims::<T>::contains_key(&claim), Error::<T>::AlreadyClaimed);

    // Get the block number from the FRAME System pallet.
    let current_block = <frame_system::Pallet<T>>::block_number();

    // Store the claim with the sender and block number.
    Claims::<T>::insert(&claim, (&sender, current_block));

    // Emit an event that the claim was created.
    Self::deposit_event(Event::ClaimCreated { who: sender, claim });

    Ok(())
  }

  #[pallet::weight(0)]
  #[pallet::call_index(2)]
  pub fn revoke_claim(origin: OriginFor<T>, claim: T::Hash) -> DispatchResult {

    let sender = ensure_signed(origin)?;

    let (owner, _) = Claims::<T>::get(&claim).ok_or(Error::<T>::NoSuchClaim)?;

    ensure!(sender == owner, Error::<T>::NotClaimOwner);

    Claims::<T>::remove(&claim);

    Self::deposit_event(Event::ClaimRevoked { who: sender, claim });
    Ok(())
  }

  #[pallet::weight(0)]
  #[pallet::call_index(3)]
	pub fn transfer_claim(origin: OriginFor<T>, claim: T::Hash, receiver: T::AccountId) -> DispatchResult {

  	let sender = ensure_signed(origin)?;

  	let (owner, block_number) = Claims::<T>::get(&claim).ok_or(Error::<T>::NoSuchClaim)?;

  	ensure!(sender == owner, Error::<T>::NotClaimOwner);

 	Claims::<T>::remove(&claim);

   	Claims::<T>::insert(&claim, (&receiver, block_number));

  	Self::deposit_event(Event::ClaimRevoked { who: sender, claim });
  	Self::deposit_event(Event::ClaimCreated { who: receiver, claim });

 	 Ok(())
	}
}
}
