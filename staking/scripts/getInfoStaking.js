require("dotenv").config();
let blake = require("blakejs")

const { CLAccountHash, CLPublicKey, U64_ID, U256_ID } = require("casper-js-sdk");
let Staking = require("./sdkStaking").Staking;
let contractHash =
  "7faeeb6f5facec8d4dc79a7d313215e1cd53ea878ff8d2100176ee703e8a237c"; // staking contract
const { NODE_ADDRESS, EVENT_STREAM_ADDRESS, CHAIN_NAME, WASM_PATH } =
  process.env;
async function main() {
  let contract = new Staking(contractHash, NODE_ADDRESS, CHAIN_NAME);
  await contract.init();

  let a = "account-hash-ac4f741d11c268b21a188784dbed2b06db0d37683d9b401d7e9871a8b36a42ee";
  let b = "1";
  function strToBytes(str) {
    const bytes = [];
    for (ii = 0; ii < str.length; ii++) {
      const code = str.charCodeAt(ii); // x00-xFFFF
      bytes.push(code & 255); // low, high
    }
    return bytes;
  }

  let aBytes = strToBytes(a)
  let bBytes = strToBytes(b)
  let finalS1 = aBytes.concat(bBytes)
  let finalS2 = bBytes.concat(aBytes)
  console.log('finalS1', Buffer.from(finalS1).toString('hex'), finalS1.length)
  console.log('finalS1', Buffer.from(finalS2).toString('hex'))
  console.log(a.length)



  let h1 = blake.blake2b(Buffer.from(finalS1), null, 32)
  let h2 = blake.blake2b(Buffer.from(finalS2), null, 32)



  console.log('h1', Buffer.from(h1).toString('hex'))
  console.log('h2', Buffer.from(h2).toString('hex'))

  try {
    let userInfo = await contract.userInfo(Buffer.from(h1).toString('hex')) 
    console.log("userInfo: ", userInfo)
  } catch (e) {
    console.error(e)
  }
}

main();
