const {
  utils,
  helpers,
  CasperContractClient,
} = require("casper-js-client-helper");

const {
  CLValueBuilder,
  CLPublicKey,
  CLKey,
  CLString,
  CasperClient,
  CLByteArray,
  RuntimeArgs,
  CLAccountHash,
  DeployUtil,
  Keys,
  CLTypeBuilder,
} = require("casper-js-sdk");
const { DEFAULT_TTL } = require("casper-js-client-helper/dist/constants");

const { setClient, contractSimpleGetter, createRecipientAddress } = helpers;

const sleep = (ms) => {
  return new Promise((resolve) => setTimeout(resolve, ms));
};

const getDeploy = async (NODE_URL, deployHash) => {
  const client = new CasperClient(NODE_URL);
  let i = 300;
  while (i != 0) {
    const [deploy, raw] = await client.getDeploy(deployHash);
    if (raw.execution_results.length !== 0) {
      // @ts-ignore
      if (raw.execution_results[0].result.Success) {
        return deploy;
      } else {
        // @ts-ignore
        throw Error(
          "Contract execution: " +
          // @ts-ignore
          raw.execution_results[0].result.Failure.error_message
        );
      }
    } else {
      i--;
      await sleep(1000);
      continue;
    }
  }
  throw Error("Timeout after " + i + "s. Something's wrong");
};

const genRanHex = (size = 64) =>
  [...Array(size)]
    .map(() => Math.floor(Math.random() * 16).toString(16))
    .join("");

const Staking = class {
  constructor(contractHash, nodeAddress, chainName, namedKeysList = []) {
    this.contractHash = contractHash.startsWith("hash-")
      ? contractHash.slice(5)
      : contractHash;
    this.nodeAddress = nodeAddress;
    this.chainName = chainName;
    this.contractClient = new CasperContractClient(nodeAddress, chainName);
    this.namedKeysList = [
      "contract_hash",
      "contract_owner",
      "pool_info",
      "pool_list",
      "reward_per_second",
      "user_info",
      "reward_token",
      "total_alloc_point",
      "start_block",
    ];
    this.namedKeysList.push(...namedKeysList)

  }

  static async createInstance(contractHash, nodeAddress, chainName, namedKeysList = []) {
    let staking = new Staking(contractHash, nodeAddress, chainName, namedKeysList);
    await staking.init();
    // console.log("NameKey: ", staking.namedKeys)
    return staking;
  }

  async init() {
    console.log("intializing", this.nodeAddress, this.contractHash);
    const { contractPackageHash, namedKeys } = await setClient(
      this.nodeAddress,
      this.contractHash,
      this.namedKeysList
    );
    console.log("done");
    this.contractPackageHash = contractPackageHash;
    this.contractClient.chainName = this.chainName;
    this.contractClient.contractHash = this.contractHash;
    this.contractClient.contractPackageHash = this.contractPackageHash;
    this.contractClient.nodeAddress = this.nodeAddress;
    /* @ts-ignore */
    this.namedKeys = namedKeys;
  }

  async contractOwner() {
    return await contractSimpleGetter(this.nodeAddress, this.contractHash, [
      "contract_owner"
    ]);
  }

  async requestIndex() {
    return await contractSimpleGetter(this.nodeAddress, this.contractHash, [
      "request_index",
    ]);
  }

  async userInfo(keyToCheck) {
    try {
        const result = await utils.contractDictionaryGetter(
          this.nodeAddress,
          keyToCheck,
          this.namedKeys.userInfo
        );
        return result.toString();
      } catch (e) {
        if (e.toString().includes("Failed to find base key at path")) {
          return "0";
        }
        throw e;
      }

}

  async getIndexFromRequestId(requestId) {
    try {
      const itemKey = requestId.toString();
      const result = await utils.contractDictionaryGetter(
        this.nodeAddress,
        itemKey,
        this.namedKeys.requestIds
      );
      return result;
    } catch (e) {
      throw e;
    }
  }

  async addNewPool({
    keys,
    lpContractHash, // contract LP
    allocPoint, // alloc point
    accRewardPerShare,
    minStakeDuration,
    penaltyRate,
    lastRewardSecond,
    paymentAmount,
    ttl,
  }) {

    if (!paymentAmount) {
      paymentAmount = paymentAmount ? paymentAmount : "10000000000";
      ttl = ttl ? ttl : DEFAULT_TTL;
    }
    lpContractHash = new CLByteArray(
      Uint8Array.from(Buffer.from(lpContractHash, "hex"))
    );
    let runtimeArgs = {};
    runtimeArgs = RuntimeArgs.fromMap({
      alloc_point: CLValueBuilder.u64(allocPoint),
      lp_contract_hash: createRecipientAddress(lpContractHash),
      acc_reward_per_share: CLValueBuilder.u256(accRewardPerShare),
      min_stake_duration: CLValueBuilder.u256(minStakeDuration),
      penalty_rate : CLValueBuilder.u256(penaltyRate),
      last_reward_second : CLValueBuilder.u64(lastRewardSecond),
    })
    console.log("sending");
    console.log(paymentAmount)
    console.log(ttl)
    let trial = 5;
    while (true) {
      try {
        let hash = await this.contractClient.contractCall({
          entryPoint: "add_new_pool",
          keys: keys,
          paymentAmount,
          runtimeArgs,
          cb: (deployHash) => {
            console.log("deployHash", deployHash);
          },
          ttl,
        });

        return hash;
      } catch (e) {
        trial--
        if (trial == 0) {
          throw e;
        }
        console.log('waiting 3 seconds')
        await sleep(3000)
      }
    }
  }

  async stake({
    keys,
    poolId, // contract LP
    amount, // alloc point
    stakeDuration,
    paymentAmount,
    ttl,
  }) {

    if (!paymentAmount) {
      paymentAmount = paymentAmount ? paymentAmount : "50000000000";
      ttl = ttl ? ttl : DEFAULT_TTL;
    }
    let runtimeArgs = {};
    runtimeArgs = RuntimeArgs.fromMap({
      pool_id: CLValueBuilder.u64(poolId),
      amount : CLValueBuilder.u256(amount),
      stake_duration : CLValueBuilder.u256(stakeDuration),
    })
    console.log("sending");
    console.log(paymentAmount)
    console.log(ttl)
    let trial = 5;
    while (true) {
      try {
        let hash = await this.contractClient.contractCall({
          entryPoint: "stake",
          keys: keys,
          paymentAmount,
          runtimeArgs,
          cb: (deployHash) => {
            console.log("deployHash", deployHash);
          },
          ttl,
        });

        return hash;
      } catch (e) {
        trial--
        if (trial == 0) {
          throw e;
        }
        console.log('waiting 3 seconds')
        await sleep(3000)
      }
    }
  }

  async unStake({
    keys,
    poolId, // contract LP
    amount, // alloc point
    paymentAmount,
    ttl,
  }) {

    if (!paymentAmount) {
      paymentAmount = paymentAmount ? paymentAmount : "100000000000";
      ttl = ttl ? ttl : DEFAULT_TTL;
    }
    let runtimeArgs = {};
    runtimeArgs = RuntimeArgs.fromMap({
      pool_id: CLValueBuilder.u64(poolId),
      amount : CLValueBuilder.u256(amount),
    })
    console.log("sending");
    console.log(paymentAmount)
    console.log(ttl)
    let trial = 5;
    while (true) {
      try {
        let hash = await this.contractClient.contractCall({
          entryPoint: "un_stake",
          keys: keys,
          paymentAmount,
          runtimeArgs,
          cb: (deployHash) => {
            console.log("deployHash", deployHash);
          },
          ttl,
        });

        return hash;
      } catch (e) {
        trial--
        if (trial == 0) {
          throw e;
        }
        console.log('waiting 3 seconds')
        await sleep(3000)
      }
    }
  }

  async getPendingReward({
    keys,
    poolId, // contract LP
    user, // publickey
    paymentAmount,
    ttl,
  }) {

    if (!paymentAmount) {
      paymentAmount = paymentAmount ? paymentAmount : "50000000000";
      ttl = ttl ? ttl : DEFAULT_TTL;
    }
    let runtimeArgs = {};
    runtimeArgs = RuntimeArgs.fromMap({
      pool_id: CLValueBuilder.u64(poolId),
      user : createRecipientAddress(CLPublicKey.fromHex(user)),
    })
    console.log("sending");
    console.log(paymentAmount)
    console.log(ttl)
    let trial = 5;
    while (true) {
      try {
        let hash = await this.contractClient.contractCall({
          entryPoint: "get_pending_rewards",
          keys: keys,
          paymentAmount,
          runtimeArgs,
          cb: (deployHash) => {
            console.log("deployHash", deployHash);
          },
          ttl,
        });

        return hash;
      } catch (e) {
        trial--
        if (trial == 0) {
          throw e;
        }
        console.log('waiting 3 seconds')
        await sleep(3000)
      }
    }
  }


  async setSupportToken({
    keys,
    nftContractHash, // contract CSP
    nftEnabled,
    paymentAmount,
    ttl,
  }) {

    if (!paymentAmount) {
      paymentAmount = paymentAmount ? paymentAmount : "1000000000";
      ttl = ttl ? ttl : DEFAULT_TTL;
    }

    nftContractHash = new CLByteArray(
      Uint8Array.from(Buffer.from(nftContractHash, "hex"))
    );
    let runtimeArgs = {};
    runtimeArgs = RuntimeArgs.fromMap({
      nft_enabled: CLValueBuilder.bool(nftEnabled),
      nft_contract_hash: createRecipientAddress(nftContractHash),
    })
    console.log("sending");
    console.log(paymentAmount)
    console.log(ttl)
    let trial = 5;
    while (true) {
      try {
        let hash = await this.contractClient.contractCall({
          entryPoint: "set_support_token",
          keys: keys,
          paymentAmount,
          runtimeArgs,
          cb: (deployHash) => {
            console.log("deployHash", deployHash);
          },
          ttl,
        });

        return hash;
      } catch (e) {
        trial--
        if (trial == 0) {
          throw e;
        }
        console.log('waiting 3 seconds')
        await sleep(3000)
      }
    }
  }
  async transferOwner({
    keys,
    newOwner, // contract CSP
    paymentAmount,
    ttl,
  }) {

    if (!paymentAmount) {
      paymentAmount = paymentAmount ? paymentAmount : "1000000000";
      ttl = ttl ? ttl : DEFAULT_TTL;
    }
    let runtimeArgs = {};
    runtimeArgs = RuntimeArgs.fromMap({
      contract_owner: createRecipientAddress(CLPublicKey.fromHex(newOwner)),
    })
    console.log("sending");
    console.log(paymentAmount)
    console.log(ttl)
    let trial = 5;
    while (true) {
      try {
        let hash = await this.contractClient.contractCall({
          entryPoint: "transfer_owner",
          keys: keys,
          paymentAmount,
          runtimeArgs,
          cb: (deployHash) => {
            console.log("deployHash", deployHash);
          },
          ttl,
        });

        return hash;
      } catch (e) {
        trial--
        if (trial == 0) {
          throw e;
        }
        console.log('waiting 3 seconds')
        await sleep(3000)
      }
    }
  }

  async changeMaketFee({
    keys,
    marketFee, // contract CSP
    paymentAmount,
    ttl,
  }) {

    if (!paymentAmount) {
      paymentAmount = paymentAmount ? paymentAmount : "1000000000";
      ttl = ttl ? ttl : DEFAULT_TTL;
    }
    let runtimeArgs = {};
    runtimeArgs = RuntimeArgs.fromMap({
      market_fee: CLValueBuilder.u256(marketFee),
    })
    console.log("sending");
    console.log(paymentAmount)
    console.log(ttl)
    let trial = 5;
    while (true) {
      try {
        let hash = await this.contractClient.contractCall({
          entryPoint: "change_fee",
          keys: keys,
          paymentAmount,
          runtimeArgs,
          cb: (deployHash) => {
            console.log("deployHash", deployHash);
          },
          ttl,
        });

        return hash;
      } catch (e) {
        trial--
        if (trial == 0) {
          throw e;
        }
        console.log('waiting 3 seconds')
        await sleep(3000)
      }
    }
  }

};

module.exports = { genRanHex, Staking };
