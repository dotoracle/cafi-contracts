const {
    utils,
    helpers,
    CasperContractClient,
} = require("casper-js-client-helper");
const { DEFAULT_TTL } = require("casper-js-client-helper/dist/constants");

const { CLValueBuilder, CLByteArray, CLKey, CLPublicKey, CLAccountHash, RuntimeArgs, CLValueParsers } = require("casper-js-sdk");

const { setClient, contractSimpleGetter, createRecipientAddress } = helpers;
const sleep = (ms) => {
    return new Promise((resolve) => setTimeout(resolve, ms));
};
const ERC20 = class {
    constructor(contractHash, nodeAddress, chainName, namedKeysList = []) {
        this.contractHash = contractHash.startsWith("hash-")
            ? contractHash.slice(5)
            : contractHash;
        this.nodeAddress = nodeAddress;
        this.chainName = chainName;
        this.contractClient = new CasperContractClient(nodeAddress, chainName);
        this.namedKeysList = [
            "allowances",
            "balances",
            "decimals",
            "name",
            "symbol",
            "total_supply",
            "supported_token",
            "supported_token_decimasl",
        ];
        this.namedKeysList.push(...namedKeysList)
    }

    static async createInstance(contractHash, nodeAddress, chainName, namedKeysList = []) {
        let wNFT = new ERC20(contractHash, nodeAddress, chainName, namedKeysList);
        await wNFT.init();
        return wNFT;
    }

    async init() {
        const { contractPackageHash, namedKeys } = await setClient(
            this.nodeAddress,
            this.contractHash,
            this.namedKeysList
        );
        this.contractPackageHash = contractPackageHash;
        this.contractClient.chainName = this.chainName
        this.contractClient.contractHash = this.contractHash
        this.contractClient.contractPackageHash = this.contractPackageHash
        this.contractClient.nodeAddress = this.nodeAddress
        /* @ts-ignore */
        this.namedKeys = namedKeys;
    }




    async totalTokenSupply() {
        return await this.readContractField("total_supply");
    }

    async readContractField(field) {
        return await contractSimpleGetter(this.nodeAddress, this.contractHash, [
            field,
        ]);
    }

    async getOperator(tokenId) {
        try {
            const itemKey = tokenId.toString();
            const result = await utils.contractDictionaryGetter(
                this.nodeAddress,
                itemKey,
                this.namedKeys.operator
            );
            return Buffer.from(result.val.data.data).toString("hex");
        } catch (e) {
            throw e;
        }
    }

    static getAccountItemKey(account) {
        let itemKey = "";
        if (typeof account == String) {
            console.log("1 : ")
            itemKey = account.toString();
            console.log(itemKey)
        } else {
            console.log("2 : ")
            let key = createRecipientAddress(account);
            itemKey = Buffer.from(key.data.data).toString("hex");
            console.log(itemKey)
        }
        return itemKey;
    }

    async balanceOf(account) {
        try {
            const key = createRecipientAddress(account);
            const keyBytes = CLValueParsers.toBytes(key).unwrap();
            const itemKey = Buffer.from(keyBytes).toString("base64");
            const result = await utils.contractDictionaryGetter(
              this.nodeAddress,
              itemKey,
              this.namedKeys.balances
            );
            return result.toString();
          } catch (e) {
            if (e.toString().includes("Failed to find base key at path")) {
              return "0";
            }
            throw e;
          }
    }

    async approve({ keys, spencer, amount, paymentAmount, ttl }) {

        // get operator in CLType::Key
        // spencer input should be CONTRACT HASH

        const contracthashbytearray = new CLByteArray(Uint8Array.from(Buffer.from(spencer, 'hex')));
        // console.log("contracthashbytearray", contracthashbytearray)

        const contractHash = new CLKey(contracthashbytearray);
        // console.log("contractHash", contractHash)

        let runtimeArgs = {};
        runtimeArgs = RuntimeArgs.fromMap({

            spender: contractHash,
            amount: CLValueBuilder.u256(amount),
        });

        return await this.contractClient.contractCall({
            entryPoint: "approve",
            keys: keys,
            paymentAmount: paymentAmount ? paymentAmount : "2000000000",
            runtimeArgs,
            cb: (deployHash) => { },
            ttl: ttl ? ttl : DEFAULT_TTL,
        });
    }

    async burn({ keys, owner, amount, paymentAmount, ttl }) {

        let ownerAccountHashByte = Uint8Array.from(
            Buffer.from(owner, 'hex'),
        )
        const ownerKey = createRecipientAddress(new CLAccountHash(ownerAccountHashByte))

        let runtimeArgs = {};
        runtimeArgs = RuntimeArgs.fromMap({
            owner: ownerKey,
            amount: CLValueBuilder.u256(amount),
        });

        return await this.contractClient.contractCall({
            entryPoint: "burn",
            keys: keys,
            paymentAmount: paymentAmount ? paymentAmount : "1000000000",
            runtimeArgs,
            cb: (deployHash) => { },
            ttl: ttl ? ttl : DEFAULT_TTL,
        });
    }

    async transfer(keys, source, recipient, tokenId, paymentAmount, ttl) {
        let identifierMode = await this.identifierMode();
        identifierMode = parseInt(identifierMode.toString());
        let runtimeArgs = {};
        if (identifierMode == 0) {
            runtimeArgs = RuntimeArgs.fromMap({
                token_id: CLValueBuilder.u64(parseInt(tokenId)),
                source_key: createRecipientAddress(source),
                target_key: createRecipientAddress(recipient),
            });
        } else {
            runtimeArgs = RuntimeArgs.fromMap({
                token_hash: CLValueBuilder.string(tokenId),
                source_key: createRecipientAddress(source),
                target_key: createRecipientAddress(recipient),
            });
        }

        return await this.contractClient.contractCall({
            entryPoint: "transfer",
            keys: keys,
            paymentAmount: paymentAmount ? paymentAmount : "1000000000",
            runtimeArgs,
            cb: (deployHash) => { },
            ttl: ttl ? ttl : DEFAULT_TTL,
        });
    }

    async setSupportedToken({ keys, supportedToken, enabled, paymentAmount, ttl }) {

        // get operator in CLType::Key
        // spencer input should be CONTRACT HASH

        const contracthashbytearray = new CLByteArray(Uint8Array.from(Buffer.from(supportedToken, 'hex')));
        console.log("contracthashbytearray", contracthashbytearray)

        const supportedTokenInput = new CLKey(contracthashbytearray);
        console.log("supportedToken", supportedTokenInput)

        let runtimeArgs = {};
        runtimeArgs = RuntimeArgs.fromMap({

            supported_token: supportedTokenInput,
            enabled: CLValueBuilder.bool(enabled),
        });

        return await this.contractClient.contractCall({
            entryPoint: "set_supported_token",
            keys: keys,
            paymentAmount: paymentAmount ? paymentAmount : "3000000000",
            runtimeArgs,
            cb: (deployHash) => { },
            ttl: ttl ? ttl : DEFAULT_TTL,
        });
    }

    async setSupportedTokenDecimals({ keys, supportedToken, decimals, paymentAmount, ttl }) {

        // get operator in CLType::Key
        // spencer input should be CONTRACT HASH

        const contracthashbytearray = new CLByteArray(Uint8Array.from(Buffer.from(supportedToken, 'hex')));
        console.log("contracthashbytearray", contracthashbytearray)

        const supportedTokenInput = new CLKey(contracthashbytearray);
        console.log("supportedToken", supportedTokenInput)

        let runtimeArgs = {};
        runtimeArgs = RuntimeArgs.fromMap({

            supported_token: supportedTokenInput,
            decimals: CLValueBuilder.u8(decimals),
        });

        return await this.contractClient.contractCall({
            entryPoint: "set_supported_token_decimals",
            keys: keys,
            paymentAmount: paymentAmount ? paymentAmount : "3000000000",
            runtimeArgs,
            cb: (deployHash) => { },
            ttl: ttl ? ttl : DEFAULT_TTL,
        });
    }
    async deposit({ keys, owner, depositToken, amount, paymentAmount, ttl }) {

        let ownerAccountHashByte = Uint8Array.from(
            Buffer.from(owner, 'hex'),
        )
        //console.log("ownerAccountHashByte: ", ownerAccountHashByte)

        const ownerKey = createRecipientAddress(new CLAccountHash(ownerAccountHashByte))

        //console.log("ownerKey: ", ownerKey)

        const contracthashbytearray = new CLByteArray(Uint8Array.from(Buffer.from(depositToken, 'hex')));
        // console.log("contracthashbytearray", contracthashbytearray)

        const depositTokenInput = new CLKey(contracthashbytearray);
        // console.log("depositTokenInput", depositTokenInput)



        let runtimeArgs = {};
        runtimeArgs = RuntimeArgs.fromMap({
            owner: ownerKey,
            deposit_token: depositTokenInput,
            amount: CLValueBuilder.u256(amount),
        });

        let trial = 5;
        while (true) {
            try {
                let hash = await this.contractClient.contractCall({
                    entryPoint: "deposit",
                    keys: keys,
                    paymentAmount: paymentAmount ? paymentAmount : "10000000000",
                    runtimeArgs,
                    cb: (deployHash) => { },
                    ttl: ttl ? ttl : DEFAULT_TTL,
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

    async redeem({ keys, owner, redeemToken, amount, paymentAmount, ttl }) {

        let ownerAccountHashByte = Uint8Array.from(
            Buffer.from(owner, 'hex'),
        )
        //console.log("ownerAccountHashByte: ", ownerAccountHashByte)

        const ownerKey = createRecipientAddress(new CLAccountHash(ownerAccountHashByte))

        //console.log("ownerKey: ", ownerKey)

        const contracthashbytearray = new CLByteArray(Uint8Array.from(Buffer.from(redeemToken, 'hex')));
        console.log("contracthashbytearray", contracthashbytearray)

        const redeemTokenInput = new CLKey(contracthashbytearray);
        console.log("redeemTokenInput", redeemTokenInput)



        let runtimeArgs = {};
        runtimeArgs = RuntimeArgs.fromMap({
            owner: ownerKey,
            redeem_token: redeemTokenInput,
            amount: CLValueBuilder.u256(amount),
        });

        let trial = 5;
        while (true) {
            try {
                let hash = await this.contractClient.contractCall({
                    entryPoint: "redeem",
                    keys: keys,
                    paymentAmount: paymentAmount ? paymentAmount : "10000000000",
                    runtimeArgs,
                    cb: (deployHash) => { },
                    ttl: ttl ? ttl : DEFAULT_TTL,
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

module.exports = ERC20;
