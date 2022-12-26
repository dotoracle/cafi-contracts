require("dotenv").config();
let contractInfo = require("./contractinfo.json");
const { CasperContractClient, helpers } = require("casper-js-client-helper");
const { getDeploy } = require("./utils");
const { createRecipientAddress } = helpers;
const ERC20 = require('./erc20')
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

let noteContract = "a1ad933de2a21ee72360653340f2868a71f11bc4173aa865d5cc8bbc2ade34d6"
console.log("noteContract: ", noteContract)

let tony = "020261207299a7d59261d28a0780b92f76b5caff3ee2e3f767d7cd832e269c181767" // publicKey


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


let privateKeyPemTony = `
-----BEGIN PRIVATE KEY-----
${keytonya}
${keytonyb}
${keytonyc}
-----END PRIVATE KEY-----
`; // tony key

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
  let note = await ERC20.createInstance(noteContract, NODE_ADDRESS, CHAIN_NAME)

  let accounthash = "a769093d50eebe829668ce0116cf24da9f17dcfe223bac30e1c33967d5888c71" // account hash Tony
  // let USDT = "22a34d9a6b1acbf38f6fd9bdfe086f13a96ca341f8cdf3e3cd39ee0b67f56d85"
  let USDC = "95896c8167b3343095a98829a9ec58198956d5a832f0571193d935b1bb0e3065"
  // let BUSD = "6a17bac467b1bebe40267ea3f0b30407c21d9998dedf9d758cac5761408f5366"


  // let accounthash = "55884917f4107a59e8c06557baee7fdada631af6d1c105984d196a84562854eb" // ABB

  let hash = await note.deposit({
    keys: KEYSTony,
    owner: accounthash,
    depositToken: USDC,
    amount: '2000000',
  })

  console.log(`... Contract installation deployHash: ${hash}`);

  await getDeploy(NODE_ADDRESS, hash);

  console.log(`... Contract installed successfully.`);
};

test();
