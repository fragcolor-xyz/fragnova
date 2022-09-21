use crate as pallet_accounts;
use crate::{dummy_data::*, mock, mock::*, *};
use codec::Encode;
use ethabi::Token;
use frame_support::{assert_noop, assert_ok, dispatch::DispatchResult, traits::TypedGet};
use frame_system::offchain::{SignedPayload, SigningTypes};
use serde_json::json;
use sp_core::{offchain::testing, H256};
use sp_runtime::{offchain::storage::StorageValueRef, SaturatedConversion};

pub use internal_lock_update_tests::lock_;
pub use link_tests::link_;

mod link_tests {
	use super::*;

	pub fn link_(link: &Link) -> DispatchResult {
		Accounts::link(Origin::signed(link.clamor_account_id), link.link_signature.clone())
	}

	#[test]
	fn link_should_work() {
		new_test_ext().execute_with(|| {
			let dd = DummyData::new();

			let link = dd.link;

			assert_ok!(link_(&link));

			assert_eq!(<EVMLinks<Test>>::get(&link.clamor_account_id).unwrap(), link.get_recovered_ethereum_account_id());
			assert!(
				<EVMLinksReverse<Test>>::get(&link.get_recovered_ethereum_account_id()).unwrap()
					== link.clamor_account_id
			);

			let event = <frame_system::Pallet<Test>>::events()
				.pop()
				.expect("Expected at least one EventRecord to be found")
				.event;
			assert_eq!(
				event,
				mock::Event::from(pallet_accounts::Event::Linked {
					sender: link.clamor_account_id,
					eth_key: link.get_recovered_ethereum_account_id()
				})
			);
		});
	}

	#[test]
	fn link_should_not_work_if_signature_parameter_is_invalid() {
		new_test_ext().execute_with(|| {
			let dd = DummyData::new();

			let link = Link { link_signature: dd.link_signature, ..dd.link };

			assert_noop!(link_(&link), Error::<Test>::VerificationFailed,);
		});
	}

	#[test]
	fn link_should_not_work_if_clamor_account_is_already_linked() {
		new_test_ext().execute_with(|| {
			let dd = DummyData::new();

			let link = dd.link;

			assert_ok!(link_(&link));

			let link_diff_ethereum_account_id = Link {
				clamor_account_id: link.clamor_account_id,
				link_signature: create_link_signature(
					link.clamor_account_id,
					dd.ethereum_account_pair,
				),
			};

			assert_noop!(
				link_(&link_diff_ethereum_account_id),
				Error::<Test>::AccountAlreadyLinked
			);
		});
	}

	#[test]
	fn link_should_not_work_if_ethereum_account_is_already_linked() {
		new_test_ext().execute_with(|| {
			let dd = DummyData::new();

			let link = Link {
				clamor_account_id: dd.account_id,
				link_signature: create_link_signature(
					dd.account_id,
					dd.ethereum_account_pair.clone(),
				),
			};

			assert_ok!(link_(&link));

			let link_diff_clamor_account_id = Link {
				clamor_account_id: dd.account_id_second,
				link_signature: create_link_signature(
					dd.account_id_second,
					dd.ethereum_account_pair.clone(),
				),
			};

			assert_noop!(link_(&link_diff_clamor_account_id), Error::<Test>::AccountAlreadyLinked);
		});
	}
}

mod unlink_tests {
	use super::*;

	#[test]
	fn unlink_should_work() {
		new_test_ext().execute_with(|| {
			let dd = DummyData::new();
			let link = dd.link;

			assert_ok!(link_(&link));

			assert_ok!(Accounts::unlink(
				Origin::signed(link.clamor_account_id),
				link.get_recovered_ethereum_account_id()
			));

			assert_eq!(<EVMLinks<Test>>::contains_key(&link.clamor_account_id), false);
			assert_eq!(<EVMLinksReverse<Test>>::contains_key(&link.get_recovered_ethereum_account_id()), false);

			assert!(<PendingUnlinks<Test>>::get().contains(&link.clamor_account_id));

			let event = <frame_system::Pallet<Test>>::events()
				.pop()
				.expect("Expected at least one EventRecord to be found")
				.event;
			assert_eq!(
				event,
				mock::Event::from(pallet_accounts::Event::Unlinked {
					sender: link.clamor_account_id,
					eth_key: link.get_recovered_ethereum_account_id(),
				})
			);
		});
	}

	#[test]
	fn unlink_should_not_work_if_link_does_not_exist() {
		new_test_ext().execute_with(|| {
			let dd = DummyData::new();
			let link = dd.link;

			assert_noop!(
				Accounts::unlink(
					Origin::signed(link.clamor_account_id),
					link.get_recovered_ethereum_account_id()
				),
				Error::<Test>::AccountNotLinked
			);
		});
	}

	#[test]
	fn unlink_should_not_work_if_origin_parameter_and_account_paramter_are_linked_but_not_with_each_other(
	) {
		new_test_ext().execute_with(|| {
			let dd = DummyData::new();

			let link = dd.link;

			let link_second = dd.link_second;

			assert_ok!(link_(&link));
			assert_ok!(link_(&link_second));

			assert_noop!(
				Accounts::unlink(
					Origin::signed(link.clamor_account_id),
					link_second.get_recovered_ethereum_account_id()
				),
				Error::<Test>::DifferentAccountLinked
			);
		});
	}
}

mod sync_frag_locks_tests {
	use super::*;

	fn hardcode_expected_request_and_response(
		state: &mut testing::OffchainState,
		lock: Lock,
	) -> u64 {
		let geth_url = Some(String::from("https://www.dummywebsite.com/"));

		sp_clamor::init(geth_url);

		let latest_block_number = lock
			.data
			.block_number // ensure that `lock.block_number` exists by making `latest_block_number` greater than or equal to it
			.saturating_add(<Test as pallet_accounts::Config>::EthConfirmations::get())
			.saturating_add(69)
			.saturating_add(1234567890);

		state.expect_request(testing::PendingRequest {
			method: String::from("POST"),
			uri: String::from_utf8(sp_clamor::clamor::get_geth_url().unwrap()).unwrap(),
			headers: vec![(String::from("Content-Type"), String::from("application/json"))],
			body: json!({
				"jsonrpc": "2.0",
				"method": "eth_blockNumber",
				"id": 1u64,
			})
			.to_string()
			.into_bytes(),
			response: Some(
				json!({
					"id": 69u64,
					"jsonrpc": "2.0",
					"result": format!("0x{:x}", latest_block_number),
				})
				.to_string()
				.into_bytes(),
			),
			sent: true,
			..Default::default()
		});

		let from_block = 0;
		let to_block = latest_block_number
			.saturating_sub(<Test as pallet_accounts::Config>::EthConfirmations::get());

		state.expect_request(testing::PendingRequest {
			method: String::from("POST"),
			uri: String::from_utf8(sp_clamor::clamor::get_geth_url().unwrap()).unwrap(),
			headers: vec![(String::from("Content-Type"), String::from("application/json"))],
			body: json!({
				"jsonrpc": "2.0",
				"method": "eth_getLogs", // i.e get the event logs of the smart contract (more info: https://docs.alchemy.com/alchemy/guides/eth_getlogs#what-are-logs)
				"id": "0", // WHY IS THIS A STRING @sinkingsugar  MOLTO IMPORTANTE!
				"params": [{
					"fromBlock": format!("0x{:x}", from_block),
					"toBlock": format!("0x{:x}", to_block), // Give us the event logs that were emitted (if any) from the block number `from_block` to the block number `to_block`, inclusive
					"address": <
					<Test as pallet_accounts::Config>::EthFragContract as pallet_accounts::EthFragContract
					>::get_partner_contracts()[0],
					"topics": [
						// [] to OR
						[pallet_accounts::LOCK_EVENT, pallet_accounts::UNLOCK_EVENT]
					],
				}]
			})
			.to_string()
			.into_bytes(),
			response: Some(
				json!({
					"id": 69u64,
					"jsonrpc": "2.0",
					"result": [
						{
							"address": <
							<Test as pallet_accounts::Config>::EthFragContract as pallet_accounts::EthFragContract
							>::get_partner_contracts()[0],
							"topics": [
								pallet_accounts::LOCK_EVENT,
								format!("0x{}", hex::encode(ethabi::encode(&[Token::Address(lock.data.sender)])))
							],
							"data": format!("0x{}", hex::encode(
								ethabi::encode(
									&[
										Token::Bytes(lock.data.signature.0.to_vec()),
										Token::Uint(lock.data.amount),
										Token::Uint(lock.data.lock_period)
									]
								),
							)),
							"blockNumber": format!("0x{:x}", lock.data.block_number),

							// Following key-values were blindly copied from https://docs.alchemy.com/alchemy/apis/ethereum/eth-getlogs (since they won't aren't even looked at in the function `sync_frag_locks`):
							// So they are all wrong
							"transactionHash": "0xab059a62e22e230fe0f56d8555340a29b2e9532360368f810595453f6fdd213b",
							"transactionIndex": "0xac",
							"blockHash": "0x8243343df08b9751f5ca0c5f8c9c0460d8a9b6351066fae0acbd4d3e776de8bb",
							"logIndex": "0x56",
							"removed": false,
						},
					]
				})
				.to_string()
				.into_bytes(),
			),
			sent: true,
			..Default::default()
		});

		to_block
	}

	#[test]
	fn sync_frag_locks_should_work() {
		let (mut t, pool_state, offchain_state, ed25519_public_key) = new_test_ext_with_ocw();

		let dd = DummyData::new();
		let lock = dd.lock;

		let to_block =
			hardcode_expected_request_and_response(&mut offchain_state.write(), lock.clone());

		let expected_data = EthLockUpdate {
			public: <Test as SigningTypes>::Public::from(ed25519_public_key),
			..lock.data
		};

		t.execute_with(|| {
			Accounts::sync_partner_contracts(1);

			let tx = pool_state.write().transactions.pop().unwrap();
			let tx = <Extrinsic as codec::Decode>::decode(&mut &*tx).unwrap();
			assert_eq!(tx.signature, None); // Because it's an **unsigned transaction** with a signed payload

			if let Call::Accounts(crate::Call::internal_lock_update { data, signature }) = tx.call {
				assert_eq!(data, expected_data);

				let signature_valid =
					<EthLockUpdate<<Test as SigningTypes>::Public> as SignedPayload<Test>>::verify::<
						crypto::FragAuthId,
					>(&data, signature); // Notice in `pallet_accounts` that `EthLockUpdate<T::Public>` implements the trait `SignedPayload`

				assert!(signature_valid); // If `signature_valid` is true, it means `payload` and `signature` recovered the public address `data.public`
			}

			let storage = StorageValueRef::persistent(b"frag_sync_last_block");
			assert_eq!(
				storage.get::<Vec<u8>>().unwrap().unwrap(),
				format!("0x{:x}", to_block).as_bytes().to_vec()
			);
		});
	}
}

mod internal_lock_update_tests {
	use super::*;

	pub fn lock_(lock: &Lock) -> DispatchResult {
		Accounts::internal_lock_update(
			Origin::none(),
			lock.data.clone(),
			sp_core::ed25519::Signature([69u8; 64]), // this can be anything and it will still work
		)
	}

	fn unlock_(unlock: &Unlock) -> DispatchResult {
		Accounts::internal_lock_update(
			Origin::none(),
			unlock.data.clone(),
			sp_core::ed25519::Signature([69u8; 64]), // this can be anything
		)
	}

	fn apply_20_percent(amount: u128) -> u128 {
		if amount == 0 {
			return 0;
		}
		let amount_float = amount as f64;
		let result = (amount_float/100.0 * 20.0).round();
		result as u128
	}

	#[test]
	fn lock_by_unlinked_account_should_lock_frag_internally_and_reserve_tickets_and_nova() {
		new_test_ext().execute_with(|| {
			let dd = DummyData::new();

			let current_block_number = System::block_number();

			let lock = dd.lock;

			assert_ok!(lock_(&lock));

			assert_eq!(
				<EthLockedFrag<Test>>::get(&lock.data.sender).unwrap(),
				EthLock {
					amount: SaturatedConversion::saturated_into::<
						<Test as pallet_balances::Config>::Balance,
					>(lock.data.amount.clone()),
					block_number: current_block_number,
					lock_period: U256::from(1),
				}
			);
			let percentage_amount = apply_20_percent(lock.data.amount.clone().as_u128());

			assert_eq!(
				<EthReservedTickets<Test>>::get(&lock.data.sender).unwrap(),
				SaturatedConversion::saturated_into::<
						<Test as pallet_balances::Config>::Balance,
					>(percentage_amount)
			);

			assert_eq!(
				<EthReservedNova<Test>>::get(&lock.data.sender).unwrap(),
				SaturatedConversion::saturated_into::<
					<Test as pallet_balances::Config>::Balance,
				>(percentage_amount)
			);

			let data_tuple = (
				lock.data.amount,
				lock.data.lock_period,
				lock.data.sender,
				lock.data.signature,
				true,
				lock.data.block_number,
			);

			let data_hash: H256 = data_tuple.using_encoded(sp_io::hashing::blake2_256).into();

			assert_eq!(<EVMLinkVotingClosed<Test>>::get(&data_hash).unwrap(), current_block_number);

			let event = <frame_system::Pallet<Test>>::events()
				.pop()
				.expect("Expected at least one EventRecord to be found")
				.event;
			assert_eq!(
				event,
				mock::Event::from(pallet_accounts::Event::Locked {
					eth_key: lock.data.sender,
					balance: SaturatedConversion::saturated_into::<
						<Test as pallet_balances::Config>::Balance,
					>(lock.data.amount),
					lock_period: lock.data.lock_period
				})
			);
		});
	}

	#[test]
	fn lock_by_linked_account_should_lock_frag_and_mint_tickets_and_assign_nova() {
		new_test_ext_with_nova().execute_with(|| {
			let dd = DummyData::new();
			let lock = dd.lock;
			let link = lock.link.clone();
			let current_block_number = System::block_number();

			assert_ok!(link_(&link));
			assert_ok!(lock_(&lock));
			// assert that Frag is locked in Clamor
			assert_eq!(
				<EthLockedFrag<Test>>::get(&lock.data.sender).unwrap(),
				EthLock {
					amount: SaturatedConversion::saturated_into::<
						<Test as pallet_balances::Config>::Balance,
					>(lock.data.amount.clone()),
					block_number: current_block_number,
					lock_period: U256::from(1),
				}
			);
			// check the balance of the Clamor account
			let minted = pallet_assets::Pallet::<Test>::balance(get_ticket_asset_id(),
																&link.clamor_account_id);
			let percentage_amount = apply_20_percent(lock.data.amount.clone().as_u128());
			assert_eq!(U256::from(minted), U256::from(percentage_amount));

			let nova = pallet_balances::Pallet::<Test>::free_balance(&link.clamor_account_id);
			assert_eq!(U256::from(nova), U256::from(percentage_amount));
		});
	}

	#[test]
	fn lock_should_not_work_if_locked_amount_is_zero() {
		new_test_ext().execute_with(|| {
			let dd = DummyData::new();

			let mut lock = dd.lock;
			lock.data.amount = U256::from(0u32);
			lock.data.lock_period = U256::from(1);
			lock.data.signature = create_lock_signature(
				lock.ethereum_account_pair.clone(),
				lock.data.amount.clone(),
				lock.data.lock_period.clone(),
			);

			assert_noop!(lock_(&lock), Error::<Test>::SystematicFailure);
		});
	}

	#[test]
	fn lock_should_not_work_if_the_sender_is_not_recovered_from_the_signature() {
		new_test_ext().execute_with(|| {
			let dd = DummyData::new();

			let mut lock = dd.lock;
			lock.data.signature = dd.lock_signature;

			assert_noop!(lock_(&lock), Error::<Test>::VerificationFailed);
		});
	}

	#[test]
	fn unlock_should_work() {
		new_test_ext().execute_with(|| {
			let dd = DummyData::new();
			let unlock = dd.unlock;
			//let lock = dd.lock;
			let link = unlock.lock.link.clone();

			let current_block_number = System::block_number(); //@sinkingsugar

			assert_ok!(lock_(&unlock.lock));

			assert_ok!(unlock_(&unlock));

			assert_eq!(
				<EthLockedFrag<Test>>::get(&unlock.data.sender).unwrap(),
				EthLock {
					amount: SaturatedConversion::saturated_into::<
						<Test as pallet_balances::Config>::Balance,
					>(0),
					block_number: current_block_number,
					lock_period: U256::from(999),
				}
			);

			assert_eq!(
				<EthReservedTickets<Test>>::get(&unlock.data.sender).unwrap(),
				SaturatedConversion::saturated_into::<
					<Test as pallet_balances::Config>::Balance,
				>(0)
			);

			assert_eq!(
				<EthReservedNova<Test>>::get(&unlock.data.sender).unwrap(),
				SaturatedConversion::saturated_into::<
					<Test as pallet_balances::Config>::Balance,
				>(0)
			);

			let minted = pallet_assets::Pallet::<Test>::balance(get_ticket_asset_id(),
																&link.clamor_account_id);
			assert_eq!(minted, 0);

			let nova = pallet_balances::Pallet::<Test>::free_balance(&link.clamor_account_id);
			assert_eq!(nova, 0);

			let data_tuple = (
				unlock.data.amount,
				unlock.data.lock_period,
				unlock.data.sender,
				unlock.data.signature,
				false,
				unlock.data.block_number,
			);

			let data_hash: H256 = data_tuple.using_encoded(sp_io::hashing::blake2_256).into();

			assert_eq!(<EVMLinkVotingClosed<Test>>::get(&data_hash).unwrap(), current_block_number);

			let event = <frame_system::Pallet<Test>>::events()
				.pop()
				.expect("Expected at least one EventRecord to be found")
				.event;
			assert_eq!(
				event,
				mock::Event::from(pallet_accounts::Event::Unlocked {
					eth_key: unlock.data.sender,
					balance: SaturatedConversion::saturated_into::<
						<Test as pallet_balances::Config>::Balance,
					>(0)
				})
			);
		});
	}

	#[test]
	fn unlock_should_unlink_clamor_account_if_clamor_account_is_linked() {
		new_test_ext_with_nova().execute_with(|| {
			let dd = DummyData::new();

			let unlock = dd.unlock;
			let lock = unlock.lock.clone();
			let link = lock.link.clone();

			assert_ok!(link_(&link));

			assert_ok!(lock_(&lock));
			assert_ok!(unlock_(&unlock));

			assert_eq!(<EVMLinks<Test>>::contains_key(&link.clamor_account_id), false);
			assert_eq!(<EVMLinksReverse<Test>>::contains_key(&link.get_recovered_ethereum_account_id()), false);

			assert!(<PendingUnlinks<Test>>::get().contains(&link.clamor_account_id));

			let event = System::events()
				.get(System::events().len() - 2)
				.expect("Expected at least two EventRecords to be found")
				.event
				.clone();
			assert_eq!(
				event,
				mock::Event::from(pallet_accounts::Event::Unlinked {
					sender: link.clamor_account_id,
					eth_key: link.get_recovered_ethereum_account_id()
				})
			);
		});
	}

	#[test]
	fn unlock_should_not_work_if_unlocked_amount_is_greater_than_zero() {
		new_test_ext().execute_with(|| {
			let dd = DummyData::new();
			let mut unlock = dd.unlock;
			unlock.data.amount = U256::from(69u32); // greater than zero
			unlock.data.signature = create_unlock_signature(
				unlock.lock.ethereum_account_pair.clone(),
				U256::from(69u32),
			);

			assert_ok!(lock_(&unlock.lock));

			assert_noop!(unlock_(&unlock), Error::<Test>::SystematicFailure);
		});
	}
}
