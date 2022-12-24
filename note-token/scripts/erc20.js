const {
    utils,
    helpers,
    CasperContractClient,
} = require("casper-js-client-helper");
const { DEFAULT_TTL } = require("casper-js-client-helper/dist/constants");

const { CLValueBuilder, CLByteArray, CLKey, CLPublicKey, CLAccountHash, RuntimeArgs } = require("casper-js-sdk");

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

    NFTMetadataKind = {
        CEP78: 0,
        NFT721: 1,
        Raw: 2,
        CustomValidated: 3,
    };

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



    async identifierMode() {
        let mode = await contractSimpleGetter(this.nodeAddress, this.contractHash, [
            "identifier_mode",
        ]);
        return mode.toNumber()
    }

    async collectionName() {
        return await this.readContractField("collection_name");
    }

    async allowMinting() {
        return await this.readContractField("allow_minting");
    }

    async collectionSymbol() {
        return await this.readContractField("collection_symbol");
    }

    async contractWhitelist() {
        return await this.readContractField("contract_whitelist");
    }

    async holderMode() {
        return await this.readContractField("holder_mode");
    }

    async installer() {
        return await this.readContractField("installer");
    }

    async jsonSchema() {
        return await this.readContractField("json_schema");
    }

    async metadataMutability() {
        return await this.readContractField("metadata_mutability");
    }

    async mintingMode() {
        return await this.readContractField("minting_mode");
    }

    async nftKind() {
        return await this.readContractField("nft_kind");
    }

    async nftMetadataKind() {
        return await this.readContractField("nft_metadata_kind");
    }

    async numberOfMintedTokens() {
        return await this.readContractField("number_of_minted_tokens");
    }

    async ownershipMode() {
        return await this.readContractField("ownership_mode");
    }

    async receiptName() {
        return await this.readContractField("receipt_name");
    }

    async totalTokenSupply() {
        return await this.readContractField("total_supply");
    }

    async whitelistMode() {
        return await this.readContractField("whitelist_mode");
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

    async getOwnerOf(tokenId) {
        try {
            const itemKey = tokenId.toString();
            const result = await utils.contractDictionaryGetter(
                this.nodeAddress,
                itemKey,
                this.namedKeys.tokenOwners
            );
            return Buffer.from(result.data).toString("hex");
        } catch (e) {
            throw e;
        }
    }

    async burntTokens(tokenId) {
        try {
            const itemKey = tokenId.toString();
            const result = await utils.contractDictionaryGetter(
                this.nodeAddress,
                itemKey,
                this.namedKeys.burntTokens
            );
            return result ? true : false;
        } catch (e) { }
        return false;
    }

    async getTokenMetadata(tokenId) {
        try {
            const itemKey = tokenId.toString();
            let nftMetadataKind = await this.nftMetadataKind();
            nftMetadataKind = parseInt(nftMetadataKind.toString());
            let result = null;
            if (nftMetadataKind == this.NFTMetadataKind.CEP78) {
                result = await utils.contractDictionaryGetter(
                    this.nodeAddress,
                    itemKey,
                    this.namedKeys.metadataCep78
                );
            } else if (nftMetadataKind == this.NFTMetadataKind.CustomValidated) {
                result = await utils.contractDictionaryGetter(
                    this.nodeAddress,
                    itemKey,
                    this.namedKeys.metadataCustomValidated
                );
            } else if (nftMetadataKind == this.NFTMetadataKind.NFT721) {
                result = await utils.contractDictionaryGetter(
                    this.nodeAddress,
                    itemKey,
                    this.namedKeys.metadataNft721
                );
            } else if (nftMetadataKind == this.NFTMetadataKind.Raw) {
                result = await utils.contractDictionaryGetter(
                    this.nodeAddress,
                    itemKey,
                    this.namedKeys.metadataRaw
                );
            }

            return result;
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

    async getOwnedTokens(account) {
        try {
            let itemKey = CEP78.getAccountItemKey(account);
            const result = await utils.contractDictionaryGetter(
                this.nodeAddress,
                itemKey,
                this.namedKeys.ownedTokens
            );
            return result.map((e) => e.data);
        } catch (e) {
            throw e;
        }
    }

    async balanceOf(account) {
        try {
            let itemKey = ERC20.getAccountItemKey(account);
            console.log("itemKey: ", itemKey)
            console.log("this.namedKeys: ", this.namedKeys.balances)
            const result = await utils.contractDictionaryGetter(
                this.nodeAddress,
                itemKey,
                this.namedKeys.balances
            );
            return result;
        } catch (e) {
            throw e;
        }
    }

    async approve({ keys, spencer, amount, paymentAmount, ttl }) {

        // get operator in CLType::Key
        // spencer input should be CONTRACT HASH

        const contracthashbytearray = new CLByteArray(Uint8Array.from(Buffer.from(spencer, 'hex')));
        console.log("contracthashbytearray", contracthashbytearray)

        const contractHash = new CLKey(contracthashbytearray);
        console.log("contractHash", contractHash)

        let runtimeArgs = {};
        runtimeArgs = RuntimeArgs.fromMap({

            spender: contractHash,
            amount: CLValueBuilder.u256(amount),
        });

        return await this.contractClient.contractCall({
            entryPoint: "approve",
            keys: keys,
            paymentAmount: paymentAmount ? paymentAmount : "1000000000",
            runtimeArgs,
            cb: (deployHash) => { },
            ttl: ttl ? ttl : DEFAULT_TTL,
        });
    }

    async approveForAll(keys, operator, paymentAmount, ttl) {
        let key = createRecipientAddress(operator);
        let runtimeArgs = RuntimeArgs.fromMap({
            operator: key,
        });

        return await this.contractClient.contractCall({
            entryPoint: "set_approval_for_all",
            keys: keys,
            paymentAmount: paymentAmount ? paymentAmount : "1000000000",
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
        console.log("contracthashbytearray", contracthashbytearray)

        const depositTokenInput = new CLKey(contracthashbytearray);
        console.log("depositTokenInput", depositTokenInput)



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
                    paymentAmount: paymentAmount ? paymentAmount : "5000000000",
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

        const contracthashbytearray = new CLByteArray(Uint8Array.from(Buffer.from(depositToken, 'hex')));
        console.log("contracthashbytearray", contracthashbytearray)

        const depositTokenInput = new CLKey(contracthashbytearray);
        console.log("depositTokenInput", depositTokenInput)



        let runtimeArgs = {};
        runtimeArgs = RuntimeArgs.fromMap({
            owner: ownerKey,
            redeem_token: depositTokenInput,
            amount: CLValueBuilder.u256(amount),
        });

        let trial = 5;
        while (true) {
            try {
                let hash = await this.contractClient.contractCall({
                    entryPoint: "redeem",
                    keys: keys,
                    paymentAmount: paymentAmount ? paymentAmount : "5000000000",
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
