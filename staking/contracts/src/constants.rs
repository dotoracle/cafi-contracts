//! Constants used by the Stake contract.

pub const MARKETPLACE_CONTRACT_KEY_NAME: &str = "marketplace_contract";

// Named keys
pub const FEE_TOKEN_KEY_NAME: &str = "contract_hash";
pub const CONTRACT_HASH_KEY_NAME: &str = "contract_hash";
pub const PUNK_MARKETPLACE_KEY_NAME: &str = "punk_marketplace_contract_hash";
pub const CONTRACT_OWNER_KEY_NAME: &str = "contract_owner";
pub const DEV : &str = "dev";
pub const ARG_DEV : &str = "dev";
pub const REWARD_TOKEN: &str = "reward_token";
pub const TOKEN_CONTRACT_LIST: &str = "token_contract_hash";
pub const TOKEN_CONTRACT_MAP: &str = "token_contract_map";
        
pub const REWARD_RATE_KEY_NAME: &str = "reward_rate";

pub const LAST_UPDATE_KEY_NAME: &str = "last_update_time";

pub const REWARD_PER_TOKEN_STORED_KEY_NAME: &str = "reward_per_token_stored";

pub const TOTAL_SUPPLY_KEY_NAME: &str = "total_supply";

// Dictionaries

pub const BALANCES_KEY_NAME: &str = "balances";

pub const REWARDS_KEY_NAME: &str = "rewards";

pub const USER_REWARD_PER_TOKEN_PAID_KEY_NAME: &str = "user_reward_per_token_paid";

// Entry points

pub const REQUEST_BRIDGE_ENTRY_POINT_NAME: &str = "request_bridge_nft";
pub const UNLOCK_NFT_ENTRY_POINT_NAME: &str = "unlock_nft";
pub const TRANSFER_OWNER_ENTRY_POINT_NAME: &str = "transfer_owner";
pub const INIT_ENTRY_POINT_NAME: &str = "init";

// Runtime argument names
pub const ARG_CONTRACT_HASH: &str = "contract_hash";
pub const ARG_FEE_TOKEN_HASH: &str = "contract_owner";
pub const ARG_RECEIVER_ADDRESS: &str = "receiver_address";
pub const ARG_REQUEST_ID: &str = "request_id";
pub const ARG_TO_CHAINID:  &str = "to_chainid";
pub const ARG_FROM_CHAINID:  &str = "from_chainid";
pub const ARG_LP_CONTRACT_HASH:  &str = "lp_contract_hash";
pub const ARG_IDENTIFIER_MODE:  &str = "identifier_mode";
pub const ARG_UNLOCK_ID:  &str = "unlock_id";
pub const ARG_CONTRACT_OWNER:  &str = "contract_owner";
pub const ARG_NFT_ENABLED:  &str = "nft_enabled";
pub const ARG_MINIMUM_OFFER: &str = "minimum_offer";
/// Name of named-key for `name`.
pub const NAME_KEY_NAME: &str = "name";
/// Name of named-key for `symbol`
pub const SYMBOL_KEY_NAME: &str = "symbol";
/// Name of named-key for `decimals`
pub const DECIMALS_KEY_NAME: &str = "decimals";
/// Name of named-key for `contract`
pub const ERC20_TOKEN_CONTRACT_KEY_NAME: &str = "erc20_token_contract";
/// Name of dictionary-key for `allowances`
pub const ALLOWANCES_KEY_NAME: &str = "allowances";
/// Name of named-key for `minter`
pub const MINTER_KEY_NAME: &str = "minter";

/// Name of `name` entry point.
pub const NAME_ENTRY_POINT_NAME: &str = "name";
/// Name of `symbol` entry point.
pub const SYMBOL_ENTRY_POINT_NAME: &str = "symbol";
/// Name of `decimals` entry point.
pub const DECIMALS_ENTRY_POINT_NAME: &str = "decimals";
/// Name of `balance_of` entry point.
pub const BALANCE_OF_ENTRY_POINT_NAME: &str = "balance_of";
/// Name of `transfer` entry point.
pub const TRANSFER_ENTRY_POINT_NAME: &str = "transfer";
/// Name of `mint` entry point.
pub const MINT_ENTRY_POINT_NAME: &str = "mint";
/// Name of `approve` entry point.
pub const APPROVE_ENTRY_POINT_NAME: &str = "approve";
/// Name of `allowance` entry point.
pub const ALLOWANCE_ENTRY_POINT_NAME: &str = "allowance";
/// Name of `transfer_from` entry point.
pub const TRANSFER_FROM_ENTRY_POINT_NAME: &str = "transfer_from";
/// Name of `total_supply` entry point.
pub const TOTAL_SUPPLY_ENTRY_POINT_NAME: &str = "total_supply";
/// Name of `minter` entry point.
pub const MINTER_ENTRY_POINT_NAME: &str = "mint";
/// Name of `burn` entry point.
pub const BURN_ENTRY_POINT_NAME: &str = "burn";
/// Name of `change_minter` entry point.
pub const CHANGE_MINTER_ENTRY_POINT_NAME: &str = "change_minter";

/// Name of `address` runtime argument.
pub const ADDRESS_RUNTIME_ARG_NAME: &str = "address";
/// Name of `owner` runtime argument.
pub const OWNER_RUNTIME_ARG_NAME: &str = "owner";
/// Name of `spender` runtime argument.
pub const SPENDER_RUNTIME_ARG_NAME: &str = "spender";
/// Name of `amount` runtime argument.
pub const ARG_AMOUNT: &str = "amount";
/// Name of `recipient` runtime argument.
pub const RECIPIENT_RUNTIME_ARG_NAME: &str = "recipient";
/// Name of `name` runtime argument.
pub const NAME_RUNTIME_ARG_NAME: &str = "name";
/// Name of `symbol` runtime argument.
pub const SYMBOL_RUNTIME_ARG_NAME: &str = "symbol";
/// Name of `decimals` runtime argument.
pub const DECIMALS_RUNTIME_ARG_NAME: &str = "decimals";
/// Name of `total_supply` runtime argument.
pub const TOTAL_SUPPLY_RUNTIME_ARG_NAME: &str = "total_supply";
/// Name of `minter` runtime argument.
pub const MINTER_RUNTIME_ARG_NAME: &str = "minter";

pub const ARG_COLLECTION_NAME: &str = "collection_name";
pub const ARG_COLLECTION_SYMBOL: &str = "collection_symbol";
pub const ARG_TOTAL_TOKEN_SUPPLY: &str = "total_token_supply";
pub const ARG_TOKEN_OWNER: &str = "token_owner";
pub const ARG_TARGET_KEY: &str = "target_key";
pub const ARG_SOURCE_KEY: &str = "source_key";

pub const ARG_RECEIPT_NAME: &str = "receipt_name";
pub const ARG_CONTRACT_WHITELIST: &str = "contract_whitelist";
pub const ARG_NFT_METADATA_KIND: &str = "nft_metadata_kind";
pub const ARG_METADATA_MUTABILITY: &str = "metadata_mutability";
pub const ARG_NFT_SUPPORTED_TOKEN :&str = "nft_supported_token";
pub const ARG_IS_WRAPPED_TOKEN:&str = "is_wrapped_token";
pub const ARG_TOKEN_ADDRESS :&str ="token_address";
pub const ARG_STAKE_DURATION: &str = "stake_duration";


pub const TOKEN_IDS: &str = "token_ids";
pub const NFT_SUPPPORTED_CONTRACTS: &str = "nft_supported_contracts";
pub const TOKEN_STAKE: &str = "token_stake";
pub const USER_INFO : &str = "user_info";
pub const POOL_INFO : &str = "pool_info";
pub const REWARD_PER_SECOND: &str = "reward_per_second";
pub const OPERATOR: &str = "operator";
pub const NUMBER_OF_MINTED_TOKENS: &str = "number_of_minted_tokens";
pub const INSTALLER: &str = "installer";
pub const JSON_SCHEMA: &str = "json_schema";
pub const METADATA_SCHEMA: &str = "metadata_schema";
pub const CONTRACT_NAME: &str = "nft_contract";
pub const HASH_KEY_NAME: &str = "nft_contract_package";
pub const ACCESS_KEY_NAME: &str = "nft_contract_package_access";
pub const CONTRACT_VERSION: &str = "contract_version";
pub const COLLECTION_NAME: &str = "collection_name";
pub const COLLECTION_SYMBOL: &str = "collection_symbol";
pub const TOTAL_TOKEN_SUPPLY: &str = "total_token_supply";
pub const OWNERSHIP_MODE: &str = "ownership_mode";
pub const NFT_KIND: &str = "nft_kind";
pub const ALLOW_MINTING: &str = "allow_minting";
pub const MINTING_MODE: &str = "minting_mode";
pub const HOLDER_MODE: &str = "holder_mode";
pub const WHITELIST_MODE: &str = "whitelist_mode";
pub const TOKEN_OWNERS: &str = "token_owners";
pub const TOKEN_ISSUERS: &str = "token_issuers";
pub const OWNED_TOKENS: &str = "owned_tokens";
pub const BURNT_TOKENS: &str = "burnt_tokens";
pub const TOKEN_COUNTS: &str = "balances";
pub const CONTRACT_WHITELIST: &str = "contract_whitelist";
pub const RECEIPT_NAME: &str = "receipt_name";
pub const NFT_METADATA_KIND: &str = "nft_metadata_kind";
pub const IDENTIFIER_MODE: &str = "identifier_mode";
pub const METADATA_MUTABILITY: &str = "metadata_mutability";
pub const METADATA_CUSTOM_VALIDATED: &str = "metadata_custom_validated";
pub const METADATA_CEP78: &str = "metadata_cep78";
pub const METADATA_NFT721: &str = "metadata_nft721";
pub const METADATA_RAW: &str = "metadata_raw";
pub const ARG_START_SECOND:&str = "start_second";
pub const ARG_NEW_DEV : &str = "new_dev";
pub const ARG_REWARD_TOKEN: &str = "reward_token";
pub const ARG_REWARD_PER_SECOND : &str = "reward_per_second";
pub const STAKING_CONTRACT_NAME : &str = "staking_contract_name";
pub const WCSPR_CONTRACT : &str = "wcspr_contract";

pub const SET_WRAPPED_TOKEN_ENTRY_POINT_NAME : &str="set_wrapped_token";
pub const TRANSFER_DEV_ENTRY_POINT_NAME :&str = "transfer_dev";
pub const ENTRY_POINT_INIT: &str = "init";
pub const ENTRY_POINT_SET_VARIABLES: &str = "set_variables";
pub const ENTRY_POINT_MINT: &str = "mint";
pub const ENTRY_POINT_BURN: &str = "burn";
pub const ENTRY_POINT_TRANSFER: &str = "transfer";
pub const ENTRY_POINT_APPROVE: &str = "approve";
pub const ENTRY_POINT_BALANCE_OF: &str = "balance_of";
pub const ENTRY_POINT_OWNER_OF: &str = "owner_of";
pub const ENTRY_POINT_GET_APPROVED: &str = "get_approved";
pub const ENTRY_POINT_METADATA: &str = "metadata";
pub const ENTRY_POINT_GET_CREATOR : &str = "get_creator";
pub const CHANGE_OFFER_ENTRY_POINT_NAME : &str = "change_offer";
pub const ENTRY_POINT_SET_APPROVE_FOR_ALL: &str = "set_approval_for_all";
pub const ENTRY_POINT_SET_TOKEN_METADATA: &str = "set_token_metadata";
pub const OFFER_ENTRY_POINT_NAME : &str = "offer";
pub const BID_ENTRY_POINT_NAME : &str = "bid";
pub const INCREASE_BID_ENTRY_POINT_NAME : &str = "increase_bid";
pub const SET_REWARD_PER_SECOND : &str = "set_reward_per_second";
pub const CHANGE_WCSPR_CONTRACT_ENTRY_POINT_NAME : &str = "change_wcspr_contract";
pub const CHANGE_IS_ROYALTY_ENTRY_POINT_NAME : &str = "change_is_royalty";
pub const CHANGE_FEE_ENTRY_POINT_NAME : &str = "change_fee";
pub const REVOKE_OFFER_ENTRY_POINT_NAME : &str = "revoke_offer";
pub const REVOKE_BID_ENTRY_POINT_NAME : &str = "revoke_bid";
pub const SET_REWARD_TOKEN : &str = "set_reward_token";
pub const ADD_NEW_POOL : &str = "add_new_pool";
pub const STAKE : &str = "stake";
pub const UN_STAKE : &str = "un_stake";
pub const FEE_PORTION : &str = "fee_portion";
pub const GET_APPROVED_ENTRY_POINT_NAME : &str = "get_approved";
pub const ARG_BIDDING_OFFER : &str = "bidding_offer";
pub const ARG_SRC_PURSE : &str = "src_purse";
pub const CONTRACT_PURSE : &str = "contract_purse";
pub const ARG_NEW_OFFER : &str = "new_offer";
pub const ARG_WCSPR_CONTRACT : &str = "wcspr_contract";
pub const ARG_NEW_MINIMUM_OFFER : &str = "new_minimum_offer";
pub const ARG_ROYALTY_FEE : &str = "royalty_fee";
pub const START_BLOCK : &str = "start_block";
pub const ARG_IS_ROYALTY : &str = "is_royalty";
pub const IS_ROYALTY : &str = "is_royalty";
pub const ARG_POOL_INFO : &str = "pool_info";
pub const ARG_TOTAL_WEIGHT : &str = "total_weight";
pub const ARG_ALLOC_POINT : &str = "alloc_point";
pub const ARG_LAST_REWARD_SECOND : &str = "last_reward_second";
pub const ARG_ACC_REWARD_PER_SHARE : &str = "acc_reward_per_share";
pub const ARG_MIN_STAKE_DURATION : &str = "min_stake_duration";
pub const ARG_PENALTY_RATE : &str = "penalty_rate";
pub const ARG_USER : &str = "user";
pub const POOL_LIST : &str = "pool_list";
pub const ARG_POOL_ID : &str = "pool_id";
pub const TOTAL_ALLOC_POINT : &str = "total_alloc_point";