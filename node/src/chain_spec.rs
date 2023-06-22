use hex::ToHex;
use hex_literal::hex;
use node_primitives::{AccountId, Balance};
use pallet_im_online::sr25519::AuthorityId as ImOnlineId;
use sc_service::{ChainType, Properties};
use sc_telemetry::TelemetryEndpoints;
use sp_consensus_aura::sr25519::AuthorityId as AuraId;
use sp_consensus_grandpa::AuthorityId as GrandpaId;
use sp_core::{crypto::Ss58Codec, crypto::UncheckedInto, sr25519, Pair, Public};
use sp_runtime::traits::{IdentifyAccount, Verify};
use std::str::FromStr;
use zeno_runtime::{
	constants::currency::*, opaque::SessionKeys, AuraConfig, BalancesConfig, CouncilConfig,
	GenesisConfig, GrandpaConfig, ImOnlineConfig, Multisig, SessionConfig, Signature, SudoConfig,
	SystemConfig, TechnicalCommitteeConfig, ValidatorSetConfig, WASM_BINARY,
};

const STAGING_TELEMETRY_URL: &str = "wss://telemetry.polkadot.io/submit/";
const DEFAULT_PROTOCOL_ID: &str = "zeno";

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

type AccountPublic = <Signature as Verify>::Signer;

/// Generate an account ID from seed.
pub fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId
where
	AccountPublic: From<<TPublic::Pair as Pair>::Public>,
{
	AccountPublic::from(get_from_seed::<TPublic>(seed)).into_account()
}

pub fn public_from_ss58<TPublic: Public + FromStr>(ss58: &str) -> TPublic
where
	<TPublic as FromStr>::Err: std::fmt::Debug,
{
	TPublic::from_ss58check(ss58).expect("supply valid ss58!")
}

fn session_keys(aura: AuraId, grandpa: GrandpaId, im_online: ImOnlineId) -> SessionKeys {
	SessionKeys { aura, grandpa, im_online }
}

/// Generate an Aura authority key.
pub fn authority_keys_from_seed(
	seed: &str,
) -> (AccountId, AccountId, AuraId, GrandpaId, ImOnlineId) {
	(
		get_account_id_from_seed::<sr25519::Public>(&format!("{}//stash", seed)),
		get_account_id_from_seed::<sr25519::Public>(seed),
		get_from_seed::<AuraId>(seed),
		get_from_seed::<GrandpaId>(seed),
		get_from_seed::<ImOnlineId>(seed),
	)
}

/// Token
pub fn zeno_properties() -> Properties {
	let mut p = Properties::new();
	p.insert("ss58format".into(), 42.into());
	p.insert("tokenDecimals".into(), 15_i16.into());
	p.insert("tokenSymbol".into(), "DOLLARS".into());
	p
}

pub fn multisig_account(mut accounts: Vec<AccountId>, threshold: u16) -> AccountId {
	// sort accounts by public key, as js/apps would do
	accounts.sort_by(|a, b| (*a).encode_hex::<String>().cmp(&(*b).encode_hex::<String>()));
	Multisig::multi_account_id(&accounts, threshold)
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
					(get_account_id_from_seed::<sr25519::Public>("Alice"), 1_000 * DOLLARS),
					(get_account_id_from_seed::<sr25519::Public>("Bob"), 1_000 * DOLLARS),
					(get_account_id_from_seed::<sr25519::Public>("Alice//stash"), 1_000 * DOLLARS),
					(get_account_id_from_seed::<sr25519::Public>("Bob//stash"), 1_000 * DOLLARS),
				],
				vec![
					get_account_id_from_seed::<sr25519::Public>("Alice"),
					get_account_id_from_seed::<sr25519::Public>("Bob"),
					get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
					get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
				],
				true,
			)
		},
		// Bootnodes
		vec![],
		// Telemetry
		None,
		// Protocol ID
		Some(DEFAULT_PROTOCOL_ID),
		// Properties
		None,
		Some(zeno_properties()),
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
					(get_account_id_from_seed::<sr25519::Public>("Alice"), 1_000 * DOLLARS),
					(get_account_id_from_seed::<sr25519::Public>("Bob"), 1_000 * DOLLARS),
					(get_account_id_from_seed::<sr25519::Public>("Charlie"), 1_000 * DOLLARS),
					(get_account_id_from_seed::<sr25519::Public>("Dave"), 1_000 * DOLLARS),
					(get_account_id_from_seed::<sr25519::Public>("Eve"), 1_000 * DOLLARS),
					(get_account_id_from_seed::<sr25519::Public>("Ferdie"), 1_000 * DOLLARS),
					(get_account_id_from_seed::<sr25519::Public>("Alice//stash"), 1_000 * DOLLARS),
					(get_account_id_from_seed::<sr25519::Public>("Bob//stash"), 1_000 * DOLLARS),
					(
						get_account_id_from_seed::<sr25519::Public>("Charlie//stash"),
						1_000 * DOLLARS,
					),
					(get_account_id_from_seed::<sr25519::Public>("Dave//stash"), 1_000 * DOLLARS),
					(get_account_id_from_seed::<sr25519::Public>("Eve//stash"), 1_000 * DOLLARS),
					(get_account_id_from_seed::<sr25519::Public>("Ferdie//stash"), 1_000 * DOLLARS),
				],
				vec![
					get_account_id_from_seed::<sr25519::Public>("Alice"),
					get_account_id_from_seed::<sr25519::Public>("Bob"),
				],
				true,
			)
		},
		// Bootnodes
		vec![],
		// Telemetry
		None,
		// Protocol ID
		Some(DEFAULT_PROTOCOL_ID),
		// Properties
		None,
		Some(zeno_properties()),
		// Extensions
		None,
	))
}

pub fn staging_network_config() -> ChainSpec {
	let boot_nodes = vec![];

	ChainSpec::from_genesis(
		"Zeno Staging",
		"zeno_staging",
		ChainType::Live,
		staging_network_config_genesis,
		boot_nodes,
		Some(
			TelemetryEndpoints::new(vec![(STAGING_TELEMETRY_URL.to_string(), 0)])
				.expect("Staging telemetry url is valid; qed"),
		),
		Some(DEFAULT_PROTOCOL_ID),
		None,
		Some(zeno_properties()),
		None,
	)
}

fn staging_network_config_genesis() -> GenesisConfig {
	let wasm_binary =
		WASM_BINARY.ok_or_else(|| "Development wasm not available".to_string()).unwrap();
	// for i in 1 2 3 4; do for j in stash controller; do subkey inspect "$SECRET//$i//$j"; done; done
	// for i in 1 2 3 4; do for j in babe; do subkey --sr25519 inspect "$SECRET//$i//$j"; done; done
	// for i in 1 2 3 4; do for j in grandpa; do subkey --ed25519 inspect "$SECRET//$i//$j"; done; done
	// for i in 1 2 3 4; do for j in im_online; do subkey --sr25519 inspect "$SECRET//$i//$j"; done; done
	let initial_authorities: Vec<(AccountId, AccountId, AuraId, GrandpaId, ImOnlineId)> = vec![
		(
			// 5DWhheWDPzVFaBvqrrfSQrA9aFZppbsNnatRMook7FChCybQ
			hex!["401017229ff5388d262a3145e99fae36cd10432e84912e2675495fd9cab4c04d"].into(),
			// 5CFCr2DqTGrAyUSCEzSDFuGn4zsMappHUyBW4APBMCdFafJf
			hex!["08020a09173661c9b41affb513f5091214b19e53825bbe53d472fbd7d9ac0f3d"].into(),
			// 5FRLoFLM56PZSPKQ7AssQYLGb1oJBhQgPDAqDLw5sqiXHhrW
			hex!["9471f6879a0bb8129eb1e4a869a1d138ed17f14b5dc69a7c2c3c1ddafb61635d"]
				.unchecked_into(),
			// 5Dmx5cYTZKzybz9d1hLZGtCgPVE1oZ2UpBAcnHmi8Seb5ZtQ
			hex!["4bb12caf3375637bc70193c25ec2a2479b0f1dd2ad0fb624e1b5b6778f67d2f1"]
				.unchecked_into(),
			// 5FcEqxwHgh39oqJZvcDhXSKshpUenMiBCqyUaZXmv6JycTty
			hex!["9cc19f53450e5bda61816d9982cd8bb4bf453bdd3f6ae6f022298c827e7fac0f"]
				.unchecked_into(),
		),
		(
			// 5GpRt7DNKUtj5XRYJTtzMnwM1jqPzXYEm6pSMG6dY74fNXZt
			hex!["d249f44f25ed1b2363245c59ea104a4b252b9cdc5a338ac2d139c7db9a5cff40"].into(),
			// 5FsXN8XXwrjb8TBK4sKDbF44WLyf41npGsPhKoDZmYNX5CJC
			hex!["a869ea3e9921811a708e912f724bb64af9fed54427209890c571746f221b497f"].into(),
			// 5G4U4aYiTPbwB9n5dhpxkE83yDJw5RXDMnn9whZuRBH2E9uZ
			hex!["b0c27f342aec278ac1f6bc21437119485a3938b687fc2ecc92aad096df05a975"]
				.unchecked_into(),
			// 5CeZit9rqkV3pbQMdFWxEeAjD83CmYWGpzPYStQCbPNSw5rU
			hex!["19d2f621ae9690cbd9d37bc919a91b9aa8a072dc987a1fdcb8b9afcb65fac7fd"]
				.unchecked_into(),
			// 5FsxicJpWnXFbegfARjEBe6PU2ESn6hBu6SeLNcgnTuJzt2V
			hex!["a8bf42d84432b080e93e948a1b4034fb2d3956aaadb9307aa7435e532ea2b56d"]
				.unchecked_into(),
		),
		(
			// 5CqqPkoJcnk9TZJUAea8yPiDb6ZPvUL4hozikMysn45rxnkD
			hex!["226b695dd01733d0a3f5baa1dc6fe69bd6605d3a2c3efe3b0f22a9266c21a255"].into(),
			// 5HC6S6G64Jm1p9nZvtjzftLdTyMdw2Mqr25hwiFyJRFF1o1z
			hex!["e2cfe22fd5c21fee880b4813e7b8d11de860486ef60c916072d81210bae03c63"].into(),
			// 5HYHTPBNyqQb4jxScXNtBAdoK7jjHnhDd2DFpaav7HHgn5HX
			hex!["f235e559bc6edf38ea1c001f2d6bee8c13b0a61ac848428513af77295b933060"]
				.unchecked_into(),
			// 5CWGZtXD1XR9Wyc9y2ZpdWsLKJNF6H2hKokLpCc8NBFbRC44
			hex!["137f3fc6c427872952cadf185d254f118985db4a0b2db51f3ebd10df77775cd1"]
				.unchecked_into(),
			// 5GKcxEJitd9KSwx5CZVUdfrEEo32kiUZBm9vs39LhxVE6wkp
			hex!["bc511b2d7ce401fb975fbcb9f164a11942b3003fd75235983c2025c89e0ff319"]
				.unchecked_into(),
		),
		(
			// 5DXjBVqPKAMRYoqEXZatfvwH22wyJETFG1wBCo55Jz7xLVR1
			hex!["40d85189f85ca2bd00d12f9aaacdf6786f4a81ef5f691bb1027c4993c3b21140"].into(),
			// 5CPA4DJzfhJJhU2C2nXZ9MBPHPj36ipeQmvmepsk58KBjHqo
			hex!["0e129c7197b7afcb4b938b4384d74ebe80f427b465d370f7b4f13a2cc4ca8c15"].into(),
			// 5FeQoaUQBS48FiMVP4bCh7KaztmRkm6tKGc8yqzEmC5jJhWh
			hex!["9e69a33ac604ffc7a66f99f82cf0728997114d7c4de130734f5ab962332f3f25"]
				.unchecked_into(),
			// 5DsHYb9RXu1f3wHN4nPThkUGx4gamYyUQ7p6Zk1BR3UNcyxU
			hex!["4fc2ed4c37349489421a905411c99713add4b5aa800cf8b8d19db62d86f6f345"]
				.unchecked_into(),
			// 5GvMt8Z1gM4hudYnBPQF83HWmhvPeNfhEo1Yfg2BDw2CYdZ9
			hex!["d6cff6807ec79e13c06128bdc51eaa6544068f6c54c7d8c23901ad408c604538"]
				.unchecked_into(),
		),
	];
	let committee_accounts: Vec<AccountId> = vec![
		"5GHJvRMyqSGnMSWoLgE9WSufoZZ6dBowdkDV4dvYApBykd9Z".parse().unwrap(),
		"5HK2RNXZRmVEsPDXidWwV1zNFzRAaCRxzsvtYio179jELd49".parse().unwrap(),
		"5DRVZN78VKbgNny4bHf2MpmHq6SVhrT6g23ciTYHi36woLMT".parse().unwrap(),
		"5CDrkPqy6KQDYNXNXiK5NMij1p7gNQuR2WB9My8y1fYvAspA".parse().unwrap(),
	];
	let sudo_account: AccountId =
		public_from_ss58::<sr25519::Public>("5FemZuvaJ7wVy4S49X7Y9mj7FyTR4caQD5mZo2rL7MXQoXMi")
			.into();
	let multisig_controller_accounts: Vec<AccountId> = vec![
		public_from_ss58::<sr25519::Public>("5CDrkPqy6KQDYNXNXiK5NMij1p7gNQuR2WB9My8y1fYvAspA")
			.into(),
		public_from_ss58::<sr25519::Public>("5DRVZN78VKbgNny4bHf2MpmHq6SVhrT6g23ciTYHi36woLMT")
			.into(),
		public_from_ss58::<sr25519::Public>("5HK2RNXZRmVEsPDXidWwV1zNFzRAaCRxzsvtYio179jELd49")
			.into(),
		public_from_ss58::<sr25519::Public>("5GHJvRMyqSGnMSWoLgE9WSufoZZ6dBowdkDV4dvYApBykd9Z")
			.into(),
	];
	let multisig_controller_threshold: u16 = 3;

	let mut allocations = vec![(sudo_account.clone(), 100 * DOLLARS)];
	allocations.append(
		&mut multisig_controller_accounts
			.iter()
			.map(|a| (a.clone(), 100 * DOLLARS))
			.collect(),
	);
	allocations.append(&mut vec![(
		multisig_account(multisig_controller_accounts, multisig_controller_threshold),
		500 * DOLLARS,
	)]);
	testnet_genesis(
		wasm_binary,
		initial_authorities,
		sudo_account,
		allocations.clone(),
		committee_accounts,
		true,
	)
}

/// Configure initial storage state for FRAME modules.
fn testnet_genesis(
	wasm_binary: &[u8],
	initial_authorities: Vec<(AccountId, AccountId, AuraId, GrandpaId, ImOnlineId)>,
	root_key: AccountId,
	initial_token_allocation: Vec<(AccountId, Balance)>,
	committee_accounts: Vec<AccountId>,
	_enable_println: bool,
) -> GenesisConfig {
	let num_committee_members = committee_accounts.len();
	GenesisConfig {
		system: SystemConfig {
			// Add Wasm runtime to storage.
			code: wasm_binary.to_vec(),
		},
		balances: BalancesConfig { balances: initial_token_allocation },
		session: SessionConfig {
			keys: initial_authorities
				.iter()
				.map(|x| {
					(x.0.clone(), x.0.clone(), session_keys(x.2.clone(), x.3.clone(), x.4.clone()))
				})
				.collect::<Vec<_>>(),
		},
		aura: AuraConfig { authorities: vec![] },
		grandpa: GrandpaConfig { authorities: vec![] },
		validator_set: ValidatorSetConfig {
			initial_validators: initial_authorities.iter().map(|x| x.0.clone()).collect::<Vec<_>>(),
		},
		technical_membership: Default::default(),
		treasury: Default::default(),
		council: CouncilConfig {
			members: committee_accounts
				.iter()
				.take((num_committee_members + 1) / 2)
				.cloned()
				.collect(),
			phantom: Default::default(),
		},
		technical_committee: TechnicalCommitteeConfig {
			members: committee_accounts
				.iter()
				.take((num_committee_members + 1) / 2)
				.cloned()
				.collect(),
			phantom: Default::default(),
		},
		im_online: ImOnlineConfig { keys: vec![] },
		sudo: SudoConfig {
			// Assign network admin rights.
			key: Some(root_key),
		},
		transaction_payment: Default::default(),
	}
}
