#![cfg(test)]

use frame_support::{parameter_types, traits::ConstU64, PalletId};
use frame_system as system;
use pallet_balances::AccountData;
use sp_core::H256;
use sp_runtime::{
	testing::Header,
	traits::{BlakeTwo256, IdentityLookup},
};

use primitives_ethereum_address::EthereumAddress;
use primitives_profile_roles::ProfileRoles;

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;

pub type AccountId = u64;

frame_support::construct_runtime!(
	pub enum Test where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic,
	{
		System: frame_system,
		Balances: pallet_balances,
		Labs: labs,
		Services: services,
		Orders: orders,
		GeneticTesting: genetic_testing,
		Certifications: certifications,
		UserProfile: user_profile,
		Timestamp: pallet_timestamp,
		RandomnessCollectiveFlip: pallet_randomness_collective_flip,
		Assets: pallet_assets,
	}
);

impl pallet_randomness_collective_flip::Config for Test {}

parameter_types! {
	pub const BlockHashCount: u64 = 250;
	pub const SS58Prefix: u8 = 42;
}

impl system::Config for Test {
	type BaseCallFilter = frame_support::traits::Everything;
	type BlockWeights = ();
	type BlockLength = ();
	type AccountId = AccountId;
	type RuntimeCall = RuntimeCall;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Index = u64;
	type BlockNumber = u64;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type Header = Header;
	type RuntimeEvent = RuntimeEvent;
	type RuntimeOrigin = RuntimeOrigin;
	type BlockHashCount = BlockHashCount;
	type DbWeight = ();
	type Version = ();
	type PalletInfo = PalletInfo;
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type AccountData = AccountData<Balance>;
	type SystemWeightInfo = ();
	type SS58Prefix = SS58Prefix;
	type OnSetCode = ();
	type MaxConsumers = frame_support::traits::ConstU32<16>;
}

pub type Moment = u64;
pub const MILLISECS_PER_BLOCK: Moment = 6000;
pub const SLOT_DURATION: Moment = MILLISECS_PER_BLOCK;

parameter_types! {
	pub const MinimumPeriod: Moment = SLOT_DURATION / 2;
	pub const LabPalletId: PalletId = PalletId(*b"dbio/lab");
	pub const OrderPalletId: PalletId = PalletId(*b"dbio/ord");
}

impl pallet_timestamp::Config for Test {
	/// A timestamp: milliseconds since the unix epoch.
	type Moment = Moment;
	type OnTimestampSet = ();
	type MinimumPeriod = MinimumPeriod;
	type WeightInfo = ();
}

type Balance = u64;

parameter_types! {
	pub static ExistentialDeposit: Balance = 0;
}

impl pallet_balances::Config for Test {
	type MaxLocks = ();
	type MaxReserves = ();
	type ReserveIdentifier = [u8; 8];
	/// The type for recording an account's balance.
	type Balance = Balance;
	/// The ubiquitous event type.
	type RuntimeEvent = RuntimeEvent;
	type DustRemoval = ();
	type ExistentialDeposit = ExistentialDeposit;
	type AccountStore = System;
	type WeightInfo = ();
}

pub type AssetId = u32;
pub type AssetBalance = u128;

parameter_types! {
	pub const ApprovalDeposit: Balance = 1;
	pub const AssetDeposit: Balance = 1;
	pub const MetadataDepositBase: Balance = 1;
	pub const MetadataDepositPerByte: Balance = 1;
	pub const StringLimit: u32 = 50;
}

impl pallet_assets::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type Balance = AssetBalance;
	type AssetId = AssetId;
	type Currency = Balances;
	type ForceOrigin = frame_system::EnsureRoot<AccountId>;
	type AssetAccountDeposit = ConstU64<10>;
	type AssetDeposit = AssetDeposit;
	type MetadataDepositBase = MetadataDepositBase;
	type MetadataDepositPerByte = MetadataDepositPerByte;
	type ApprovalDeposit = ApprovalDeposit;
	type StringLimit = StringLimit;
	type Freezer = ();
	type Extra = ();
	type WeightInfo = ();
}

impl labs::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type Currency = Balances;
	type Services = Services;
	type Orders = Orders;
	type PalletId = LabPalletId;
	type Certifications = Certifications;
	type EthereumAddress = EthereumAddress;
	type ProfileRoles = ProfileRoles;
	type UserProfile = UserProfile;
	type LabWeightInfo = ();
}

impl services::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type Currency = Balances;
	type ServiceOwner = Labs;
	type WeightInfo = ();
}

impl certifications::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type CertificationOwner = Labs;
	type WeightInfo = ();
}

impl genetic_testing::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type Orders = Orders;
	type RandomnessSource = RandomnessCollectiveFlip;
	type GeneticTestingWeightInfo = ();
}

impl orders::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type Services = Services;
	type GeneticTesting = GeneticTesting;
	type Currency = Balances;
	type Assets = Assets;
	type OrdersWeightInfo = ();
	type PalletId = OrderPalletId;
}

impl user_profile::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type EthereumAddress = EthereumAddress;
	type ProfileRoles = ProfileRoles;
	type WeightInfo = ();
}
