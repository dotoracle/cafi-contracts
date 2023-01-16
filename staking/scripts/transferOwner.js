require("dotenv").config();
let contractInfo = require("./contractinfo.json");
const { CasperContractClient, helpers } = require("casper-js-client-helper");
const { getDeploy } = require("./utils");
const { createRecipientAddress } = helpers;
const sdk = require('../../indexCasperPunk')
let key = require('./keys.json').key
let keytonya = require('./keys.json').keyTonya
let keytonyb = require('./keys.json').keyTonyb
let keytonyc = require('./keys.json').keyTonyc


const { CLValueBuilder, Keys, RuntimeArgs, CLByteArrayBytesParser, CLByteArray, CLKey, CLPublicKey, CLAccountHash } = require("casper-js-sdk");

const { NODE_ADDRESS, EVENT_STREAM_ADDRESS, CHAIN_NAME, WASM_PATH } =
  process.env;

let privateKeyPem = `
-----BEGIN PRIVATE KEY-----
${key}
-----END PRIVATE KEY-----
`; // abb key
let privateKeyPemTony = `
-----BEGIN PRIVATE KEY-----
${keytonya}
${keytonyb}
${keytonyc}
-----END PRIVATE KEY-----
`; // tony key

let marketPlaceHash = "hash-2ce10419d31b6705144d11d4c0a56ddc9271b0a5fe5d7c8fbcc1180735b212d5"
console.log("market-place: ", marketPlaceHash)

let newOwnerPub =
  "020261207299a7d59261d28a0780b92f76b5caff3ee2e3f767d7cd832e269c181767"
// "805347b595cc24814f0d50482069a1dba24f9bfb2823c6e900386f147f25754b"
let oldOwnerPub = "017e80955a6d493a4a4b9f1b5dd23d2edcdc2c8b00fcd9689f2f735f501bd088c5"
let privateKeyBuffer = Keys.Ed25519.parsePrivateKey(
  Keys.Ed25519.readBase64WithPEM(privateKeyPem)
);
let publicKey = Keys.Ed25519.privateToPublicKey(
  Uint8Array.from(privateKeyBuffer)
);
let KEYS = new Keys.Ed25519.parseKeyPair(
  publicKey,
  Uint8Array.from(privateKeyBuffer)
);
let privateKeyBufferTony = Keys.Secp256K1.parsePrivateKey(
  Keys.Secp256K1.readBase64WithPEM(privateKeyPemTony)
);


let publicKeyTony = Keys.Secp256K1.privateToPublicKey(
  Uint8Array.from(privateKeyBufferTony)
);

let KEYSTony = new Keys.Secp256K1.parseKeyPair(
  publicKeyTony,
  Uint8Array.from(privateKeyBufferTony),
  "raw"
);

const test = async () => {


  let market = await sdk.CSPMarketPlace.createInstance(marketPlaceHash, NODE_ADDRESS, CHAIN_NAME, [])


  let hashTransferOwner = await market.transferOwner({
    keys: KEYS,
    newOwner: newOwnerPub,
  })

  console.log(`... Transfer owner installation deployHash: ${hashTransferOwner}`);

  await getDeploy(NODE_ADDRESS, hashTransferOwner);

  console.log(`... Transfer owner installed successfully.`);

  let hashTransferOwner1 = await market.transferOwner({
    keys: KEYSTony,
    newOwner: oldOwnerPub,
  })

  console.log(`... Transfer owner back installation deployHash: ${hashTransferOwner1}`);

  await getDeploy(NODE_ADDRESS, hashTransferOwner1);

  console.log(`... Transfer owner back installed successfully.`);

  // change is_royalty

  let hashChangeIsRoyalty = await market.changeIsRoyalty({
    keys: KEYS,
    isRoyalty: false,
  })

  console.log(`... changeIsRoyalty installation deployHash: ${hashChangeIsRoyalty}`);

  await getDeploy(NODE_ADDRESS, hashChangeIsRoyalty);

  console.log(`... changeIsRoyalty installed successfully.`);


  // change is_royalty

  let hashChangeIsRoyaltytrue = await market.changeIsRoyalty({
    keys: KEYS,
    isRoyalty: true,
  })

  console.log(`... hashChangeIsRoyaltytrue installation deployHash: ${hashChangeIsRoyaltytrue}`);

  await getDeploy(NODE_ADDRESS, hashChangeIsRoyaltytrue);

  console.log(`... hashChangeIsRoyaltytrue installed successfully.`);


  // change Royalty Fee
  let hashChangeRoyaltyFee = await market.changeRoyaltyFee({
    keys: KEYS,
    royaltyFee: "10",
  })

  console.log(`... hashChangeRoyaltyFee installation deployHash: ${hashChangeRoyaltyFee}`);

  await getDeploy(NODE_ADDRESS, hashChangeRoyaltyFee);

  console.log(`... hashChangeRoyaltyFee installed successfully.`);

  // change MARKET Fee
  let hashChangeMarketFee = await market.changeMaketFee({
    keys: KEYS,
    marketFee: "50",
  })

  console.log(`... hashChangeMarketFee installation deployHash: ${hashChangeMarketFee}`);

  await getDeploy(NODE_ADDRESS, hashChangeMarketFee);

  console.log(`... hashChangeMarketFee installed successfully.`);

    // change MARKET Fee back
    let hashChangeMarketFeeback = await market.changeMaketFee({
      keys: KEYS,
      marketFee: "80",
    })
  
    console.log(`... hashChangeMarketFeeback installation deployHash: ${hashChangeMarketFeeback}`);
  
    await getDeploy(NODE_ADDRESS, hashChangeMarketFeeback);
  
    console.log(`... hashChangeMarketFeeback installed successfully.`);
  

  //Change wcspr contract
  let new1 = "33f48edd5874bc6095cb0018ff364867578ff44ff18cdb9ecf0c3220d09a8325" // wcspr hash

  let hashChangeWcspr = await market.changeWcsprContract({
    keys: KEYS,
    newWcsprContract: new1,
  })

  console.log(`... hashChangeWcspr installation deployHash: ${hashChangeWcspr}`);

  await getDeploy(NODE_ADDRESS, hashChangeWcspr);

  console.log(`... hashChangeWcspr installed successfully.`);


    //Change wcspr contract back
    let old1 = "30070685c86e7fb410839f1ffc86de2181d4776926248e0946350615929b1ce2" // wcspr hash

    let hashChangeWcsprback = await market.changeWcsprContract({
      keys: KEYS,
      newWcsprContract: old1,
    })
  
    console.log(`... hashChangeWcsprback installation deployHash: ${hashChangeWcsprback}`);
  
    await getDeploy(NODE_ADDRESS, hashChangeWcsprback);
  
    console.log(`... hashChangeWcsprback installed successfully.`);
  


};

test();
