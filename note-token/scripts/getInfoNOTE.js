require("dotenv").config();
const { CLAccountHash, CLPublicKey, U64_ID, U256_ID } = require("casper-js-sdk");
let ERC20 = require("./erc20");
let contractHashNote =
  "a1ad933de2a21ee72360653340f2868a71f11bc4173aa865d5cc8bbc2ade34d6"; // Note contract

let contractHashUSDC =
  "95896c8167b3343095a98829a9ec58198956d5a832f0571193d935b1bb0e3065"; // USDC contract

let contractInfo = require("./contractinfo.json");
const { NODE_ADDRESS, EVENT_STREAM_ADDRESS, CHAIN_NAME, WASM_PATH } =
  process.env;


async function main() {
  let contractNote = new ERC20(contractHashNote, NODE_ADDRESS, CHAIN_NAME);
  await contractNote.init();

  try {
    let totalSupplyNote = await contractNote.totalTokenSupply()
    let balNote = await contractNote.balanceOf(CLPublicKey.fromHex("020261207299a7d59261d28a0780b92f76b5caff3ee2e3f767d7cd832e269c181767")) // Tony
    console.log("totalSupplyNote: ", parseInt(totalSupplyNote))
    console.log("balNote: ", balNote)
  } catch (e) {
    console.error(e)
  }


  let contractUSDC = new ERC20(contractHashUSDC, NODE_ADDRESS, CHAIN_NAME);
  await contractUSDC.init();

  try {
    let totalSupplyUSDC = await contractUSDC.totalTokenSupply()
    let balUSDC = await contractUSDC.balanceOf(CLPublicKey.fromHex("020261207299a7d59261d28a0780b92f76b5caff3ee2e3f767d7cd832e269c181767")) // Tony
    console.log("totalSupplyUSDC: ", parseInt(totalSupplyUSDC))
    console.log("balUSDC: ", balUSDC)
  } catch (e) {
    console.error(e)
  }

}

main();
