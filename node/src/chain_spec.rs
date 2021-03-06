use sc_service::ChainType;
use sp_consensus_aura::sr25519::AuthorityId as AuraId;
use sp_core::{sr25519, Pair, Public};
use sp_finality_grandpa::AuthorityId as GrandpaId;
use sp_runtime::traits::{IdentifyAccount, Verify};
use suntime::{
    AccountId,
    AuraConfig,
    BalancesConfig,
    GenesisConfig,
    GrandpaConfig,
    IndicesConfig,
    OrgId,
    Share,
    ShareId,
    SharesAtomicConfig,
    Signature,
    SudoConfig,
    SystemConfig,
    WASM_BINARY, // Signal, VoteId
};

/// Specialized `ChainSpec`.
pub type ChainSpec = sc_service::GenericChainSpec<GenesisConfig>;

/// Helper function to generate a crypto pair from seed
pub fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
    TPublic::Pair::from_string(&format!("//{}", seed), None)
        .expect("static values are valid; qed")
        .public()
}

type AccountPublic = <Signature as Verify>::Signer;

/// Helper function to generate an account ID from seed
pub fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId
where
    AccountPublic: From<<TPublic::Pair as Pair>::Public>,
{
    AccountPublic::from(get_from_seed::<TPublic>(seed)).into_account()
}

/// Helper function to generate an authority key for Aura
pub fn get_authority_keys_from_seed(s: &str) -> (AuraId, GrandpaId) {
    (get_from_seed::<AuraId>(s), get_from_seed::<GrandpaId>(s))
}

pub fn development_config() -> ChainSpec {
    ChainSpec::from_genesis(
        "Development",
        "dev",
        ChainType::Development,
        || {
            testnet_genesis(
                // initial authorities
                vec![get_authority_keys_from_seed("Alice")],
                // root key
                get_account_id_from_seed::<sr25519::Public>("Alice"),
                // endowed accounts
                vec![
                    get_account_id_from_seed::<sr25519::Public>("Alice"),
                    get_account_id_from_seed::<sr25519::Public>("Bob"),
                    get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
                    get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
                ],
                // membership shares
                vec![
                    (
                        1,
                        1,
                        get_account_id_from_seed::<sr25519::Public>("Alice"),
                        10,
                    ),
                    (1, 1, get_account_id_from_seed::<sr25519::Public>("Bob"), 10),
                ],
                // total issuance
                vec![(1, 1, 20)],
                // shareholder membership
                vec![(
                    1,
                    1,
                    vec![
                        get_account_id_from_seed::<sr25519::Public>("Alice"),
                        get_account_id_from_seed::<sr25519::Public>("Bob"),
                    ],
                )],
                true,
            )
        },
        vec![],
        None,
        None,
        None,
        None,
    )
}

pub fn local_testnet_config() -> ChainSpec {
    ChainSpec::from_genesis(
        "Local Testnet",
        "local_testnet",
        ChainType::Local,
        || {
            testnet_genesis(
                // initial authorities
                vec![
                    get_authority_keys_from_seed("Alice"),
                    get_authority_keys_from_seed("Bob"),
                ],
                // root key
                get_account_id_from_seed::<sr25519::Public>("Alice"),
                // endowed accounts
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
                // membership shares
                vec![
                    (
                        1,
                        1,
                        get_account_id_from_seed::<sr25519::Public>("Alice"),
                        10,
                    ),
                    (1, 1, get_account_id_from_seed::<sr25519::Public>("Bob"), 10),
                    (
                        1,
                        1,
                        get_account_id_from_seed::<sr25519::Public>("Charlie"),
                        10,
                    ),
                    (
                        1,
                        1,
                        get_account_id_from_seed::<sr25519::Public>("Dave"),
                        10,
                    ),
                    (1, 1, get_account_id_from_seed::<sr25519::Public>("Eve"), 10),
                    (
                        1,
                        1,
                        get_account_id_from_seed::<sr25519::Public>("Ferdie"),
                        10,
                    ),
                ],
                // total issuance
                vec![(1, 1, 60)],
                // shareholder membership
                vec![(
                    1,
                    1,
                    vec![
                        get_account_id_from_seed::<sr25519::Public>("Alice"),
                        get_account_id_from_seed::<sr25519::Public>("Bob"),
                        get_account_id_from_seed::<sr25519::Public>("Charlie"),
                        get_account_id_from_seed::<sr25519::Public>("Dave"),
                        get_account_id_from_seed::<sr25519::Public>("Eve"),
                        get_account_id_from_seed::<sr25519::Public>("Ferdie"),
                    ],
                )],
                true,
            )
        },
        vec![],
        None,
        None,
        None,
        None,
    )
}

pub fn testnet_genesis(
    initial_authorities: Vec<(AuraId, GrandpaId)>,
    root_key: AccountId,
    endowed_accounts: Vec<AccountId>,
    membership_shares: Vec<(OrgId, ShareId, AccountId, Share)>,
    total_issuance: Vec<(OrgId, ShareId, Share)>,
    shareholder_membership: Vec<(OrgId, ShareId, Vec<AccountId>)>,
    _enable_println: bool,
) -> GenesisConfig {
    GenesisConfig {
        frame_system: Some(SystemConfig {
            code: WASM_BINARY.to_vec(),
            changes_trie_config: Default::default(),
        }),
        pallet_balances: Some(BalancesConfig {
            balances: endowed_accounts
                .iter()
                .cloned()
                .map(|k| (k, 1 << 60))
                .collect(),
        }),
        pallet_indices: Some(IndicesConfig { indices: vec![] }),
        shares_atomic: Some(SharesAtomicConfig {
            omnipotent_key: root_key.clone(),
            membership_shares,
            total_issuance,
            shareholder_membership,
        }),
        pallet_aura: Some(AuraConfig {
            authorities: initial_authorities.iter().map(|x| (x.0.clone())).collect(),
        }),
        pallet_grandpa: Some(GrandpaConfig {
            authorities: initial_authorities
                .iter()
                .map(|x| (x.1.clone(), 1))
                .collect(),
        }),
        pallet_sudo: Some(SudoConfig { key: root_key }),
    }
}
