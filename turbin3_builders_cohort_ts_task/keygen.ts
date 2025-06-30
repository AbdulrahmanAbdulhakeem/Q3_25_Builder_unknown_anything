import { Keypair } from "@solana/web3.js";

//Create Keypair
const kp = Keypair.generate()
console.log(`You have generated a public key ${kp.publicKey.toBase58()}`)
// RcfAdNQXpvJofTY8wbuHKPVqEo8JEJUVdPzycXAHSEW

console.log(`[${kp.secretKey}]`)