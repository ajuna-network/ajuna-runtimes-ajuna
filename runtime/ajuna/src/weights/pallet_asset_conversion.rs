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

//! Autogenerated weights for `pallet_asset_conversion`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 39.0.0
//! DATE: 2024-08-17, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `DESKTOP-0F6V7QQ`, CPU: `Intel(R) Core(TM) i7-10875H CPU @ 2.30GHz`
//! WASM-EXECUTION: `Compiled`, CHAIN: `Some("ajuna-rococo-local")`, DB CACHE: 1024

// Executed Command:
// ./target/release/ajuna-node
// benchmark
// pallet
// --chain=ajuna-rococo-local
// --steps=50
// --repeat=20
// --pallet=pallet_asset_conversion
// --extrinsic=*
// --wasm-execution=compiled
// --heap-pages=4096
// --header=./HEADER-AGPL
// --output=./runtime/ajuna/src/weights/pallet_asset_conversion.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;

/// Weight functions for `pallet_asset_conversion`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_asset_conversion::WeightInfo for WeightInfo<T> {
	/// Storage: `AssetConversion::Pools` (r:1 w:1)
	/// Proof: `AssetConversion::Pools` (`max_values`: None, `max_size`: Some(30), added: 2505, mode: `MaxEncodedLen`)
	/// Storage: `Assets::Asset` (r:2 w:0)
	/// Proof: `Assets::Asset` (`max_values`: None, `max_size`: Some(210), added: 2685, mode: `MaxEncodedLen`)
	/// Storage: `AssetConversion::NextPoolAssetId` (r:1 w:1)
	/// Proof: `AssetConversion::NextPoolAssetId` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// Storage: `PoolAssets::Asset` (r:1 w:1)
	/// Proof: `PoolAssets::Asset` (`max_values`: None, `max_size`: Some(210), added: 2685, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:0)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `PoolAssets::Account` (r:1 w:1)
	/// Proof: `PoolAssets::Account` (`max_values`: None, `max_size`: Some(134), added: 2609, mode: `MaxEncodedLen`)
	fn create_pool() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `658`
		//  Estimated: `6360`
		// Minimum execution time: 77_500_000 picoseconds.
		Weight::from_parts(78_400_000, 0)
			.saturating_add(Weight::from_parts(0, 6360))
			.saturating_add(T::DbWeight::get().reads(7))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	/// Storage: `AssetConversion::Pools` (r:1 w:0)
	/// Proof: `AssetConversion::Pools` (`max_values`: None, `max_size`: Some(30), added: 2505, mode: `MaxEncodedLen`)
	/// Storage: `Assets::Asset` (r:2 w:2)
	/// Proof: `Assets::Asset` (`max_values`: None, `max_size`: Some(210), added: 2685, mode: `MaxEncodedLen`)
	/// Storage: `Assets::Account` (r:4 w:4)
	/// Proof: `Assets::Account` (`max_values`: None, `max_size`: Some(134), added: 2609, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `PoolAssets::Asset` (r:1 w:1)
	/// Proof: `PoolAssets::Asset` (`max_values`: None, `max_size`: Some(210), added: 2685, mode: `MaxEncodedLen`)
	/// Storage: `PoolAssets::Account` (r:2 w:2)
	/// Proof: `PoolAssets::Account` (`max_values`: None, `max_size`: Some(134), added: 2609, mode: `MaxEncodedLen`)
	fn add_liquidity() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1307`
		//  Estimated: `11426`
		// Minimum execution time: 171_600_000 picoseconds.
		Weight::from_parts(245_800_000, 0)
			.saturating_add(Weight::from_parts(0, 11426))
			.saturating_add(T::DbWeight::get().reads(11))
			.saturating_add(T::DbWeight::get().writes(10))
	}
	/// Storage: `AssetConversion::Pools` (r:1 w:0)
	/// Proof: `AssetConversion::Pools` (`max_values`: None, `max_size`: Some(30), added: 2505, mode: `MaxEncodedLen`)
	/// Storage: `Assets::Asset` (r:2 w:2)
	/// Proof: `Assets::Asset` (`max_values`: None, `max_size`: Some(210), added: 2685, mode: `MaxEncodedLen`)
	/// Storage: `Assets::Account` (r:4 w:4)
	/// Proof: `Assets::Account` (`max_values`: None, `max_size`: Some(134), added: 2609, mode: `MaxEncodedLen`)
	/// Storage: `PoolAssets::Asset` (r:1 w:1)
	/// Proof: `PoolAssets::Asset` (`max_values`: None, `max_size`: Some(210), added: 2685, mode: `MaxEncodedLen`)
	/// Storage: `PoolAssets::Account` (r:1 w:1)
	/// Proof: `PoolAssets::Account` (`max_values`: None, `max_size`: Some(134), added: 2609, mode: `MaxEncodedLen`)
	fn remove_liquidity() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1411`
		//  Estimated: `11426`
		// Minimum execution time: 149_400_000 picoseconds.
		Weight::from_parts(153_500_000, 0)
			.saturating_add(Weight::from_parts(0, 11426))
			.saturating_add(T::DbWeight::get().reads(9))
			.saturating_add(T::DbWeight::get().writes(8))
	}
	/// Storage: `Assets::Asset` (r:3 w:3)
	/// Proof: `Assets::Asset` (`max_values`: None, `max_size`: Some(210), added: 2685, mode: `MaxEncodedLen`)
	/// Storage: `Assets::Account` (r:6 w:6)
	/// Proof: `Assets::Account` (`max_values`: None, `max_size`: Some(134), added: 2609, mode: `MaxEncodedLen`)
	/// The range of component `n` is `[2, 3]`.
	fn swap_exact_tokens_for_tokens(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `22 + n * (419 ±0)`
		//  Estimated: `990 + n * (5218 ±0)`
		// Minimum execution time: 93_900_000 picoseconds.
		Weight::from_parts(101_100_000, 0)
			.saturating_add(Weight::from_parts(0, 990))
			// Standard Error: 165_162
			.saturating_add(Weight::from_parts(1_696_585, 0).saturating_mul(n.into()))
			.saturating_add(T::DbWeight::get().reads((3_u64).saturating_mul(n.into())))
			.saturating_add(T::DbWeight::get().writes((3_u64).saturating_mul(n.into())))
			.saturating_add(Weight::from_parts(0, 5218).saturating_mul(n.into()))
	}
	/// Storage: `Assets::Asset` (r:3 w:3)
	/// Proof: `Assets::Asset` (`max_values`: None, `max_size`: Some(210), added: 2685, mode: `MaxEncodedLen`)
	/// Storage: `Assets::Account` (r:6 w:6)
	/// Proof: `Assets::Account` (`max_values`: None, `max_size`: Some(134), added: 2609, mode: `MaxEncodedLen`)
	/// The range of component `n` is `[2, 3]`.
	fn swap_tokens_for_exact_tokens(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `22 + n * (419 ±0)`
		//  Estimated: `990 + n * (5218 ±0)`
		// Minimum execution time: 96_500_000 picoseconds.
		Weight::from_parts(101_900_000, 0)
			.saturating_add(Weight::from_parts(0, 990))
			// Standard Error: 157_483
			.saturating_add(Weight::from_parts(1_460_975, 0).saturating_mul(n.into()))
			.saturating_add(T::DbWeight::get().reads((3_u64).saturating_mul(n.into())))
			.saturating_add(T::DbWeight::get().writes((3_u64).saturating_mul(n.into())))
			.saturating_add(Weight::from_parts(0, 5218).saturating_mul(n.into()))
	}
	/// Storage: `AssetConversion::Pools` (r:1 w:0)
	/// Proof: `AssetConversion::Pools` (`max_values`: None, `max_size`: Some(30), added: 2505, mode: `MaxEncodedLen`)
	/// Storage: `Assets::Asset` (r:2 w:2)
	/// Proof: `Assets::Asset` (`max_values`: None, `max_size`: Some(210), added: 2685, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:0)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `Assets::Account` (r:2 w:2)
	/// Proof: `Assets::Account` (`max_values`: None, `max_size`: Some(134), added: 2609, mode: `MaxEncodedLen`)
	/// Storage: `PoolAssets::Asset` (r:1 w:1)
	/// Proof: `PoolAssets::Asset` (`max_values`: None, `max_size`: Some(210), added: 2685, mode: `MaxEncodedLen`)
	/// Storage: `PoolAssets::Account` (r:1 w:1)
	/// Proof: `PoolAssets::Account` (`max_values`: None, `max_size`: Some(134), added: 2609, mode: `MaxEncodedLen`)
	/// The range of component `n` is `[0, 3]`.
	fn touch(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1371`
		//  Estimated: `6360`
		// Minimum execution time: 54_900_000 picoseconds.
		Weight::from_parts(58_442_563, 0)
			.saturating_add(Weight::from_parts(0, 6360))
			// Standard Error: 128_513
			.saturating_add(Weight::from_parts(24_095_722, 0).saturating_mul(n.into()))
			.saturating_add(T::DbWeight::get().reads(8))
			.saturating_add(T::DbWeight::get().writes((2_u64).saturating_mul(n.into())))
	}
}
