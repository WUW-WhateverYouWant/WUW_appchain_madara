#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/reference/frame-pallets/>

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

pub mod weights;
use frame_support::pallet_prelude::DispatchResult;
use frame_system::pallet_prelude::OriginFor;
pub use weights::*;

sp_api::decl_runtime_apis! {

    pub trait TemplateTrait {
        fn do_something(origin: OriginFor<T>, something: u32) -> DispatchResult;
        fn cause_error(origin: OriginFor<T>) -> DispatchResult;
    }

   
}

sp_api::decl_runtime_apis!{
	pub trait BalancesTrait {
		fn transfer_allow_death(
			origin: OriginFor<T>,
			dest: <<T as Config>::Lookup as StaticLookup>::Source,
			value: T::Balance
		) -> DispatchResult;
		fn force_transfer(
			origin: OriginFor<T>,
			source: <<T as Config>::Lookup as StaticLookup>::Source,
			dest: <<T as Config>::Lookup as StaticLookup>::Source,
			value: T::Balance
		) -> DispatchResult;
        fn force_adjust_total_issuance<T: Config<I>, I: 'static>(
            direction: AdjustmentDirection,
            delta: T::Balance
        );
		fn transfer_keep_alive(
			origin: OriginFor<T>,
			dest: <<T as Config>::Lookup as StaticLookup>::Source,
			value: T::Balance
		) -> DispatchResult
		fn transfer_all(
			origin: OriginFor<T>,
			dest: <<T as Config>::Lookup as StaticLookup>::Source,
			keep_alive: bool
		) -> DispatchResult;
		fn force_unreserve(
			origin: OriginFor<T>,
			who: <<T as Config>::Lookup as StaticLookup>::Source,
			amount: T::Balance
		) -> DispatchResult;
		fn upgrade_accounts(
			origin: OriginFor<T>,
			who: Vec<T::AccountId>
		) -> DispatchResultWithPostInfo;
		fn force_set_balance(
			origin: OriginFor<T>,
			who: <<T as Config>::Lookup as StaticLookup>::Source,
			new_free: T::Balance
		) -> DispatchResult;
		fn force_adjust_total_issuance(
			origin: OriginFor<T>,
			direction: AdjustmentDirection,
			delta: T::Balance
		) -> DispatchResult;
    }

}
