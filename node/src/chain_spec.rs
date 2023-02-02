use hex_literal::hex;
use node_primitives::*;
use serv_runtime::{
	constants::currency::*, opaque::SessionKeys, BabeConfig, BalancesConfig, CouncilConfig,
	DemocracyConfig, ElectionsConfig, GenesisConfig, GrandpaConfig, ImOnlineConfig, MaxNominations,
	SessionConfig, StakerStatus, StakingConfig, SudoConfig, SystemConfig, TechnicalCommitteeConfig,
	NominationPoolsConfig,
	BABE_GENESIS_EPOCH_CONFIG, wasm_binary_unwrap,
};
use pallet_im_online::sr25519::AuthorityId as ImOnlineId;
use sc_service::{ChainType, Properties};
use sc_telemetry::TelemetryEndpoints;
use sp_consensus_babe::AuthorityId as BabeId;
use sp_core::{crypto::UncheckedInto, sr25519, Pair, Public};
use sp_finality_grandpa::AuthorityId as GrandpaId;
use sp_runtime::{
	traits::{IdentifyAccount, Verify},
	Perbill,
};

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

const STAGING_TELEMETRY_URL: &str = "wss://telemetry.polkadot.io/submit/";
const DEFAULT_PROTOCOL_ID: &str = "zeno";

fn session_keys(babe: BabeId, grandpa: GrandpaId, im_online: ImOnlineId) -> SessionKeys {
	SessionKeys { babe, grandpa, im_online }
}


/// Generate an account ID from seed.
pub fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId
where
	AccountPublic: From<<TPublic::Pair as Pair>::Public>,
{
	AccountPublic::from(get_from_seed::<TPublic>(seed)).into_account()
}

/// Helper function to generate stash, controller and session key from seed
pub fn authority_keys_from_seed(s: &str) -> (AccountId, AccountId, BabeId, GrandpaId, ImOnlineId) {
	(
		get_account_id_from_seed::<sr25519::Public>(&format!("{}//stash", s)),
		get_account_id_from_seed::<sr25519::Public>(s),
		get_from_seed::<BabeId>(s),
		get_from_seed::<GrandpaId>(s),
		get_from_seed::<ImOnlineId>(s),
	)
}

/// Token
pub fn zeno_properties() -> Properties {
	let mut p = Properties::new();
	p.insert("ss58format".into(), 42.into());
	p.insert("tokenDecimals".into(), 10.into());
	p.insert("tokenSymbol".into(), "ZNO".into());
	p
}
pub fn development_config() -> Result<ChainSpec, String> {

	Ok(ChainSpec::from_genesis(
		// Name
		"Development",
		// ID
		"dev",
		ChainType::Development,
		move || {
			testnet_genesis(
				// Initial PoA authorities
				vec![authority_keys_from_seed("Alice")],
				vec![],
				// Sudo account
				// 5F8xghQeZTU6QLEwqxnEsNzpUjznAoa1AFJiPyYE396f6mJv
				get_account_id_from_seed::<sr25519::Public>("Alice"),
				// Pre-funded accounts
				vec![
					get_account_id_from_seed::<sr25519::Public>("Alice"),
					get_account_id_from_seed::<sr25519::Public>("Bob"),
					get_account_id_from_seed::<sr25519::Public>("Dave"),
					get_account_id_from_seed::<sr25519::Public>("Ferdie"),
					get_account_id_from_seed::<sr25519::Public>("Eve"),
					get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
					get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
				],
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
		Some(zeno_properties()),
		// Extensions
		None,
	))
}

pub fn local_testnet_config() -> Result<ChainSpec, String> {
	Ok(ChainSpec::from_genesis(
		// Name
		"Local Testnet",
		// ID
		"local_testnet",
		ChainType::Local,
		move || {
			testnet_genesis(
				// Initial PoA authorities
				vec![authority_keys_from_seed("Alice"), authority_keys_from_seed("Bob")],
				vec![],
				// Sudo account
				get_account_id_from_seed::<sr25519::Public>("Alice"),
				// Pre-funded accounts
				vec![
					"5GgZPqeWSnoVPRyy7ALAFPyU6Ws1o8pKBNtuMttAqkFRopR5".parse()
					.unwrap(),
					"5EEpSSiLWCKisQxoJFsLrR3MSzxBvREH5vofieHGQYtWMMLs".parse()
					.unwrap(),
					"5GHJvRMyqSGnMSWoLgE9WSufoZZ6dBowdkDV4dvYApBykd9Z".parse()
					.unwrap(),
					"5DNvfF8gjTys1ZAo5S9Wq2ZESLm1Ssudj8CC8Z4CDU67PsVZ".parse()
					.unwrap(),
					"5DRVZN78VKbgNny4bHf2MpmHq6SVhrT6g23ciTYHi36woLMT".parse()
					.unwrap(),
					"5CDrkPqy6KQDYNXNXiK5NMij1p7gNQuR2WB9My8y1fYvAspA".parse()
					.unwrap(),
				],
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
		"Zeno Testnet",
		"zeno_testnet",
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
		Default::default(),
	)
}

fn staging_network_config_genesis() -> GenesisConfig {
	// for i in 1 2 3 4; do for j in stash controller; do subkey inspect "$SECRET//$i//$j"; done; done
	// for i in 1 2 3 4; do for j in babe; do subkey --sr25519 inspect "$SECRET//$i//$j"; done; done
	// for i in 1 2 3 4; do for j in grandpa; do subkey --ed25519 inspect "$SECRET//$i//$j"; done; done
	// for i in 1 2 3 4; do for j in im_online; do subkey --sr25519 inspect "$SECRET//$i//$j"; done; done
	let initial_authorities: Vec<(AccountId, AccountId, BabeId, GrandpaId, ImOnlineId)> = vec![
		(
			// 5DWhheWDPzVFaBvqrrfSQrA9aFZppbsNnatRMook7FChCybQ
			hex!["401017229ff5388d262a3145e99fae36cd10432e84912e2675495fd9cab4c04d"].into(),
			// 5CFCr2DqTGrAyUSCEzSDFuGn4zsMappHUyBW4APBMCdFafJf
			hex!["08020a09173661c9b41affb513f5091214b19e53825bbe53d472fbd7d9ac0f3d"].into(),
			// 5FRLoFLM56PZSPKQ7AssQYLGb1oJBhQgPDAqDLw5sqiXHhrW
			hex!["9471f6879a0bb8129eb1e4a869a1d138ed17f14b5dc69a7c2c3c1ddafb61635d"].unchecked_into(),
			// 5Dmx5cYTZKzybz9d1hLZGtCgPVE1oZ2UpBAcnHmi8Seb5ZtQ
			hex!["4bb12caf3375637bc70193c25ec2a2479b0f1dd2ad0fb624e1b5b6778f67d2f1"].unchecked_into(),
			// 5FcEqxwHgh39oqJZvcDhXSKshpUenMiBCqyUaZXmv6JycTty
			hex!["9cc19f53450e5bda61816d9982cd8bb4bf453bdd3f6ae6f022298c827e7fac0f"].unchecked_into(),
		),
		(
			// 5GpRt7DNKUtj5XRYJTtzMnwM1jqPzXYEm6pSMG6dY74fNXZt
			hex!["d249f44f25ed1b2363245c59ea104a4b252b9cdc5a338ac2d139c7db9a5cff40"].into(),
			// 5FsXN8XXwrjb8TBK4sKDbF44WLyf41npGsPhKoDZmYNX5CJC
			hex!["a869ea3e9921811a708e912f724bb64af9fed54427209890c571746f221b497f"].into(),
			// 5G4U4aYiTPbwB9n5dhpxkE83yDJw5RXDMnn9whZuRBH2E9uZ
			hex!["b0c27f342aec278ac1f6bc21437119485a3938b687fc2ecc92aad096df05a975"].unchecked_into(),
			// 5CeZit9rqkV3pbQMdFWxEeAjD83CmYWGpzPYStQCbPNSw5rU
			hex!["19d2f621ae9690cbd9d37bc919a91b9aa8a072dc987a1fdcb8b9afcb65fac7fd"].unchecked_into(),
			// 5FsxicJpWnXFbegfARjEBe6PU2ESn6hBu6SeLNcgnTuJzt2V
			hex!["a8bf42d84432b080e93e948a1b4034fb2d3956aaadb9307aa7435e532ea2b56d"].unchecked_into(),
		),
		(
			// 5CqqPkoJcnk9TZJUAea8yPiDb6ZPvUL4hozikMysn45rxnkD
			hex!["226b695dd01733d0a3f5baa1dc6fe69bd6605d3a2c3efe3b0f22a9266c21a255"].into(),
			// 5HC6S6G64Jm1p9nZvtjzftLdTyMdw2Mqr25hwiFyJRFF1o1z
			hex!["e2cfe22fd5c21fee880b4813e7b8d11de860486ef60c916072d81210bae03c63"].into(),
			// 5HYHTPBNyqQb4jxScXNtBAdoK7jjHnhDd2DFpaav7HHgn5HX
			hex!["f235e559bc6edf38ea1c001f2d6bee8c13b0a61ac848428513af77295b933060"].unchecked_into(),
			// 5CWGZtXD1XR9Wyc9y2ZpdWsLKJNF6H2hKokLpCc8NBFbRC44
			hex!["137f3fc6c427872952cadf185d254f118985db4a0b2db51f3ebd10df77775cd1"].unchecked_into(),
			// 5GKcxEJitd9KSwx5CZVUdfrEEo32kiUZBm9vs39LhxVE6wkp
			hex!["bc511b2d7ce401fb975fbcb9f164a11942b3003fd75235983c2025c89e0ff319"].unchecked_into(),
		),
		(
			// 5DXjBVqPKAMRYoqEXZatfvwH22wyJETFG1wBCo55Jz7xLVR1
			hex!["40d85189f85ca2bd00d12f9aaacdf6786f4a81ef5f691bb1027c4993c3b21140"].into(),
			// 5CPA4DJzfhJJhU2C2nXZ9MBPHPj36ipeQmvmepsk58KBjHqo
			hex!["0e129c7197b7afcb4b938b4384d74ebe80f427b465d370f7b4f13a2cc4ca8c15"].into(),
			// 5H4ueMfsNdLXWByEgJmMMLCVeenUS3iiSW4AcQ5rqRiombPK
			hex!["dd54d92aadd95c1037de48054453b945dcd9e2da645bd2f1a6c0e620792f034b"].unchecked_into(),
			// 5F6Udepii6Gfhc4G3Mc17bZouhdXA8x14NuXAid5tig7n3Vj
			hex!["860eac80525b405dd1a36dfbe39abef0c7bf66e16b4e07280fc908b914a3f563"].unchecked_into(),
			// 5EJ7vzC5tWGmdS7erBKbhYM4Qs5qhd4SczERtQ6sA5A4H87z
			hex!["62b3b2f0c8e3b800adbc655ed604ef4e5869992f4af00f0bf1fbefb7ed1d655d"].unchecked_into(),
		),
	];

	// generated with secret: subkey inspect "$secret"/fir
	let root_key: AccountId = hex![
		// 5FemZuvaJ7wVy4S49X7Y9mj7FyTR4caQD5mZo2rL7MXQoXMi
		"9eaf896d76b55e04616ff1e1dce7fc5e4a417967c17264728b3fd8fee3b12f3c"
	]
	.into();

	let endowed_accounts: Vec<AccountId> = vec![
		"5GHJvRMyqSGnMSWoLgE9WSufoZZ6dBowdkDV4dvYApBykd9Z".parse()
		.unwrap(),
		"5DNvfF8gjTys1ZAo5S9Wq2ZESLm1Ssudj8CC8Z4CDU67PsVZ".parse()
		.unwrap(),
		"5DRVZN78VKbgNny4bHf2MpmHq6SVhrT6g23ciTYHi36woLMT".parse()
		.unwrap(),
		"5CDrkPqy6KQDYNXNXiK5NMij1p7gNQuR2WB9My8y1fYvAspA".parse()
		.unwrap(),
	];

	testnet_genesis(
		initial_authorities,
		vec![],
		root_key,
		endowed_accounts
	)
}

/// Configure initial storage state for FRAME modules.
fn testnet_genesis(
	initial_authorities: Vec<(AccountId, AccountId, BabeId, GrandpaId, ImOnlineId)>,
	initial_nominators: Vec<AccountId>,
	root_key: AccountId,
	mut endowed_accounts: Vec<AccountId>,
) -> GenesisConfig {
	// endow all authorities and nominators.
	initial_authorities
		.iter()
		.map(|x| &x.0)
		.chain(initial_nominators.iter())
		.for_each(|x| {
			if !endowed_accounts.contains(x) {
				endowed_accounts.push(x.clone())
			}
		});

	// stakers: all validators and nominators.
	const ENDOWMENT: Balance = 1_000 * DOLLARS;
	const STASH: Balance = ENDOWMENT / 1_000;
	
	let mut rng = rand::thread_rng();
	let stakers = initial_authorities
		.iter()
		.map(|x| (x.0.clone(), x.1.clone(), STASH, StakerStatus::Validator))
		.chain(initial_nominators.iter().map(|x| {
			use rand::{seq::SliceRandom, Rng};
			let limit = (MaxNominations::get() as usize).min(initial_authorities.len());
			let count = rng.gen::<usize>() % limit;
			let nominations = initial_authorities
				.as_slice()
				.choose_multiple(&mut rng, count)
				.into_iter()
				.map(|choice| choice.0.clone())
				.collect::<Vec<_>>();
			(x.clone(), x.clone(), STASH, StakerStatus::Nominator(nominations))
		}))
		.collect::<Vec<_>>();

	let num_endowed_accounts = endowed_accounts.len();

	GenesisConfig {
		system: SystemConfig {
			// Add Wasm runtime to storage.
			code: wasm_binary_unwrap().to_vec(),
		},
		balances: BalancesConfig {
			// Configure endowed accounts with initial balance of 1 << 60.
			balances: endowed_accounts.iter().cloned().map(|k| (k, ENDOWMENT)).collect(),
		},
		session: SessionConfig {
			keys: initial_authorities
				.iter()
				.map(|x| {
					(x.0.clone(), x.0.clone(), session_keys(x.2.clone(), x.3.clone(), x.4.clone()))
				})
				.collect::<Vec<_>>(),
		},
		staking: StakingConfig {
			validator_count: initial_authorities.len() as u32,
			minimum_validator_count: initial_authorities.len() as u32,
			invulnerables: initial_authorities.iter().map(|x| x.0.clone()).collect(),
			slash_reward_fraction: Perbill::from_percent(10),
			stakers,
			// TODO: ForceEra::ForceNone
			..Default::default()
		},
		babe: BabeConfig { authorities: vec![], epoch_config: Some(BABE_GENESIS_EPOCH_CONFIG) },
		grandpa: GrandpaConfig { authorities: vec![] },
		im_online: ImOnlineConfig { keys: vec![] },
		democracy: DemocracyConfig::default(),
		elections: ElectionsConfig {
			members: endowed_accounts
				.iter()
				.take((num_endowed_accounts + 1) / 2)
				.cloned()
				.map(|member| (member, STASH))
				.collect(),
		},
		council: CouncilConfig::default(),
		technical_committee: TechnicalCommitteeConfig {
			members: endowed_accounts
				.iter()
				.take((num_endowed_accounts + 1) / 2)
				.cloned()
				.collect(),
			phantom: Default::default(),
		},
		technical_membership: Default::default(),
		treasury: Default::default(),
		sudo: SudoConfig {
			// Assign network admin rights.
			key: Some(root_key),
		},
		transaction_payment: Default::default(),
		nomination_pools: NominationPoolsConfig {
			min_create_bond: 10 * DOLLARS,
			min_join_bond: 1 * DOLLARS,
			..Default::default()
		},
	}
}