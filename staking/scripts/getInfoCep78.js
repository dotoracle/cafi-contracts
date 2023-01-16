require("dotenv").config();
const { CLAccountHash, CLPublicKey, U64_ID, U256_ID } = require("casper-js-sdk");
let CEP78 = require("../../indexCasperPunk").CEP78;
let contractHash =
  "be170557d9100704b63dc5de6039373dfdc8649cc467bf2f74ea14812d233def";
let contractInfo = require("./contractinfo.json");
let nft_bridge_contract = contractInfo.namedKeys
  .filter((e) => e.name == "dotoracle_nft_bridge_contract")[0]
  .key.slice(5);
const { NODE_ADDRESS, EVENT_STREAM_ADDRESS, CHAIN_NAME, WASM_PATH } =
  process.env;
async function main() {
  let contract = new CEP78(contractHash, NODE_ADDRESS, CHAIN_NAME);
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

  //0131e805fde6a85b63aa366990136b4759a596d9a988bde62b84131bc86a910e6b // PHUONG
  try {
    let metadata = await contract.getOwnedTokens(CLPublicKey.fromHex("0131e805fde6a85b63aa366990136b4759a596d9a988bde62b84131bc86a910e6b")) // abb account
    let meta2 = await contract.getTokenMetadata(1)
    //let metadata = await contract.getOwnedTokens(CLPublicKey.fromHex("0121eb7d280926cd62ae0b44ee628ba057e9b2696021ab0e20e40e528ae243bde1"))// Vi hka
    let bal = await contract.balanceOf(CLPublicKey.fromHex("0131e805fde6a85b63aa366990136b4759a596d9a988bde62b84131bc86a910e6b"))
    console.log("metadata", metadata.map((e)=> parseInt(e)));
    console.log("bal: ", parseInt(bal))
    console.log("meta2: ", meta2)
  } catch (e) {
    console.error(e)
  }
}

main();
