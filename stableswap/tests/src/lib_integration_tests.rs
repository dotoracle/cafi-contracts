use casper_engine_test_support::{
    ExecuteRequestBuilder, InMemoryWasmTestBuilder, DEFAULT_RUN_GENESIS_REQUEST,
    DEFAULT_ACCOUNT_ADDR, MINIMUM_ACCOUNT_CREATION_BALANCE,
};

use casper_types::{
    account::AccountHash, bytesrepr::FromBytes, CLTyped, runtime_args, system::mint,
    ContractHash, ContractPackageHash, Key, PublicKey, RuntimeArgs, crypto::SecretKey, U256, U128
};
use std::convert::TryInto;
const EXAMPLE_ERC20_TOKEN: &str = "erc20_token.wasm";
const TEST_SESSION: &str = "test-session.wasm";
const DEX_CONTRACT: &str = "contract.wasm";
const ERC20_TOKEN_CONTRACT_KEY: &str = "erc20_token_contract";
const ARG_NAME: &str = "name";
const ARG_SYMBOL: &str = "symbol";
const ARG_DECIMALS: &str = "decimals";
const ARG_TOTAL_SUPPLY: &str = "total_supply";
const ARG_NEW_MINTER: &str = "new_minter";
const RESULT_KEY: &str = "result";
const TOKEN_TOTAL_SUPPLY: u128 = 1_000_000_000_000_000_000_000_000_000;


pub const STABLESWAP_CONTRACT_NAME : &str = "stableswap_contract_name";
pub const ARG_POOLED_TOKENS : &str = "pooled_tokens";
pub const ARG_LP_TOKEN : &str = "lp_token";
pub const ARG_A : &str = "a";
pub const ARG_FEE : &str = "fee";
pub const ARG_ADMIN_FEE : &str = "admin_fee";
pub const ARG_CONTRACT_OWNER:  &str = "contract_owner";

fn get_token_key_name(symbol: String) -> String {
    ERC20_TOKEN_CONTRACT_KEY.to_owned() + "_" + &symbol
}

fn get_account1_addr() -> AccountHash {
    let sk: SecretKey = SecretKey::secp256k1_from_bytes(&[221u8; 32]).unwrap();
    let pk: PublicKey = PublicKey::from(&sk);
    let a: AccountHash = pk.to_account_hash();
    a
}

fn get_account2_addr() -> AccountHash {
    let sk: SecretKey = SecretKey::secp256k1_from_bytes(&[212u8; 32]).unwrap();
    let pk: PublicKey = PublicKey::from(&sk);
    let a: AccountHash = pk.to_account_hash();
    a
}

fn get_test_result<T: FromBytes + CLTyped>(
    builder: &mut InMemoryWasmTestBuilder,
    test_session: ContractPackageHash,
) -> T {
    let contract_package = builder
        .get_contract_package(test_session)
        .expect("should have contract package");
    let enabled_versions = contract_package.enabled_versions();
    let (_version, contract_hash) = enabled_versions
        .iter()
        .rev()
        .next()
        .expect("should have latest version");

    builder.get_value(*contract_hash, RESULT_KEY)
}

fn call_and_get<T: FromBytes + CLTyped>(
    builder: &mut InMemoryWasmTestBuilder,
    test_session: ContractPackageHash,
    func_name: &str,
    args: RuntimeArgs
) -> T {
    let exec_request = ExecuteRequestBuilder::versioned_contract_call_by_hash(
        *DEFAULT_ACCOUNT_ADDR,
        test_session,
        None,
        func_name,
        args,
    )
    .build();
    builder.exec(exec_request).expect_success().commit();

    get_test_result(builder, test_session)
}

/// Converts hash addr of Account into Hash, and Hash into Account
///
/// This is useful for making sure ERC20 library respects different variants of Key when storing
/// balances.
// fn invert_erc20_address(address: Key) -> Key {
//     match address {
//         Key::Account(account_hash) => Key::Hash(account_hash.value()),
//         Key::Hash(contract_hash) => Key::Account(AccountHash::new(contract_hash)),
//         _ => panic!("Unsupported Key variant"),
//     }
// }

#[derive(Copy, Clone)]
struct TestContext {
    usdc_token: ContractHash,
    usdt_token: ContractHash,
    // dai_token: ContractHash,
    dex_contract: ContractHash,
    lp_token: ContractHash,
    dex_contract_package_hash: ContractPackageHash,
    test_session: ContractPackageHash
}

fn exec_call(builder: &mut InMemoryWasmTestBuilder, account_hash: AccountHash, contract_hash: ContractHash, fun_name: &str, args: RuntimeArgs, expect_success: bool) {
    let request = ExecuteRequestBuilder::contract_call_by_hash(
        account_hash,
        contract_hash,
        fun_name,
        args
    ).build();
    if expect_success {
        builder.exec(request).expect_success().commit();
    } else {
        builder.exec(request).expect_failure();
    }
}

fn setup() -> (InMemoryWasmTestBuilder, TestContext) {
    let mut builder = InMemoryWasmTestBuilder::default();
    builder.run_genesis(&*DEFAULT_RUN_GENESIS_REQUEST);

    let id: Option<u64> = None;
    let transfer_1_args = runtime_args! {
        mint::ARG_TARGET => get_account1_addr(),
        mint::ARG_AMOUNT => MINIMUM_ACCOUNT_CREATION_BALANCE,
        mint::ARG_ID => id,
    };
    let transfer_2_args = runtime_args! {
        mint::ARG_TARGET => get_account2_addr(),
        mint::ARG_AMOUNT => MINIMUM_ACCOUNT_CREATION_BALANCE,
        mint::ARG_ID => id,
    };

    let transfer_request_1 =
        ExecuteRequestBuilder::transfer(*DEFAULT_ACCOUNT_ADDR, transfer_1_args).build();
    let transfer_request_2 =
        ExecuteRequestBuilder::transfer(*DEFAULT_ACCOUNT_ADDR, transfer_2_args).build();

    let install_request_1 = ExecuteRequestBuilder::standard(
        *DEFAULT_ACCOUNT_ADDR,
        EXAMPLE_ERC20_TOKEN,
        runtime_args! {
            ARG_NAME => "USDC Faucet".to_string(),
            ARG_SYMBOL => "USDC".to_string(),
            ARG_DECIMALS => 18u8,
            ARG_TOTAL_SUPPLY => U256::from(TOKEN_TOTAL_SUPPLY),
            ARG_NEW_MINTER => Key::from(*DEFAULT_ACCOUNT_ADDR)
        },
    )
    .build();
    let install_request_2 = ExecuteRequestBuilder::standard(
        *DEFAULT_ACCOUNT_ADDR,
        EXAMPLE_ERC20_TOKEN,
        runtime_args! {
            ARG_NAME => "USDT Faucet",
            ARG_SYMBOL => "USDT",
            ARG_DECIMALS => 18u8,
            ARG_TOTAL_SUPPLY => U256::from(TOKEN_TOTAL_SUPPLY),
            ARG_NEW_MINTER => Key::from(*DEFAULT_ACCOUNT_ADDR)
        },
    )
    .build();
    let install_request_3 = ExecuteRequestBuilder::standard(
        *DEFAULT_ACCOUNT_ADDR,
        EXAMPLE_ERC20_TOKEN,
        runtime_args! {
            ARG_NAME => "DAI Faucet",
            ARG_SYMBOL => "DAI",
            ARG_DECIMALS => 18u8,
            ARG_TOTAL_SUPPLY => U256::from(TOKEN_TOTAL_SUPPLY),
            ARG_NEW_MINTER => Key::from(*DEFAULT_ACCOUNT_ADDR)
        },
    )
    .build();

    let install_test_session = ExecuteRequestBuilder::standard(
        *DEFAULT_ACCOUNT_ADDR,
        TEST_SESSION,
        runtime_args! {}
    )
    .build();

    builder.exec(transfer_request_1).expect_success().commit();
    builder.exec(transfer_request_2).expect_success().commit();
    builder.exec(install_request_1).expect_success().commit();
    builder.exec(install_request_2).expect_success().commit();
    builder.exec(install_request_3).expect_success().commit();
    builder.exec(install_test_session).expect_success().commit();

    let account = builder
        .get_account(*DEFAULT_ACCOUNT_ADDR)
        .expect("should have account");

    let usdc_token = account
        .named_keys()
        .get(&get_token_key_name("USDC".to_string()))
        .and_then(|key| key.into_hash())
        .map(ContractHash::new)
        .expect("should have contract hash");

    let usdt_token = account
        .named_keys()
        .get(&get_token_key_name("USDT".to_string()))
        .and_then(|key| key.into_hash())
        .map(ContractHash::new)
        .expect("should have contract hash");

    exec_call(&mut builder, *DEFAULT_ACCOUNT_ADDR, usdc_token, "transfer", runtime_args! {
        "recipient" => Key::from(get_account1_addr()),
        "amount" => U256::from(1_000_000_000_000_000_000_000_000u128)
    }, true);
    exec_call(&mut builder, *DEFAULT_ACCOUNT_ADDR, usdt_token, "transfer", runtime_args! {
        "recipient" => Key::from(get_account1_addr()),
        "amount" => U256::from(1_000_000_000_000_000_000_000_000u128)
    }, true);

    exec_call(&mut builder, *DEFAULT_ACCOUNT_ADDR, usdc_token, "transfer", runtime_args! {
        "recipient" => Key::from(get_account2_addr()),
        "amount" => U256::from(1_000_000_000_000_000_000_000_000u128)
    }, true);
    exec_call(&mut builder, *DEFAULT_ACCOUNT_ADDR, usdt_token, "transfer", runtime_args! {
        "recipient" => Key::from(get_account2_addr()),
        "amount" => U256::from(1_000_000_000_000_000_000_000_000u128)
    }, true);


    let test_session = account
        .named_keys()
        .get("test_session")
        .and_then(|key| key.into_hash())
        .map(ContractPackageHash::new)
        .expect("should have contract hash");

    // let dai_token = account
    //     .named_keys()
    //     .get(&get_token_key_name("DAI".to_string()))
    //     .and_then(|key| key.into_hash())
    //     .map(ContractHash::new)
    //     .expect("should have contract hash");

    let null_hash: [u8; 32] = vec![0u8; 32].try_into().unwrap();
    let install_dex_contract = ExecuteRequestBuilder::standard(
        *DEFAULT_ACCOUNT_ADDR,
        DEX_CONTRACT,
        runtime_args! {
            STABLESWAP_CONTRACT_NAME => "FatPandaDEX",
            ARG_POOLED_TOKENS => vec![Key::from(usdc_token), Key::from(usdt_token)],
            ARG_LP_TOKEN => Key::from(ContractHash::new(null_hash)),
            ARG_A => U128::from(50),
            ARG_FEE => 10_000_000u64,
            ARG_ADMIN_FEE => 0u64,
            ARG_CONTRACT_OWNER => Key::from(*DEFAULT_ACCOUNT_ADDR)
        },
    )
    .build();
    builder.exec(install_dex_contract).expect_success().commit();

    let account = builder
        .get_account(*DEFAULT_ACCOUNT_ADDR)
        .expect("should have account");

    let dex_contract = account
        .named_keys()
        .get(&"FatPandaDEX".to_string())
        .and_then(|key| key.into_hash())
        .map(ContractHash::new)
        .expect("should have contract hash");

    let dex_contract_package_hash = account
        .named_keys()
        .get(&"FatPandaDEX_package_name".to_string())
        .and_then(|key| key.into_hash())
        .map(ContractPackageHash::new)
        .expect("should have contract package hash");

    let install_request_lp = ExecuteRequestBuilder::standard(
        *DEFAULT_ACCOUNT_ADDR,
        EXAMPLE_ERC20_TOKEN,
        runtime_args! {
            ARG_NAME => "LP Fat Panda Token Faucet",
            ARG_SYMBOL => "LP_FP",
            ARG_DECIMALS => 18u8,
            ARG_TOTAL_SUPPLY => U256::from(0),
            ARG_NEW_MINTER => Key::from(dex_contract_package_hash)
        },
    )
    .build();
    builder.exec(install_request_lp).expect_success().commit();

    let account = builder
        .get_account(*DEFAULT_ACCOUNT_ADDR)
        .expect("should have account");

    let lp_token = account
        .named_keys()
        .get(&get_token_key_name("LP_FP".to_string()))
        .and_then(|key| key.into_hash())
        .map(ContractHash::new)
        .expect("should have contract hash");

    // update lp
    exec_call(&mut builder, *DEFAULT_ACCOUNT_ADDR, dex_contract, "update_lp", runtime_args! {
        "lp_token" => Key::from(lp_token)
    }, true);

    let tc = TestContext {
        usdc_token,
        usdt_token,
        dex_contract,
        dex_contract_package_hash,
        lp_token,
        test_session
    };

    let tokens: Vec<ContractHash> = vec![tc.usdc_token, tc.usdt_token];
    for token in tokens {
        exec_call(&mut builder, *DEFAULT_ACCOUNT_ADDR, token, "approve", runtime_args! {
            "spender" => Key::from(tc.dex_contract_package_hash),
            "amount" => U256::from(TOKEN_TOTAL_SUPPLY)
        }, true);

        exec_call(&mut builder, get_account1_addr(), token, "approve", runtime_args! {
            "spender" => Key::from(tc.dex_contract_package_hash),
            "amount" => U256::from(TOKEN_TOTAL_SUPPLY)
        }, true);

        exec_call(&mut builder, get_account2_addr(), token, "approve", runtime_args! {
            "spender" => Key::from(tc.dex_contract_package_hash),
            "amount" => U256::from(TOKEN_TOTAL_SUPPLY)
        }, true);
    }

    // calling add liquidity
    exec_call(&mut builder, *DEFAULT_ACCOUNT_ADDR, tc.dex_contract, "add_liquidity", runtime_args! {
        "amounts" => vec![U128::from(1_000_000_000_000_000_000u128), U128::from(1_000_000_000_000_000_000u128)],
        "min_to_mint" => U128::from(0),
        "deadline" => 99999999999999u64
    }, true);
    println!("gas add_liquidity {:?}", builder.last_exec_gas_cost());

    (builder, tc)
}

#[test]
fn test_add_liquidity_revert_if_contract_paused() {
    let (mut builder, tc) = setup();

    exec_call(&mut builder, *DEFAULT_ACCOUNT_ADDR, tc.dex_contract, "set_paused", runtime_args! {
        "paused" => true
    }, true);

    exec_call(&mut builder, *DEFAULT_ACCOUNT_ADDR, tc.dex_contract, "add_liquidity", runtime_args! {
        "amounts" => vec![U128::from(1_000_000_000_000_000_000u128), U128::from(1_000_000_000_000_000_000u128)],
        "min_to_mint" => U128::from(0),
        "deadline" => 99999999999999u64
    }, false);

    exec_call(&mut builder, *DEFAULT_ACCOUNT_ADDR, tc.dex_contract, "set_paused", runtime_args! {
        "paused" => false
    }, true);

    exec_call(&mut builder, *DEFAULT_ACCOUNT_ADDR, tc.dex_contract, "add_liquidity", runtime_args! {
        "amounts" => vec![U128::from(1_000_000_000_000_000_000u128), U128::from(3_000_000_000_000_000_000u128)],
        "min_to_mint" => U128::from(0),
        "deadline" => 99999999999999u64
    }, true);
}

#[test]
fn test_add_liquidity_revert_with_amounts_must_match_pooled_tokens() {
    let (mut builder, tc) = setup();

    exec_call(&mut builder, *DEFAULT_ACCOUNT_ADDR, tc.dex_contract, "add_liquidity", runtime_args! {
        "amounts" => vec![U128::from(10_000_000_000_000_000u128)],
        "min_to_mint" => U128::from(0),
        "deadline" => 99999999999999u64
    }, false);
}

#[test]
fn test_add_liquidity_revert_cannot_withdraw_more_than_available() {
    let (mut builder, tc) = setup();

    exec_call(&mut builder, *DEFAULT_ACCOUNT_ADDR, tc.dex_contract, "calculate_token_amount", runtime_args! {
        "amounts" => vec![U128::from(10_000_000_000_000_000_000_000_000_000u128), U128::from(3_000_000_000_000_000_000u128)],
        "deposit" => false
    }, false);
}

#[test]
fn test_add_liquidity_revert_must_supply_all_tokens_inpool() {
    let (mut builder, tc) = setup();

    exec_call(&mut builder, *DEFAULT_ACCOUNT_ADDR, tc.dex_contract, "add_liquidity", runtime_args! {
        "amounts" => vec![U128::from(1_000_000_000_000_000_000u128), U128::from(3_000_000_000_000_000_000u128)],
        "min_to_mint" => U128::max_value(),
        "deadline" => 99999999999999u64
    }, false);
}

#[test]
fn test_add_liquidity_success_with_expected_output_amount() {
    let (mut builder, tc) = setup();
    let calculated_token_amount: U128 = call_and_get(
        &mut builder,
        tc.test_session,
        "calculate_token_amount",
        runtime_args! {
            "contract_hash" => tc.dex_contract,
            "amounts" => vec![U128::from(1_000_000_000_000_000_000u128), U128::from(3_000_000_000_000_000_000u128)],
            "deposit" => true
        }
    );

    let calculated_token_amount_with_slippage = calculated_token_amount * 999 / 1000;

    exec_call(&mut builder, get_account1_addr(), tc.dex_contract, "add_liquidity", runtime_args! {
        "amounts" => vec![U128::from(1_000_000_000_000_000_000u128), U128::from(3_000_000_000_000_000_000u128)],
        "min_to_mint" => calculated_token_amount_with_slippage,
        "deadline" => 99999999999999u64
    }, true);

    let actual_pool_token_amount: U128 = call_and_get(
        &mut builder,
        tc.test_session,
        "get_balance",
        runtime_args! {
            "contract_hash" => tc.lp_token,
            "address" => Key::from(get_account1_addr())
        }
    );

    assert_eq!(actual_pool_token_amount.as_u128(), 3991672211258372957u128);
}


#[test]
fn test_add_liquidity_success_with_actual_output_amount_within_range() {
    let (mut builder, tc) = setup();
    let calculated_token_amount: U128 = call_and_get(
        &mut builder,
        tc.test_session,
        "calculate_token_amount",
        runtime_args! {
            "contract_hash" => tc.dex_contract,
            "amounts" => vec![U128::from(1_000_000_000_000_000_000u128), U128::from(3_000_000_000_000_000_000u128)],
            "deposit" => true
        }
    );

    let calculated_token_amount_with_negative_slippage: U128 = calculated_token_amount * 999 / 1000;
    let calculated_token_amount_with_positive_slippage: U128 = calculated_token_amount * 1001 / 1000;


    exec_call(&mut builder, get_account1_addr(), tc.dex_contract, "add_liquidity", runtime_args! {
        "amounts" => vec![U128::from(1_000_000_000_000_000_000u128), U128::from(3_000_000_000_000_000_000u128)],
        "min_to_mint" => calculated_token_amount_with_negative_slippage,
        "deadline" => 99999999999999u64
    }, true);

    let actual_pool_token_amount: U128 = call_and_get(
        &mut builder,
        tc.test_session,
        "get_balance",
        runtime_args! {
            "contract_hash" => tc.lp_token,
            "address" => Key::from(get_account1_addr())
        }
    );

    assert!(actual_pool_token_amount.as_u128() >= calculated_token_amount_with_negative_slippage.as_u128());
    assert!(actual_pool_token_amount.as_u128() <= calculated_token_amount_with_positive_slippage.as_u128());
}


