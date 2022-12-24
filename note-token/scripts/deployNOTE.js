require('dotenv').config() 
const fs = require('fs');

const { utils, helpers } = require('casper-js-client-helper')
const { sleep, getDeploy } = require('./utils')

const {
  CLValueBuilder,
  Keys,
  CLPublicKey,
  CLPublicKeyType,
  RuntimeArgs,
  CLString,
  CLAccountHash
} = require('casper-js-sdk')
let key = require('./keys.json').key
let apoc = require('./keys.json').apoc

const {
  fromCLMap,
  toCLMap,
  installContract,
  setClient,
  contractSimpleGetter,
  contractCallFn,
  createRecipientAddress
} = helpers;

const {
  NODE_ADDRESS,
  EVENT_STREAM_ADDRESS,
  CHAIN_NAME,
  WASM_PATH
} = process.env
let paymentAmount = '120000000000' //30

let privateKeyPem = `
-----BEGIN PRIVATE KEY-----
${apoc}
-----END PRIVATE KEY-----
`

let privateKeyBuffer = Keys.Ed25519.parsePrivateKey(Keys.Ed25519.readBase64WithPEM(privateKeyPem))
let publicKey = Keys.Ed25519.privateToPublicKey(Uint8Array.from(privateKeyBuffer))
let KEYS = new Keys.Ed25519.parseKeyPair(publicKey, Uint8Array.from(privateKeyBuffer))
console.log('pubkey', KEYS.accountHex())
let contract_key_name = "NOTE-TOKEN"
let contract_exp_symbol = "NOTE"
let contract_owner = "80883b3d876c33d77ef2826eb559d445a53fc7c6db2ac008fb0e321c1380d291" // APOC
let dev = "80883b3d876c33d77ef2826eb559d445a53fc7c6db2ac008fb0e321c1380d291" // APOC

let EXP_WASM_PATH = "target/wasm32-unknown-unknown/release/erc20_token.wasm"
const test = async () => {

  let ownerAccountHashByte = Uint8Array.from(
    Buffer.from(contract_owner, 'hex'),
)
const ownerKey = createRecipientAddress(new CLAccountHash(ownerAccountHashByte))


  const runtimeArgs = RuntimeArgs.fromMap({
    "name": CLValueBuilder.string(contract_key_name),
     "symbol": CLValueBuilder.string(contract_exp_symbol), 
     "decimals": CLValueBuilder.u8(9), 
     "total_supply": CLValueBuilder.u256(1), // inital supply
     "fee" : CLValueBuilder.u256(10), // per 1000
     "fee_receiver": ownerKey,
     "contract_owner": ownerKey,
  });

  console.log("A")
  console.log(CHAIN_NAME)
  console.log(NODE_ADDRESS)
  console.log(KEYS)
  console.log(runtimeArgs)
  console.log(paymentAmount)
  console.log(EXP_WASM_PATH)

  let hash = await installContract(
    CHAIN_NAME,
    NODE_ADDRESS,
    KEYS,
    runtimeArgs,
    paymentAmount,
    EXP_WASM_PATH
  );
  console.log("B")

  console.log(`... Contract installation deployHash: ${hash}`)

  await getDeploy(NODE_ADDRESS, hash)

  let accountInfo = await utils.getAccountInfo(NODE_ADDRESS, KEYS.publicKey)

  console.log(`... Contract installed successfully.`)

  console.log(`... Account Info: `)
  console.log(JSON.stringify(accountInfo, null, 2))
  fs.writeFileSync('scripts/contractinfo.json', JSON.stringify(accountInfo, null, 2));

  // const contractHash = await utils.getAccountNamedKeyValue(
  //   accountInfo,
  //   `erc20_token_contract`,
  // )

  // await getDeploy(NODE_ADDRESS!, installDeployHash)

  // console.log(`... Contract installed successfully.`)

  // let accountInfo = await utils.getAccountInfo(NODE_ADDRESS!, KEYS.publicKey)

  // console.log(`... Account Info: `)
  // console.log(JSON.stringify(accountInfo, null, 2))

  // const contractHash = await utils.getAccountNamedKeyValue(
  //   accountInfo,
  //   `erc20_token_contract`,
  // )

  // await erc20.setContractHash(
  //   contractHash.slice(
  //     5
  //   )
  // );

  // console.log(`... Contract Hash: ${contractHash}`)

  // let deployed_minter = await erc20.minter()
  // console.log(`... deployed_minter: ${deployed_minter}`)
  // console.log(`... fee: ${await erc20.swapFee()}`)
  // console.log(`... dev: ${await erc20.dev()}`)
}

test()
