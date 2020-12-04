#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{ensure, decl_module, decl_storage, decl_event, decl_error, dispatch, traits::Get};
use frame_system::ensure_signed;
use sp_std::prelude::*;
use sp_runtime::DispatchResult;
use sp_io::stark;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

/// Config pallet parameters and types
pub trait Trait: frame_system::Trait {
    type Event: From<Event<Self>> + Into<<Self as frame_system::Trait>::Event>;
}

// Currently the pallet is only used for proof verification. A place holder storage is used.
decl_storage! {
	trait Store for Module<T: Trait> as DistaffVM {
		Something get(fn something): Option<u32>;
	}
}

// Event for a successful STARK proof verification
decl_event!(
	pub enum Event<T> where <T as frame_system::Trait>::AccountId {
		Verified(AccountId),
	}
);

// Error message in case of a failed STARK proof verification
decl_error! {
	pub enum Error for Module<T: Trait> {
		// STARK proof verification failed
		StarkVerifyFailed,
	}
}

// The Distaff VM module
decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		type Error = Error<T>;

		fn deposit_event() = default;

		#[weight = 10_000 + T::DbWeight::get().reads_writes(1,1)]
		/// The STARK proof verification function
		pub fn verify(
			origin,
			// 4 input data fields are needed for a successful proof verification
			program_hash: [u8; 32],
			public_inputs: Box<Vec<u128>>,
			outputs: Box<Vec<u128>>,
			proof: Vec<u8>) {
			let who = ensure_signed(origin)?;
			Self::stark_verify(&program_hash, &public_inputs, &outputs, &proof)?;
			Self::deposit_event(RawEvent::Verified(who));
		}
	}
}

// STARK proof verification via the STARK primitive module
impl<T: Trait> Module<T> {
    fn stark_verify(
        program_hash: &[u8; 32],
        public_inputs: &[u128],
        outputs: &[u128],
        proof: &[u8]
    ) -> DispatchResult {
        let r = stark::st_verify(program_hash, public_inputs, outputs, proof);
        ensure!(r.is_some(), Error::<T>::StarkVerifyFailed);
        Ok(())
    }
}
