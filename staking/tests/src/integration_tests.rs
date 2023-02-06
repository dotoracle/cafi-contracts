#[cfg(test)]
mod tests {
    use casper_engine_test_support::{
        ExecuteRequestBuilder, InMemoryWasmTestBuilder, DEFAULT_ACCOUNT_ADDR, DEFAULT_ACCOUNT_KEY,
        DEFAULT_RUN_GENESIS_REQUEST,
    };
    use casper_types::{runtime_args, ContractHash, RuntimeArgs, Key, CLValue, U256};

    const STAKING_CONTRACT: &str = "contract.wasm"; // The wasm name of staking contract
    const CONTRACT_KEY : &str = "this_is_staking";
    const CONTRACT_VERSION_KEY : &str = "contract_version";
    const TOTAL_ALLOC_POINT : &str = "total_alloc_point";
    const COUNTER_CALL_WASM: &str = "counter-call.wasm"; // Session code that calls the contract



    #[test]
    /// Install staking contract and checking its entry_points
    fn should_install_staking_contract() {
        let mut builder = InMemoryWasmTestBuilder::default();
        builder.run_genesis(&*DEFAULT_RUN_GENESIS_REQUEST).commit();

        // let mut runtime_args = RuntimeArgs::new();
        // runtime_args.insert_cl_value("contract_hash".to_string(), DEFAULT_ACCOUNT_KEY);
        // runtime_args.insert_cl_value("contract_owner", CLValue::from_t(DEFAULT_ACCOUNT_KEY).unwrap());
        // runtime_args.insert_cl_value("reward_token", CLValue::from_t(DEFAULT_ACCOUNT_KEY).unwrap());
        // runtime_args.insert_cl_value("reward_per_second", CLValue::from_t(1u64).unwrap());
        // runtime_args.insert_cl_value("start_second", CLValue::from_t(1u64).unwrap());


        // Install the contract.
        let contract_v1_installation_request = ExecuteRequestBuilder::standard(
            *DEFAULT_ACCOUNT_ADDR,
            STAKING_CONTRACT,
            runtime_args!{
                "staking_contract_name" => CONTRACT_KEY.to_string(),
                "contract_owner" => Key::Account(*DEFAULT_ACCOUNT_ADDR),
                "reward_token" => Key::Account(*DEFAULT_ACCOUNT_ADDR),
                "reward_per_second" => U256::from("1"),
                "start_second" => 1u64,
            },
        )
        .build();

        builder
            .exec(contract_v1_installation_request)
            .expect_success()
            .commit();

        // Check the contract hash.
        let contract_v1_hash = builder
            .get_expected_account(*DEFAULT_ACCOUNT_ADDR)
            .named_keys()
            .get(CONTRACT_KEY)
            .expect("must have contract hash key as part of contract creation")
            .into_hash()
            .map(ContractHash::new)
            .expect("must get contract hash");

        // // Verify the first contract version is 1.
        // let account = builder
        //     .get_account(*DEFAULT_ACCOUNT_ADDR)
        //     .expect("should have account");

        // let version_key = *account
        //     .named_keys()
        //     .get(CONTRACT_VERSION_KEY)
        //     .expect("version uref should exist");

        // let version = builder
        //     .query(None, version_key, &[])
        //     .expect("should be stored value.")
        //     .as_cl_value()
        //     .expect("should be cl value.")
        //     .clone()
        //     .into_t::<u32>()
        //     .expect("should be u32.");

        // assert_eq!(version, 1);

        // Verify the initial value of count is 0.
        let contract = builder
            .get_contract(contract_v1_hash)
            .expect("this contract should exist");

        let count_key = *contract
            .named_keys()
            .get(TOTAL_ALLOC_POINT)
            .expect("count uref should exist in the contract named keys");

        let count = builder
            .query(None, count_key, &[])
            .expect("should be stored value.")
            .as_cl_value()
            .expect("should be cl value.")
            .clone()
            .into_t::<u64>()
            .expect("should be u64.");

        assert_eq!(count, 0);

        // Use session code to increment the counter.
        let session_code_request = ExecuteRequestBuilder::standard(
            *DEFAULT_ACCOUNT_ADDR,
            COUNTER_CALL_WASM,
            runtime_args! {
                CONTRACT_KEY => contract_v1_hash
            },
        )
        .build();

        builder.exec(session_code_request).expect_success().commit();

        // // Verify the value of count is now 1.
        // let incremented_count = builder
        //     .query(None, count_key, &[])
        //     .expect("should be stored value.")
        //     .as_cl_value()
        //     .expect("should be cl value.")
        //     .clone()
        //     .into_t::<i32>()
        //     .expect("should be i32.");

        // assert_eq!(incremented_count, 1);

        // // Call the decrement entry point, which should not be in this version.
        // let contract_decrement_request = ExecuteRequestBuilder::contract_call_by_hash(
        //     *DEFAULT_ACCOUNT_ADDR,
        //     contract_v1_hash,
        //     ENTRY_POINT_COUNTER_DECREMENT,
        //     runtime_args! {},
        // )
        // .build();

        // // Try executing the decrement entry point and expect an error.
        // builder
        //     .exec(contract_decrement_request)
        //     .expect_failure()
        //     .commit();

        // // Ensure the count value was not decremented.
        // let current_count = builder
        //     .query(None, count_key, &[])
        //     .expect("should be stored value.")
        //     .as_cl_value()
        //     .expect("should be cl value.")
        //     .clone()
        //     .into_t::<i32>()
        //     .expect("should be i32.");

        // assert_eq!(current_count, 1);
    }
}

fn main() {
    panic!("Execute \"cargo test\" to test the contract, not \"cargo run\".");
}
