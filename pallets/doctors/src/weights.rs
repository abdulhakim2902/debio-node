//! Autogenerated weights for doctors
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-11-14, STEPS: `20`, REPEAT: 10, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// ./target/release/debio
// benchmark
// --chain=dev
// --execution=wasm
// --wasm-execution=compiled
// --pallet=doctors
// --extrinsic=*
// --steps=20
// --repeat=10
// --heap-pages=4096
// --output=./pallets/doctors/src/weights.rs
// --template=./.maintain/pallet-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for doctors.
pub trait WeightInfo { 
	fn register_doctor() -> Weight; 
	fn update_doctor() -> Weight; 
	fn deregister_doctor() -> Weight; 
}

/// Weights for doctors using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>); 
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> { 
	// Storage: Doctors Doctors (r:1 w:1) 
	// Storage: Doctors DoctorsByCountryRegionCity (r:1 w:1) 
	// Storage: Doctors DoctorCount (r:1 w:1) 
	// Storage: Doctors DoctorCountByCountryRegionCity (r:1 w:1) 
	// Storage: UserProfile ProfileRolesByAccountId (r:1 w:1) 
	fn register_doctor() -> Weight { 
		91_000_000_u64 
			.saturating_add(T::DbWeight::get().reads(5_u64)) 
			.saturating_add(T::DbWeight::get().writes(5_u64)) 
	}
	// Storage: Doctors Doctors (r:1 w:1) 
	// Storage: Doctors DoctorsByCountryRegionCity (r:2 w:2) 
	// Storage: Doctors DoctorCountByCountryRegionCity (r:2 w:2) 
	fn update_doctor() -> Weight { 
		124_000_000_u64 
			.saturating_add(T::DbWeight::get().reads(5_u64)) 
			.saturating_add(T::DbWeight::get().writes(5_u64)) 
	}
	// Storage: Doctors Doctors (r:1 w:1) 
	// Storage: Doctors DoctorsByCountryRegionCity (r:1 w:1) 
	// Storage: Doctors DoctorCountByCountryRegionCity (r:1 w:1) 
	// Storage: Doctors DoctorCount (r:1 w:1) 
	fn deregister_doctor() -> Weight { 
		94_600_000_u64 
			.saturating_add(T::DbWeight::get().reads(4_u64)) 
			.saturating_add(T::DbWeight::get().writes(4_u64)) 
	}
}

// For backwards compatibility and tests
impl WeightInfo for () { 
	// Storage: Doctors Doctors (r:1 w:1) 
	// Storage: Doctors DoctorsByCountryRegionCity (r:1 w:1) 
	// Storage: Doctors DoctorCount (r:1 w:1) 
	// Storage: Doctors DoctorCountByCountryRegionCity (r:1 w:1) 
	// Storage: UserProfile ProfileRolesByAccountId (r:1 w:1) 
	fn register_doctor() -> Weight { 
		91_000_000_u64
			.saturating_add(RocksDbWeight::get().reads(5_u64)) 
			.saturating_add(RocksDbWeight::get().writes(5_u64)) 
	} 
	// Storage: Doctors Doctors (r:1 w:1) 
	// Storage: Doctors DoctorsByCountryRegionCity (r:2 w:2) 
	// Storage: Doctors DoctorCountByCountryRegionCity (r:2 w:2) 
	fn update_doctor() -> Weight { 
		124_000_000_u64
			.saturating_add(RocksDbWeight::get().reads(5_u64)) 
			.saturating_add(RocksDbWeight::get().writes(5_u64)) 
	} 
	// Storage: Doctors Doctors (r:1 w:1) 
	// Storage: Doctors DoctorsByCountryRegionCity (r:1 w:1) 
	// Storage: Doctors DoctorCountByCountryRegionCity (r:1 w:1) 
	// Storage: Doctors DoctorCount (r:1 w:1) 
	fn deregister_doctor() -> Weight { 
		94_600_000_u64
			.saturating_add(RocksDbWeight::get().reads(4_u64)) 
			.saturating_add(RocksDbWeight::get().writes(4_u64)) 
	} 
}
