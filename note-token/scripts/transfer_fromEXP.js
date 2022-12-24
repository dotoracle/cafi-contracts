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

console.log("A")
let privateKeyPemTony = `
-----BEGIN PRIVATE KEY-----
${keytonya}
${keytonyb}
${keytonyc}
-----END PRIVATE KEY-----
`; // tony key



let expContract = contractInfo.namedKeys
  .filter((e) => e.name == "erc20_token_contract")[0]
  .key.slice(5);
console.log("expContract: ", expContract)
let nft_contract =
"1a22cab6274df9a09fc721b04f83f04895538339ddf37c17883c12a4bb4a55cd"
//"a7643ef321cce2cd1401a338be87c1a6cffffe4f482b5364f35ccc1f085e9c22" // CSP contract
//  "6fcf59753e5ab985122a88470101acb338594614266a506a2e3cf57025bc4ddc"
// "68d05b72593981f73f5ce7ce5dcac9033aa0ad4e8c93b773f8b939a18c0bbc3b";
//"805347b595cc24814f0d50482069a1dba24f9bfb2823c6e900386f147f25754b";
//"52f370db3aeaa8c094e73a3aa581c85abc775cc52605e9cd9364cae0501ce645";
//"44f244fb474431a20c4968d60550f790000d21785650c963f9ac5e02c126e1fb";

let toAddress = "020261207299a7d59261d28a0780b92f76b5caff3ee2e3f767d7cd832e269c181767" // publicKey


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
  let exp = await ERC20.createInstance(expContract, NODE_ADDRESS, CHAIN_NAME)
  // let cep78 = await sdk.CEP78.createInstance(nft_contract, NODE_ADDRESS, CHAIN_NAME)
  // const contracthashbytearray = new CLByteArray(Uint8Array.from(Buffer.from("5db43d7bda61a954f4a73d51de9ee3a1c1a58d2b9cf895e1b98c6d3f73ee38e9", 'hex')));
  // const nftContractHash = new CLKey(contracthashbytearray);

  // let hashApprove = await cep78.approveForAll({
  //   keys: KEYS,
  //   operator: nftContractHash
  // })
  // console.log(`... Contract installation deployHash: ${hashApprove}`);

  // await getDeploy(NODE_ADDRESS, hashApprove);

  //let accounthash = "a769093d50eebe829668ce0116cf24da9f17dcfe223bac30e1c33967d5888c71" // account hash

  let contractHash = "941280b71b1b66d3cd7808fa74f073617385f518d1b6c84c4b43e7b5b1eed653" // CSP CONTRACT
  
  let hash = await exp.approve({
    keys: KEYSTony,
    spencer: contractHash,
    amount: '3000000000',
  })

  console.log(`... Contract installation deployHash: ${hash}`);

  await getDeploy(NODE_ADDRESS, hash);

  console.log(`... Contract installed successfully.`);
};

test();
