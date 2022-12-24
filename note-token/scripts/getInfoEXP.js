require("dotenv").config();
const { CLAccountHash, CLPublicKey, U64_ID, U256_ID } = require("casper-js-sdk");
let ERC20 = require("./erc20");
let contractHash =
  //"f22f266ea7171e2e3c7c21266c8b7f0da2ddee8b2357ca85339af73a4018d374";
  //"fd7427f382e6129faf66a886e77bfdcc6edfd183813aab1b0b553cbd96a9c500";
  //"d2605b4544842bced2eeed6d3c541e220cd4942ad39cbd5e3ea14d475cdf194b"; // new 43113 wrapped token
  //"805347b595cc24814f0d50482069a1dba24f9bfb2823c6e900386f147f25754b";
  //"52f370db3aeaa8c094e73a3aa581c85abc775cc52605e9cd9364cae0501ce645";
  //"68d05b72593981f73f5ce7ce5dcac9033aa0ad4e8c93b773f8b939a18c0bbc3b";
  "bc851e618e0ed7c607b6c4876b95f67221376affe74feb7ee7c49f6f4e7c7514"; // EXP contract
let contractInfo = require("./contractinfo.json");
let nft_bridge_contract = contractInfo.namedKeys
  .filter((e) => e.name == "dotoracle_nft_bridge_contract")[0]
  .key.slice(5);
const { NODE_ADDRESS, EVENT_STREAM_ADDRESS, CHAIN_NAME, WASM_PATH } =
  process.env;
async function main() {
  let contract = new ERC20(contractHash, NODE_ADDRESS, CHAIN_NAME);
  await contract.init();
  // let identifier_mode = await contract.identifierMode();
  // console.log("identifier_mode", identifier_mode.toString());

  //   let ownerOf = await cep78.getOwnerOf(31);
  //   console.log("ownerOf", ownerOf);

  //   let balanceOf = await cep78.balanceOf("3bdcc50ce1e1e0119d4901b686c65c66b63cc17e5fa5da2299e332c545ec23c6")
  //   console.log('balanceOf', balanceOf.toString())

  //   let burntTokens = await cep78.burntTokens(31);
  //   console.log("burntTokens", burntTokens);

  //   let metadata = await cep78.getTokenMetadata(31);
  //   console.log("metadata", metadata);

  //   let operator = await cep78.getOperator(31);
  //   console.log("operator", operator);

  try {
    let totalSupply = await contract.totalTokenSupply() // abb account
    // let meta2 = await contract.getTokenMetadata(1)
    //let metadata = await contract.getOwnedTokens(CLPublicKey.fromHex("0121eb7d280926cd62ae0b44ee628ba057e9b2696021ab0e20e40e528ae243bde1"))// Vi hka
    let bal = await contract.balanceOf(CLPublicKey.fromHex("017e80955a6d493a4a4b9f1b5dd23d2edcdc2c8b00fcd9689f2f735f501bd088c5"))
    // console.log("metadata", metadata.map((e)=> parseInt(e)));
    console.log("totalSupply: ", parseInt(totalSupply))
    console.log("bal: ", bal)
  } catch (e) {
    console.error(e)
  }
}

main();
