// Ajuna Node
// Copyright (C) 2022 BlogaTech AG

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.

// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

pub mod treasury {
	use crate::AccountId;
	use core::marker::PhantomData;
	use frame_support::traits::{fungible::NativeOrWithId, Get};
	use pallet_treasury::ArgumentsFactory as TreasuryArgumentsFactory;
	use parachains_common::AssetIdForTrustBackedAssets;
	use sp_core::{ConstU32, ConstU8};

	pub type AssetKind = NativeOrWithId<AssetIdForTrustBackedAssets>;

	/// Provide factory methods for the [`NativeOrWithId<AssetIdForTrustBackedAssets>`] and
	/// the `Beneficiary` for [`AccountId`].
	///
	/// This is slightly different from Polkadot's implementation, which uses a
	/// [`VersionedLocatableAsset`] for the AssetId, as the asset lives on another chain.
	pub struct TreasuryArguments;

	impl TreasuryArgumentsFactory<AssetKind, AccountId> for TreasuryArguments {
		fn create_asset_kind(seed: u32) -> AssetKind {
			AssetKind::WithId(seed)
		}
		fn create_beneficiary(seed: [u8; 32]) -> AccountId {
			AccountId::from(seed)
		}
	}
}
