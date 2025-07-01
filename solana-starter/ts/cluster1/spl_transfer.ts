import { Commitment, Connection, Keypair, LAMPORTS_PER_SOL, PublicKey } from "@solana/web3.js"
import wallet from "../turbin3-wallet.json"
import { getOrCreateAssociatedTokenAccount, transfer } from "@solana/spl-token";

// We're going to import our keypair from the wallet file
const keypair = Keypair.fromSecretKey(new Uint8Array(wallet));

//Create a Solana devnet connection
const commitment: Commitment = "confirmed";
const connection = new Connection("https://api.devnet.solana.com", commitment);

// Mint address
const mint = new PublicKey("5csc2ESg5mUP3WFwU1fHyiB91U2iwsxvi8DeTExnvfeu");
const to = new PublicKey("AYnYh4u4tyANs9KJo1xegohEQcA2pWxeqHFMwUhE15eT");
// Recipient address

(async () => {
    try {
        // Get the token account of the fromWallet address, and if it does not exist, create it
        const ata = await getOrCreateAssociatedTokenAccount(
            connection,
            keypair,
            mint,
            keypair.publicKey
        )

        // Get the token account of the toWallet address, and if it does not exist, create it
        const toAta = await getOrCreateAssociatedTokenAccount(
            connection,
            keypair,
            mint,
            to
        )

        // Transfer the new token to the "toTokenAccount" we just created
        const transferId = await transfer(
            connection,
            keypair,
            ata.address,
            toAta.address,
            keypair.publicKey,
            1_000_000n
        )

        console.log(`Transfer ID:${transferId}`)
    } catch(e) {
        console.error(`Oops, something went wrong: ${e}`)
    }
})();