pub use crate as pallet_fragments;
use crate::*;
use frame_support::{
	parameter_types,
	traits::{ConstU32, ConstU64},
};
use frame_system as system;
use sp_core::{ed25519::Signature, H256};
use sp_runtime::{
	testing::{Header, TestXt},
	traits::{BlakeTwo256, Extrinsic as ExtrinsicT, IdentifyAccount, IdentityLookup, Verify},
};

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;
pub const DATA: &str = "0x0155a0e40220";
pub const PROTO_HASH: Hash256 = [
	30, 138, 136, 186, 232, 46, 112, 65, 122, 54, 110, 89, 123, 195, 7, 150, 12, 134, 10, 179, 245,
	51, 83, 227, 72, 251, 5, 148, 207, 251, 119, 59,
];

pub const PUBLIC: [u8; 32] = [
	137, 65, 23, 149, 81, 74, 241, 98, 119, 101, 236, 239, 252, 189, 0, 39, 25, 240, 49, 96, 79,
	173, 215, 209, 136, 226, 220, 88, 91, 78, 26, 251,
];

// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
	pub enum Test where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic,
	{
		System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
		ProtosPallet: pallet_protos::{Pallet, Call, Storage, Event<T>},
		FragmentsPallet: pallet_fragments::{Pallet, Call, Storage, Event<T>},
		DetachPallet: pallet_detach::{Pallet, Call, Storage, Event<T>},
		Balances: pallet_balances::{Pallet, Call, Storage, Config<T>, Event<T>},
		Assets: pallet_assets::{Pallet, Call, Storage, Event<T>},
		Accounts: pallet_accounts::{Pallet, Call, Storage, Event<T>},
		Proxy: pallet_proxy::{Pallet, Call, Storage, Event<T>},
		Timestamp: pallet_timestamp::{Pallet, Call, Storage, Inherent},
	}
);

parameter_types! {
	pub const BlockHashCount: u64 = 250;
	pub const SS58Prefix: u8 = 42;
	pub StorageBytesMultiplier: u64 = 10;
}

impl system::Config for Test {
	type BaseCallFilter = frame_support::traits::Everything;
	type BlockWeights = ();
	type BlockLength = ();
	type DbWeight = ();
	type Origin = Origin;
	type Call = Call;
	type Index = u64;
	type BlockNumber = u64;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = sp_core::ed25519::Public;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = Header;
	type Event = Event;
	type BlockHashCount = BlockHashCount;
	type Version = ();
	type PalletInfo = PalletInfo;
	type AccountData = pallet_balances::AccountData<u64>;
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
	type SS58Prefix = SS58Prefix;
	type OnSetCode = ();
	type MaxConsumers = ConstU32<2>;
}

pub type Extrinsic = TestXt<Call, ()>;
type AccountId = <<Signature as Verify>::Signer as IdentifyAccount>::AccountId;

impl frame_system::offchain::SigningTypes for Test {
	type Public = <Signature as Verify>::Signer;
	type Signature = Signature;
}

impl<LocalCall> frame_system::offchain::SendTransactionTypes<LocalCall> for Test
where
	Call: From<LocalCall>,
{
	type OverarchingCall = Call;
	type Extrinsic = Extrinsic;
}

impl<LocalCall> frame_system::offchain::CreateSignedTransaction<LocalCall> for Test
where
	Call: From<LocalCall>,
{
	fn create_transaction<C: frame_system::offchain::AppCrypto<Self::Public, Self::Signature>>(
		call: Call,
		_public: <Signature as Verify>::Signer,
		_account: AccountId,
		nonce: u64,
	) -> Option<(Call, <Extrinsic as ExtrinsicT>::SignaturePayload)> {
		Some((call, (nonce, ())))
	}
}

impl pallet_randomness_collective_flip::Config for Test {}

impl pallet_balances::Config for Test {
	type Balance = u64;
	type DustRemoval = ();
	type Event = Event;
	type ExistentialDeposit = ConstU64<1>;
	type AccountStore = System;
	type WeightInfo = ();
	type MaxLocks = ();
	type MaxReserves = ();
	type ReserveIdentifier = [u8; 8];
}

impl pallet_assets::Config for Test {
	type Event = Event;
	type Balance = u64;
	type AssetId = u32;
	type Currency = ();
	type ForceOrigin = frame_system::EnsureRoot<AccountId>;
	type AssetDeposit = ConstU32<1>;
	type AssetAccountDeposit = ConstU32<10>;
	type MetadataDepositBase = ConstU32<1>;
	type MetadataDepositPerByte = ConstU32<1>;
	type ApprovalDeposit = ConstU32<1>;
	type StringLimit = ConstU32<50>;
	type Freezer = ();
	type WeightInfo = ();
	type Extra = ();
}

parameter_types! {
	pub const TicketsAssetId: u32 = 1337;
}

impl pallet_protos::Config for Test {
	type Event = Event;
	type WeightInfo = ();
	type StorageBytesMultiplier = StorageBytesMultiplier;
	type CurationExpiration = ConstU64<5>;
	type TicketsAssetId = TicketsAssetId;
}

impl pallet_accounts::Config for Test {
	type Event = Event;
	type WeightInfo = ();
	type EthChainId = ConstU64<5>; // goerli
	type EthFragContract = ();
	type EthConfirmations = ConstU64<1>;
	type Threshold = ConstU64<1>;
	type AuthorityId = pallet_accounts::crypto::FragAuthId;
}

impl pallet_proxy::Config for Test {
	type Event = Event;
	type Call = Call;
	type Currency = ();
	type ProxyType = ();
	type ProxyDepositBase = ConstU32<1>;
	type ProxyDepositFactor = ConstU32<1>;
	type MaxProxies = ConstU32<4>;
	type WeightInfo = ();
	type MaxPending = ConstU32<2>;
	type CallHasher = BlakeTwo256;
	type AnnouncementDepositBase = ConstU32<1>;
	type AnnouncementDepositFactor = ConstU32<1>;
}

impl pallet_fragments::Config for Test {
	type Event = Event;
	type WeightInfo = ();
}

impl pallet_detach::Config for Test {
	type Event = Event;
	type WeightInfo = ();
	type AuthorityId = pallet_detach::crypto::DetachAuthId;
}

impl pallet_timestamp::Config for Test {
	/// A timestamp: milliseconds since the unix epoch.
	type Moment = u64;
	type OnTimestampSet = ();
	type MinimumPeriod = ();
	type WeightInfo = ();
}

pub fn new_test_ext() -> sp_io::TestExternalities {
	let t = frame_system::GenesisConfig::default().build_storage::<Test>().unwrap();
	t.into()
}
