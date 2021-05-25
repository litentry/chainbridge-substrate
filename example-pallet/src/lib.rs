// Ensure we're `no_std` when compiling for Wasm.
#![cfg_attr(not(feature = "std"), no_std)]

use chainbridge as bridge;
use frame_support::traits::{Currency, EnsureOrigin, ExistenceRequirement::AllowDeath,};
use frame_support::{decl_error, decl_module, dispatch::DispatchResult,};
use frame_system::{self as system,};
use sp_std::prelude::*;

mod mock;
mod tests;

type ResourceId = bridge::ResourceId;

type BalanceOf<T> =
    <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

pub trait Config: system::Config + bridge::Config {
    type BridgeOrigin: EnsureOrigin<Self::Origin, Success = Self::AccountId>;

    /// The currency mechanism.
    type Currency: Currency<Self::AccountId>;
}

decl_error! {
    pub enum Error for Module<T: Config>{
    }
}

decl_module! {
    pub struct Module<T: Config> for enum Call where origin: T::Origin {        
        /// Executes a simple currency transfer using the bridge account as the source
        #[weight = 195_000_000]
        pub fn transfer(origin, to: T::AccountId, amount: BalanceOf<T>, _r_id: ResourceId) -> DispatchResult {
            let source = T::BridgeOrigin::ensure_origin(origin)?;
            <T as Config>::Currency::transfer(&source, &to, amount.into(), AllowDeath)?;
            Ok(())
        }
    }
}
