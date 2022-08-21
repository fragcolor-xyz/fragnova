//! Benchmarking setup for pallet-fragments

use super::*;
#[allow(unused)]
use crate::Pallet as Protos;
use frame_benchmarking::{account, benchmarks, vec, whitelisted_caller};
use frame_system::RawOrigin;
use pallet_detach::Pallet as Detach;
use protos::categories::{Categories, TextCategories};
use sp_clamor::CID_PREFIX;
use sp_io::hashing::blake2_256;

const SEED: u32 = 0;

fn assert_last_event<T: Config>(generic_event: <T as Config>::Event) {
	frame_system::Pallet::<T>::assert_last_event(generic_event.into());
}

benchmarks! {

	where_clause { where
		T::AccountId: AsRef<[u8]>
	}

	upload {
		let caller: T::AccountId = whitelisted_caller();
		let mut immutable_data: [u8; 9] = [0; 9];
		hex::decode_to_slice("010000000b00803103", &mut immutable_data).unwrap();
		let immutable_data = immutable_data.to_vec();
		let proto_hash = blake2_256(immutable_data.as_slice());
		let references = vec![];
		let cid = [&CID_PREFIX[..], &proto_hash[..]].concat();
			let cid = cid.to_base58();
			let cid = [&b"z"[..], cid.as_bytes()].concat();
	}: _(RawOrigin::Signed(caller), references, Categories::Text(TextCategories::Plain), <Vec<Vec<u8>>>::new(), None, UsageLicense::Closed, immutable_data)
	verify {
		assert_last_event::<T>(Event::<T>::Uploaded { proto_hash: proto_hash, cid: cid }.into())
	}

	patch {
		let caller: T::AccountId = whitelisted_caller();

		let mut immutable_data: [u8; 9] = [0; 9];
		hex::decode_to_slice("010000000b00803103", &mut immutable_data).unwrap();
		let immutable_data = immutable_data.to_vec();
		let proto_hash = blake2_256(immutable_data.as_slice());
		let references = vec![];

		Protos::<T>::upload(RawOrigin::Signed(caller.clone()).into(), references, Categories::Text(TextCategories::Plain), <Vec<Vec<u8>>>::new(), None, UsageLicense::Closed, immutable_data.clone())?;

		let mut data: [u8; 3] = [0; 3];
		hex::decode_to_slice("C0FFEE", &mut data).unwrap();

		let patch_hash = blake2_256(data.as_slice());
		let cid = [&CID_PREFIX[..], &patch_hash[..]].concat();
			let cid = cid.to_base58();
			let cid = [&b"z"[..], cid.as_bytes()].concat();
	}: _(RawOrigin::Signed(caller), proto_hash , Some(UsageLicense::Tickets(Compact(123))), vec![], None, data.to_vec())
	verify {
		assert_last_event::<T>(Event::<T>::Patched { proto_hash: proto_hash, cid: cid }.into())
	}

	detach {
		let caller: T::AccountId = whitelisted_caller();

		let mut immutable_data: [u8; 9] = [0; 9];
		hex::decode_to_slice("010000000b00803103", &mut immutable_data).unwrap();
		let immutable_data = immutable_data.to_vec();
		let proto_hash = blake2_256(immutable_data.as_slice());
		let references = vec![];

		Protos::<T>::upload(RawOrigin::Signed(caller.clone()).into(), references, Categories::Text(TextCategories::Plain), <Vec<Vec<u8>>>::new(), None, UsageLicense::Closed, immutable_data.clone())?;

		let public: [u8; 33] = [2, 44, 133, 69, 18, 57, 0, 152, 97, 145, 160, 85, 122, 14, 119, 232, 88, 169, 142, 77, 139, 133, 214, 67, 188, 128, 137, 28, 23, 247, 242, 193, 104];
		let target_account: Vec<u8> = [203, 109, 249, 222, 30, 252, 167, 163, 153, 138, 142, 173, 78, 2, 21, 157, 95, 169, 156, 62, 13, 79, 214, 67, 38, 103, 57, 11, 180, 114, 104, 84].to_vec();
		Detach::<T>::add_eth_auth(RawOrigin::Root.into(), sp_core::ecdsa::Public::from_raw(public))?;

		let pre_len: usize = <pallet_detach::DetachRequests<T>>::get().len();
	}: _(RawOrigin::Signed(caller), proto_hash, pallet_detach::SupportedChains::EthereumMainnet, target_account)
	verify {
		assert_eq!(<pallet_detach::DetachRequests<T>>::get().len(), pre_len + 1 as usize);
	}

	transfer {
		let caller: T::AccountId = whitelisted_caller();
		let new_owner: T::AccountId = account("Sample", 100, SEED);

		let mut immutable_data: [u8; 9] = [0; 9];
		hex::decode_to_slice("010000000b00803103", &mut immutable_data).unwrap();
		let immutable_data = immutable_data.to_vec();
		let proto_hash = blake2_256(immutable_data.as_slice());
		let references = vec![];

		Protos::<T>::upload(RawOrigin::Signed(caller.clone()).into(), references, Categories::Text(TextCategories::Plain), <Vec<Vec<u8>>>::new(), None, UsageLicense::Closed, immutable_data.clone())?;
	}: _(RawOrigin::Signed(caller), proto_hash, new_owner.clone())
	verify {
		assert_last_event::<T>(Event::<T>::Transferred { proto_hash: proto_hash, owner_id: new_owner }.into())
	}

	impl_benchmark_test_suite!(Protos, crate::mock::new_test_ext(), crate::mock::Test);
}
