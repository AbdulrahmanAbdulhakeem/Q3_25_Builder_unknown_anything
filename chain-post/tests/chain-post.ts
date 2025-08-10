import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { ChainPost } from "../target/types/chain_post";
import { BN } from "bn.js";
import { randomBytes } from "crypto";
import {
  fetchMerkleTree,
  getCurrentRoot,
} from "@metaplex-foundation/spl-account-compression";
import { dasApi } from "@metaplex-foundation/digital-asset-standard-api";
import {
  Keypair,
  PublicKey,
  sendAndConfirmTransaction,
  Transaction,
} from "@solana/web3.js";
import {
  LeafSchema,
  MPL_BUBBLEGUM_PROGRAM_ID,
} from "@metaplex-foundation/mpl-bubblegum";
import tipper_wallet from "../tipper-wallet.json";
import wallet from "../Turbin3-wallet.json";
import merkle_tree_wallet from "../merkle_tree-wallet.json";
import { fromWeb3JsKeypair } from "@metaplex-foundation/umi-web3js-adapters";

import {
  ConcurrentMerkleTreeAccount,
  ValidDepthSizePair,
  createAllocTreeIx,
} from "@solana/spl-account-compression";
import assert from "assert";
import {
  keypairIdentity,
  publicKey,
} from "@metaplex-foundation/umi";
import { createUmi } from "@metaplex-foundation/umi-bundle-defaults";
import { irysUploader } from "@metaplex-foundation/umi-uploader-irys";
import fs from "fs";
import {
  findLeafAssetIdPda,
  mintV1,
  mplBubblegum,
  parseLeafFromMintV1Transaction,
} from "@metaplex-foundation/mpl-bubblegum";
import { none } from "@metaplex-foundation/umi";
import {
  mplTokenMetadata,
} from "@metaplex-foundation/mpl-token-metadata";
import { base58 } from "@metaplex-foundation/umi/serializers";
import { TOKEN_2022_PROGRAM_ID } from "@solana/spl-token";

describe("chain-post", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());
  const provider = anchor.getProvider();

  const program = anchor.workspace.chainPost as Program<ChainPost>;

  const connection = provider.connection;
  const wallet2 = provider.wallet as anchor.Wallet;

  const umi = createUmi(
    "https://devnet.helius-rpc.com/?api-key=12262688-1afc-448b-943f-b2fa77d23f4e"
  )
    .use(dasApi())
    .use(mplBubblegum())
    .use(mplTokenMetadata())
    .use(
      irysUploader({
        // mainnet address: "https://node1.irys.xyz"
        // devnet address: "https://devnet.irys.xyz"
        address: "https://devnet.irys.xyz",
      })
    );

  const walletFile = fs.readFileSync("./Turbin3-wallet.json", "utf-8");
  const secretKey = JSON.parse(walletFile);
  let keypair = umi.eddsa.createKeypairFromSecretKey(new Uint8Array(secretKey));
  umi.use(keypairIdentity(keypair));

  const maxDepthSizePair: ValidDepthSizePair = {
    maxDepth: 14,
    maxBufferSize: 64,
  };

  const canopyDepth = 11;

  const tipper = anchor.web3.Keypair.fromSecretKey(
    new Uint8Array(tipper_wallet)
  );

  const admin = anchor.web3.Keypair.fromSecretKey(new Uint8Array(wallet));

  const merkleTree = anchor.web3.Keypair.fromSecretKey(
    new Uint8Array(merkle_tree_wallet)
  );

  let leaf: LeafSchema;
  let assetId;
  const umiMerkleTree = fromWeb3JsKeypair(merkleTree);

  let nft_mint;

  const confirm = async (signature: string): Promise<string> => {
    const block = await connection.getLatestBlockhash();
    await connection.confirmTransaction({
      signature,
      ...block,
    });

    return signature;
  };

  const log = async (signature: string): Promise<string> => {
    console.log(
      `Your transaction signature: https://explorer.solana.com/transaction/${signature}?cluster=custom&customUrl=${connection.rpcEndpoint}`
    );
    return signature;
  };

  const seed = new BN(randomBytes(8));

  const platform_config = PublicKey.findProgramAddressSync(
    [Buffer.from("platformConfig"), admin.publicKey.toBuffer()],
    program.programId
  )[0];

  console.log(platform_config);

  const user_account1 = PublicKey.findProgramAddressSync(
    [Buffer.from("user"), admin.publicKey.toBuffer()],
    program.programId
  )[0];

  const user_account2 = PublicKey.findProgramAddressSync(
    [Buffer.from("user"), tipper.publicKey.toBuffer()],
    program.programId
  )[0];

  const post_account = PublicKey.findProgramAddressSync(
    [Buffer.from("post"), admin.publicKey.toBuffer(), seed.toBuffer("le", 8)],
    program.programId
  )[0];

  const tree_config = PublicKey.findProgramAddressSync(
    [merkleTree.publicKey.toBuffer()],
    new PublicKey(MPL_BUBBLEGUM_PROGRAM_ID)
  )[0];


  it("initialize the platform and create a merkle_tree", async () => {
    console.log("Merkle tree public key:", merkleTree.publicKey.toBase58());
    console.log("Admin public key:", admin.publicKey.toBase58());
    console.log("Platform config:", platform_config.toBase58());
    console.log("Tree config:", tree_config.toBase58());
    // console.log(wallet2)

    const builder = await createAllocTreeIx(
      connection,
      merkleTree.publicKey,
      admin.publicKey,
      maxDepthSizePair,
      canopyDepth
    );

    const builderTx = new Transaction().add(builder);

    const signature = await sendAndConfirmTransaction(
      connection,
      builderTx,
      [wallet2.payer, merkleTree],
      {
        commitment: "confirmed",
      }
    ).then(log);

    const tx = await program.methods
      .initialize(14, 64)
      .accounts({
        admin: admin.publicKey,
        merkleTree: merkleTree.publicKey,
      })
      .signers([admin, merkleTree])
      .rpc()
      .then(confirm)
      .then(log);

    const treeAccount = await ConcurrentMerkleTreeAccount.fromAccountAddress(
      connection,
      merkleTree.publicKey
    );

    console.log("MaxBufferSize", treeAccount.getMaxBufferSize());
    console.log("MaxDepth", treeAccount.getMaxDepth());
    console.log("Tree Authority", treeAccount.getAuthority().toString());

    assert.strictEqual(
      treeAccount.getMaxBufferSize(),
      maxDepthSizePair.maxBufferSize
    );
    assert.strictEqual(treeAccount.getMaxDepth(), maxDepthSizePair.maxDepth);
  });

  it("should initialize user", async () => {
    const tx = await program.methods
      .initializeUser()
      .accounts({
        payer: admin.publicKey,
      })
      .signers([admin])
      .rpc()
      .then(confirm)
      .then(log);

    const tx2 = await program.methods
      .initializeUser()
      .accounts({
        payer: tipper.publicKey,
      })
      .signers([tipper])
      .rpc()
      .then(confirm)
      .then(log);
  });

  it("should create post", async () => {
    const image =
      "https://gateway.irys.xyz/8rjNKB3W35Qdx9eWBT7KEksSqpuVwXnHXdMfytm6nLCG";

    let content =
      "This is the first chainpost by the admin,welcome to the platform chainers";

    const metadata = {
      name: "ChainRug",
      symbol: "$ChainPost",
      description: content,
      image,
      attributes: [{ trait_type: "type", value: "RUG" }],
      properties: {
        files: [
          {
            type: "image/png",
            uri: image,
          },
        ],
      },
      creators: [],
    };

    const myUri = await umi.uploader.uploadJson(metadata);
    console.log("Your metadata URI: ", myUri);

    console.log("Minting Compressed NFT...");

    const { signature } = await mintV1(umi, {
      leafOwner: umi.identity.publicKey,
      merkleTree: publicKey(merkleTree.publicKey),
      metadata: {
        name: "ChainPost",
        uri: myUri,
        sellerFeeBasisPoints: 550,
        collection: none(),
        creators: [],
      },
    }).sendAndConfirm(umi, {
      send: { maxRetries: 50, skipPreflight: true },
      confirm: { commitment: "finalized" },
    });

    console.log(
      "Mint V1 Transaction Signature (Base58):",
      base58.deserialize(signature)[0]
    );

    await new Promise((resolve) => setTimeout(resolve, 3000));

    leaf = await parseLeafFromMintV1Transaction(umi, signature);
  
    assetId = findLeafAssetIdPda(umi, {
      merkleTree: umiMerkleTree.publicKey,
      leafIndex: Number(leaf.nonce),
    });
    console.log("Inputs to findLeafAssetIdPda:", {
      merkleTree: umiMerkleTree.publicKey.toString(),
      leafIndex: leaf.nonce.toString(),
    });
    console.log("Generated assetId:", assetId.toString());
    console.log("assetId", assetId);
    console.log("leaf:", leaf);

    const tx = await program.methods
      .createPost(seed, content)
      .accountsPartial({
        contentCreator: admin.publicKey,
        admin: admin.publicKey,
        merkleTree: merkleTree.publicKey,
        platformConfig:platform_config,
      })
      .signers([admin])
      .rpc()
      .then(confirm)
      .then(log);
  });

  it("should tip post", async () => {
    const tx = await program.methods
      .tipPost(seed,new BN(10000))
      .accountsPartial({
        tipper: tipper.publicKey,
        contentCreator: admin.publicKey,
        postAccount:post_account
      })
      .signers([tipper])
      .rpc()
      .then(confirm)
      .then(log);
  });

  it("should comment on post", async () => {
    let title = "User comment";
    let comment = "inciteful";

    let comment_account = PublicKey.findProgramAddressSync(
      [
        Buffer.from("comment"),
        post_account.toBuffer(),
        seed.toBuffer("le", 8),
        tipper.publicKey.toBuffer(),
      ],
      program.programId
    )[0];

    console.log("Post Account PDA:", post_account.toString());
console.log("Comment Account PDA:", comment_account.toString());

    const tx = await program.methods
      .commentOnPost(seed, title, comment)
      .accountsPartial({
        commenter: tipper.publicKey,
        author: admin.publicKey,
        postAccount:post_account,
        commentAccount:comment_account
      })
      .signers([tipper])
      .rpc()
      .then(confirm)
      .then(log);


    console.log(comment_account);
  });

    it("should buy post nft", async () => {

    nft_mint = Keypair.generate();
   
    const amount = new BN(10000);
    const uri = " https://gateway.irys.xyz/4veGeQyTgxtravcxVCXwTN7p7VCDV3PTi7Uih5E3qhX7"
    const tx = await program.methods
      .buyPostNft(amount,'First Post' , uri)
      .accountsPartial({
        buyer:tipper.publicKey,
        author:admin.publicKey,
        nftMint:nft_mint.publicKey,
        postAccount:post_account,
        userAccount:user_account2,
        tokenProgram:TOKEN_2022_PROGRAM_ID
      })
      .signers([tipper,nft_mint])
      .rpc()
      .then(confirm)
      .then(log);
  });

  it("should delete post", async () => {
    let merkleTreeAccount = await fetchMerkleTree(umi, umiMerkleTree.publicKey);
    console.log(merkleTreeAccount)

    const root = Array.from(getCurrentRoot(merkleTreeAccount.tree));
    const data_hash = Array.from(leaf.dataHash);
    const creator_hash = Array.from(leaf.creatorHash);
    const nonce = new BN(leaf.nonce);

    const tx = await program.methods
      .deletePost(root, data_hash, creator_hash, nonce, Number(leaf.nonce))
      .accountsPartial({
        creatorOrAdmin: admin.publicKey,
        merkleTree: merkleTree.publicKey,
        bubblegumProgram: MPL_BUBBLEGUM_PROGRAM_ID,
        postAccount:post_account
      })
      .signers([admin])
      .rpc()
      .then(confirm)
      .then(log);
  });

});
