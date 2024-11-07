pub mod asset_rate {
	use frame_support::traits::fungible::NativeOrWithId;
	use pallet_asset_rate::AssetKindFactory;
	use parachains_common::AssetIdForTrustBackedAssets;

	/// Provides a factory method for the [`VersionedLocatableAsset`].
	/// The location of the asset is determined as a Parachain with an ID equal to the passed seed.
	pub struct AssetRateArguments;

	pub type AssetKind = NativeOrWithId<AssetIdForTrustBackedAssets>;

	impl AssetKindFactory<AssetKind> for AssetRateArguments {
		fn create_asset_kind(seed: u32) -> AssetKind {
			AssetKind::WithId(seed)
		}
	}
}

pub mod treasury {
	use crate::AccountId;
	use core::marker::PhantomData;
	use frame_support::traits::{fungible::NativeOrWithId, Get};
	use pallet_treasury::ArgumentsFactory as TreasuryArgumentsFactory;
	use parachains_common::AssetIdForTrustBackedAssets;
	use sp_core::{ConstU32, ConstU8};

	pub type AssetKind = NativeOrWithId<AssetIdForTrustBackedAssets>;

	/// Provide factory methods for the [`VersionedLocatableAsset`] and the `Beneficiary` of the
	/// [`VersionedLocation`]. The location of the asset is determined as a Parachain with an
	/// ID equal to the passed seed.
	pub struct TreasuryArguments<Parents = ConstU8<0>, ParaId = ConstU32<0>>(
		PhantomData<(Parents, ParaId)>,
	);
	impl<Parents: Get<u8>, ParaId: Get<u32>> TreasuryArgumentsFactory<AssetKind, AccountId>
		for TreasuryArguments<Parents, ParaId>
	{
		fn create_asset_kind(seed: u32) -> AssetKind {
			AssetKind::WithId(seed)
		}
		fn create_beneficiary(seed: [u8; 32]) -> AccountId {
			AccountId::from(seed)
		}
	}
}
