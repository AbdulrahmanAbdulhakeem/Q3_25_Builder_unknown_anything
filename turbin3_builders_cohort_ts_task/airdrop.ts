import { Keypair,Connection,LAMPORTS_PER_SOL } from "@solana/web3.js";
import wallet from "./dev-wallet.json";

const keypair = Keypair.fromSecretKey(new Uint8Array(wallet));

const connection = new Connection("https://api.devnet.solana.com","confirmed");

(async () => {
    try{
        const txhash = await connection.requestAirdrop(keypair.publicKey, 2 * LAMPORTS_PER_SOL);
        console.log(`Success! Check out your TX here:https://explorer.solana.com/tx/${txhash}?cluster=devnet`);
    }catch(e){
        console.error(`OOPS Something went wrong${e}`)
    }
})();