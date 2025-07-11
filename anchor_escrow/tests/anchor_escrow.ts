import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { AnchorEscrow } from "../target/types/anchor_escrow";
import {
  Keypair,
  LAMPORTS_PER_SOL,
  SystemProgram,
  PublicKey,
  Connection,
} from "@solana/web3.js";
import { randomBytes, sign } from "node:crypto";
import {
  Account,
  createMint,
  getAssociatedTokenAddressSync,
  getOrCreateAssociatedTokenAccount,
  mintTo,
} from "@solana/spl-token";
import { BN } from "bn.js";

describe("anchor_escrow", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.anchorEscrow as Program<AnchorEscrow>;

  const connection = provider.connection;

  let maker: Keypair;
  let taker: Keypair;
  let mintA: anchor.web3.PublicKey;
  let mintB: anchor.web3.PublicKey;
  let takerAtaA: Account;
  let takerAtaB: Account;
  let makerAtaA: Account;
  let makerAtaB: Account;
  let escrow: anchor.web3.PublicKey;
  let vault: anchor.web3.PublicKey;
  let bump: number;

  const seed = new BN(randomBytes(4));
  before(async () => {
    maker = anchor.web3.Keypair.generate();
    taker = anchor.web3.Keypair.generate();

    await airdrop(connection, maker.publicKey);
    await airdrop(connection, taker.publicKey);

    mintA = await createMint(connection, maker, maker.publicKey, null, 6);

    console.log(`Mint A address:${mintA}`);

    mintB = await createMint(connection, taker, taker.publicKey, null, 6);

    console.log(`Mint B address:${mintB}`);

    makerAtaA = await getOrCreateAssociatedTokenAccount(
      connection,
      maker,
      mintA,
      maker.publicKey
    );

    console.log(`Maker_ATA_A address:${makerAtaA}`);

    takerAtaA = await getOrCreateAssociatedTokenAccount(
      connection,
      taker,
      mintA,
      taker.publicKey
    );

    console.log(`Taker_ATA_A address:${takerAtaA}`);

    makerAtaB = await getOrCreateAssociatedTokenAccount(
      connection,
      maker,
      mintB,
      maker.publicKey
    );

    console.log(`Maker_ATA_B address:${makerAtaB}`);

    takerAtaB = await getOrCreateAssociatedTokenAccount(
      connection,
      taker,
      mintB,
      taker.publicKey
    );

    console.log(`Taker_ATA_B address:${takerAtaB}`);

    let mintToMakerAtaA = await mintTo(
      connection,
      maker,
      mintA,
      makerAtaA.address,
      maker,
      100000 * 10 ** 6
    );
    console.log(`Minted to makerAtaA:${mintToMakerAtaA}`);

    let mintToTakerAtaB = await mintTo(
      connection,
      taker,
      mintB,
      takerAtaB.address,
      taker,
      100000 * 10 ** 6
    );
    console.log(`Minted to makerAtaA:${mintToMakerAtaA}`);


    [escrow, bump] = anchor.web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from("escrow"),
        maker.publicKey.toBuffer(),
        seed.toArrayLike(Buffer, "le", 6),
      ],
      program.programId
    );

    console.log(`Escrow account created at:${escrow}`);

    vault = getAssociatedTokenAddressSync(mintA, escrow, true);

    console.log(`Vault address:${vault}`)
  });

  
  it("Is initialized!", async () => {
    // Add your test here.
  });
});

const airdrop = async (
  connection: anchor.web3.Connection,
  publicKey: PublicKey
) => {
  const signature = await connection.requestAirdrop(
    publicKey,
    2 * LAMPORTS_PER_SOL
  );

  console.log(`Airdrop signature:${signature}`);

  const confirm_airdrop = await connection.confirmTransaction(
    signature,
    "confirmed"
  );

  console.log(`tx signature ${confirm_airdrop}`);
  return confirm_airdrop;
};
