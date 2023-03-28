
#![cfg_attr(not(feature = "std"), no_std)]

// Re-export pallet items so that they can be accessed from the crate namespace.
pub use pallet::*;

//在存证结构体中添加一个字段，用于存储存证的接收账户地址。定义如下
#[derive(Encode, Decode, Default, Clone, PartialEq)]
pub struct Claim<AccountId, BlockNumber> {
    pub claim: Vec<u8>,
    pub claimant: AccountId,
    pub createdAt: BlockNumber,
    pub updatedAt: BlockNumber,
    pub supports: Vec<AccountId>,
    pub receiver: Option<AccountId>,
}

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

// Pallets use events to inform users when important changes are made.
// Event documentation should end with an array that provides descriptive names for parameters.
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
  /// The claim already exists.
  AlreadyClaimed,
  /// The claim does not exist, so it cannot be revoked.
  NoSuchClaim,
  /// The claim is owned by another account, so caller can't revoke it.
  NotClaimOwner,
}

#[pallet::storage]
pub(super) type Claims<T: Config> = StorageMap<_, Blake2_128Concat, T::Hash, (T::AccountId, T::BlockNumber)>;

// 添加一个新的方法 transfer_claim，用于转移存证。
#[pallet::call]
impl<T: Config> Pallet<T> {
    #[pallet::weight(0)]
    pub fn transfer_claim(origin: OriginFor<T>, claim: Vec<u8>, receiver: T::AccountId) -> DispatchResultWithPostInfo {
        let sender = ensure_signed(origin)?;

        ensure!(<Claims<T>>::contains_key(&sender), Error::<T>::NoSuchClaim);

        let mut stored_claim = <Claims<T>>::get(&sender);

        ensure!(stored_claim.claim == claim, Error::<T>::InvalidClaim);

        stored_claim.receiver = Some(receiver.clone());

        <Claims<T>>::insert(&sender, stored_claim.clone());

        Self::deposit_event(Event::ClaimTransferred(sender, receiver, stored_claim));

        Ok(().into())
    }
}

  #[pallet::weight(0)]
  #[pallet::call_index(2)]
  pub fn revoke_claim(origin: OriginFor<T>, claim: T::Hash) -> DispatchResult {
    // Check that the extrinsic was signed and get the signer.
    // This function will return an error if the extrinsic is not signed.
    let sender = ensure_signed(origin)?;

    // Get owner of the claim, if none return an error.
    let (owner, _) = Claims::<T>::get(&claim).ok_or(Error::<T>::NoSuchClaim)?;

    // Verify that sender of the current call is the claim owner.
    ensure!(sender == owner, Error::<T>::NotClaimOwner);

    // Remove claim from storage.
    Claims::<T>::remove(&claim);

    // Emit an event that the claim was erased.
    Self::deposit_event(Event::ClaimRevoked { who: sender, claim });
    Ok(())
  }
}
