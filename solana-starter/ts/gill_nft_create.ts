import {
  createSolanaClient,
  generateKeyPairSigner,
  getExplorerLink,
  getSignatureFromTransaction,
  signTransactionMessageWithSigners,
} from "gill";
import { loadKeypairSignerFromFile } from "gill/node";
import {
  buildCreateTokenTransaction,
  TOKEN_2022_PROGRAM_ADDRESS,
} from "gill/programs/token";

const { rpc, sendAndConfirmTransaction } = createSolanaClient({
  urlOrMoniker: "https://devnet.helius-rpc.com/?api-key=71d05d9f-5d94-4548-9137-c6c3d9f69b3e",
});

(async () => {
  try {
    const signer = await loadKeypairSignerFromFile("Turbin3-wallet.json");
    const { value: latestBlockhash } = await rpc.getLatestBlockhash().send();

    const mint = await generateKeyPairSigner();

    const tx = await buildCreateTokenTransaction({
      feePayer: signer,
      version: "legacy",
      decimals: 9,
      metadata: {
        name: "RUG DAY",
        symbol: "$RUG",
        sellerFeeBasisPoints:10,
        isMutable: true,
        uri: "https://devnet.irys.xyz/8rjNKB3W35Qdx9eWBT7KEksSqpuVwXnHXdMfytm6nLCG",
      },
      mint,
      latestBlockhash,
      tokenProgram:TOKEN_2022_PROGRAM_ADDRESS,
    });

    const signedTransaction = await signTransactionMessageWithSigners(tx);

    
    console.log(
      `Explorer link:${getExplorerLink({
        cluster: "devnet",
        transaction: getSignatureFromTransaction(signedTransaction),
      })}`
    );


    await sendAndConfirmTransaction(signedTransaction);
  } catch (error) {
    console.log(`OOPS,Something went wrong ${error}`);
  }
})();
