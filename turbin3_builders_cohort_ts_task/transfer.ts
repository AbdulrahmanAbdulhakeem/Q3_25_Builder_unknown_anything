import { Connection,Transaction,SystemProgram,LAMPORTS_PER_SOL,PublicKey,Keypair,sendAndConfirmTransaction } from "@solana/web3.js";
import wallet from "./dev-wallet.json"

const from = Keypair.fromSecretKey(new Uint8Array(wallet))

const to = new PublicKey("Amg1KrXiPcfhf6oqzch59p7JV4sQmiizy17LmGvw2UGw")

const connection = new Connection("https://api.devnet.solana.com","confirmed");

// (async () => {
//     try {
//         const transaction = new Transaction().add(
//             SystemProgram.transfer({
//                 fromPubkey:from.publicKey,
//                 toPubkey:to,
//                 lamports:LAMPORTS_PER_SOL/100
//             })
//         )
//         transaction.recentBlockhash = (
//             await connection.getLatestBlockhash("confirmed")
//         ).blockhash
//         transaction.feePayer = from.publicKey

//         const signature = await sendAndConfirmTransaction(
//             connection,
//             transaction,
//             [from]
//         )

//         console.log(`Success! Check out your TX here:https://explorer.solana.com/tx/${signature}?cluster=devnet`);
//     } catch (error) {
//         console.error(`OOPS Something went wrong ${error}`)
//     }
// })()

(async () => {
    try {
        const balance = await connection.getBalance(from.publicKey)
        const transaction = new Transaction().add(
            SystemProgram.transfer({
                fromPubkey:from.publicKey,
                toPubkey:to,
                lamports:balance
            })
        )
        transaction.recentBlockhash = ((await connection.getLatestBlockhash("confirmed")).blockhash)
        transaction.feePayer = from.publicKey
        const fee = (await connection.getFeeForMessage(transaction.compileMessage(),"confirmed")).value

        transaction.instructions.pop()

        transaction.add(
            SystemProgram.transfer({
                fromPubkey:from.publicKey,
                toPubkey:to,
                lamports:balance - (fee || 0)
            })
        )

        const signature = await sendAndConfirmTransaction(
            connection,
            transaction,
            [from]
        )
        console.log(`Success! Check out your TX here:https://explorer.solana.com/tx/${signature}?cluster=devnet`)
    } catch (error) {
        console.error(`OOPS Something went wrong ${error}`)
    }
})()