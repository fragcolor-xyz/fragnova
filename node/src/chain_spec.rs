//! A chain specification, or "chain spec", is a collection of configuration information that dictates which network a blockchain node will connect to,
//! which entities it will initially communicate with, and what consensus-critical state it must have at genesis.
//!
//! Source: https://docs.substrate.io/v3/runtime/chain-specs/

use clamor_runtime::{
	AccountId, AccountsConfig, AssetsConfig, AuraConfig, BalancesConfig, DetachConfig,
	GenesisConfig, GrandpaConfig, IndicesConfig, Signature, SudoConfig, SystemConfig, WASM_BINARY,
};
use sc_service::ChainType;
use sp_consensus_aura::sr25519::AuthorityId as AuraId;
use sp_core::{ecdsa, ed25519, sr25519, Pair, Public};
use sp_finality_grandpa::AuthorityId as GrandpaId;
use sp_runtime::traits::{IdentifyAccount, Verify};

pub type UploadId = ecdsa::Public;
pub type EthId = ecdsa::Public;
pub type DetachId = ed25519::Public;

// The URL for the telemetry server.
// const STAGING_TELEMETRY_URL: &str = "wss://telemetry.polkadot.io/submit/";

/// Specialized `ChainSpec`. This is a specialization of the general Substrate ChainSpec type.
pub type ChainSpec = sc_service::GenericChainSpec<GenesisConfig>;

/// Generate a crypto pair from seed.
pub fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
	TPublic::Pair::from_string(&format!("//{}", seed), None)
		.expect("static values are valid; qed")
		.public()
}

/// Generate a crypto pair from seed.
pub fn get_from_seed_to_eth(seed: &str) -> ecdsa::Public {
	let pair = ed25519::Pair::from_string(&format!("//{}", seed), None).unwrap();
	let mut message = b"fragments-frag-ecdsa-keys".to_vec();
	message.append(&mut pair.public().to_vec()); // salt it with the public key
	let signature = pair.sign(&message);
	let hash = sp_core::keccak_256(&signature.0[..]);
	let pair = ecdsa::Pair::from_seed_slice(&hash[..]).unwrap();
	let public = pair.public();
	public
}

type AccountPublic = <Signature as Verify>::Signer;

/// Generate an account ID from seed.
pub fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId
where
	AccountPublic: From<<TPublic::Pair as Pair>::Public>,
{
	AccountPublic::from(get_from_seed::<TPublic>(seed)).into_account()
}

/// Generate an Aura authority key.
pub fn authority_keys_from_seed(s: &str) -> (AuraId, GrandpaId, UploadId, EthId, DetachId) {
	(
		get_from_seed::<AuraId>(s),
		get_from_seed::<GrandpaId>(s),
		get_from_seed::<UploadId>(s),
		get_from_seed_to_eth(s),
		get_from_seed::<DetachId>(s),
	)
}

pub fn development_config() -> Result<ChainSpec, String> {
	let wasm_binary = WASM_BINARY.ok_or_else(|| "Development wasm not available".to_string())?;

	Ok(ChainSpec::from_genesis(
		// Name
		"Development",
		// ID
		"dev",
		ChainType::Development,
		move || {
			testnet_genesis(
				wasm_binary,
				// Initial PoA authorities
				vec![authority_keys_from_seed("Alice")],
				// Sudo account
				get_account_id_from_seed::<sr25519::Public>("Alice"),
				// Pre-funded accounts
				vec![
					get_account_id_from_seed::<sr25519::Public>("Alice"),
					get_account_id_from_seed::<sr25519::Public>("Bob"),
					get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
					get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
					get_account_id_from_seed::<sr25519::Public>("Dave"),
				],
				true,
			)
		},
		// Bootnodes
		vec![],
		// Telemetry
		None,
		// Protocol ID
		None,
		None,
		// Properties
		None,
		// Extensions
		None,
	))
}

pub fn local_testnet_config() -> Result<ChainSpec, String> {
	let wasm_binary = WASM_BINARY.ok_or_else(|| "Development wasm not available".to_string())?;

	Ok(ChainSpec::from_genesis(
		// Name
		"Local Testnet",
		// ID
		"local_testnet",
		ChainType::Local,
		move || {
			testnet_genesis(
				wasm_binary,
				// Initial PoA authorities
				vec![authority_keys_from_seed("Alice"), authority_keys_from_seed("Bob")],
				// Sudo account
				get_account_id_from_seed::<sr25519::Public>("Alice"),
				// Pre-funded accounts
				vec![
					get_account_id_from_seed::<sr25519::Public>("Alice"),
					get_account_id_from_seed::<sr25519::Public>("Bob"),
					get_account_id_from_seed::<sr25519::Public>("Charlie"),
					get_account_id_from_seed::<sr25519::Public>("Dave"),
					get_account_id_from_seed::<sr25519::Public>("Eve"),
					get_account_id_from_seed::<sr25519::Public>("Ferdie"),
					get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
					get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
					get_account_id_from_seed::<sr25519::Public>("Charlie//stash"),
					get_account_id_from_seed::<sr25519::Public>("Dave//stash"),
					get_account_id_from_seed::<sr25519::Public>("Eve//stash"),
					get_account_id_from_seed::<sr25519::Public>("Ferdie//stash"),
				],
				true,
			)
		},
		// Bootnodes
		vec![],
		// Telemetry
		None,
		// Protocol ID
		None,
		None,
		// Properties
		None,
		// Extensions
		None,
	))
}

pub fn live_config() -> Result<ChainSpec, String> {
	let wasm_binary = WASM_BINARY.ok_or_else(|| "Development wasm not available".to_string())?;

	Ok(ChainSpec::from_genesis(
		// Name
		"Live Network",
		// ID
		"live",
		ChainType::Live,
		move || {
			GenesisConfig {
				system: SystemConfig {
					// Add Wasm runtime to storage.
					code: wasm_binary.to_vec(),
				},
				balances: BalancesConfig::default(),
				aura: AuraConfig::default(),
				grandpa: GrandpaConfig::default(),
				sudo: SudoConfig::default(),
				transaction_payment: Default::default(),
				indices: IndicesConfig { indices: vec![] },
				detach: DetachConfig::default(),
				assets: AssetsConfig::default(),
				accounts: AccountsConfig::default(),
			}
		},
		// Bootnodes
		vec![],
		// Telemetry
		None,
		// Protocol ID
		None,
		None,
		// Properties
		None,
		// Extensions
		None,
	))
}

/// Configure initial storage state for FRAME modules.
fn testnet_genesis(
	wasm_binary: &[u8],
	initial_authorities: Vec<(AuraId, GrandpaId, UploadId, EthId, DetachId)>,
	root_key: AccountId,
	endowed_accounts: Vec<AccountId>,
	_enable_println: bool,
) -> GenesisConfig {
	GenesisConfig {
		system: SystemConfig {
			// Add Wasm runtime to storage.
			code: wasm_binary.to_vec(),
		},
		balances: BalancesConfig {
			// Configure endowed accounts with initial balance of 1 << 60.
			balances: endowed_accounts
				.iter()
				.cloned()
				.map(|k| (k, (1000000000u128 * (10u128.pow(12)))))
				.collect(),
		},
		aura: AuraConfig {
			authorities: initial_authorities.iter().map(|x| (x.0.clone())).collect(),
		},
		grandpa: GrandpaConfig {
			authorities: initial_authorities.iter().map(|x| (x.1.clone(), 1)).collect(),
		},
		sudo: SudoConfig {
			// Assign network admin rights.
			key: Some(root_key.clone()),
		},
		transaction_payment: Default::default(),
		indices: IndicesConfig { indices: vec![] },
		detach: DetachConfig {
			eth_authorities: initial_authorities.iter().map(|x| (x.3.clone())).collect(),
			keys: initial_authorities.iter().map(|x| (x.4.clone())).collect(),
		},
		assets: AssetsConfig {
			assets: vec![(1337, root_key.clone(), true, 1, false)], // Genesis assets: id, owner, is_sufficient, min_balance, is_tradeable
			metadata: vec![(1337, "Fragnova Network Tickets".into(), "TICKET".into(), 0)], // Genesis metadata: id, name, symbol, decimals
			accounts: vec![], // Genesis accounts: id, account_id, balance
		},
		accounts: AccountsConfig {
			keys: initial_authorities.iter().map(|x| (x.4.clone())).collect(),
		},
	}
}
