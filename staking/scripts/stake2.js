require("dotenv").config();
let contractInfo = require("./contractinfo.json");
const { CasperContractClient, helpers } = require("casper-js-client-helper");
const { getDeploy } = require("./utils");
const { createRecipientAddress } = helpers;
const sdk = require('./sdkStaking')
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

let stakingHash = "hash-7faeeb6f5facec8d4dc79a7d313215e1cd53ea878ff8d2100176ee703e8a237c"
console.log("market-place: ", stakingHash)

let lpContract =
  "30070685c86e7fb410839f1ffc86de2181d4776926248e0946350615929b1ce2"
// "805347b595cc24814f0d50482069a1dba24f9bfb2823c6e900386f147f25754b"

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


  let staking = await sdk.Staking.createInstance(stakingHash, NODE_ADDRESS, CHAIN_NAME, [])


  let hash = await staking.stake({
    keys: KEYSTony,
    poolId: 0, // 
    amount: "1000000000", // 
    stakeDuration: "1000",
  })

  console.log(`... stake: ${hash}`);

  await getDeploy(NODE_ADDRESS, hash);

  console.log(`... Contract installed successfully.`);
};

test();
