import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { AnchorDiceGame } from "../target/types/anchor_dice_game";
import { Keypair, LAMPORTS_PER_SOL, PublicKey } from "@solana/web3.js";
import { BN } from "bn.js";
import { randomBytes } from "crypto";
import { assert } from "console";
import { expect, should } from "chai";

describe("anchor_dice_game", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());
  const provider = anchor.getProvider();

  const program = anchor.workspace.anchorDiceGame as Program<AnchorDiceGame>;

  const connection = provider.connection;

  const confirm = async (signature: string): Promise<string> => {
    const block = await connection.getLatestBlockhash();

    await connection.confirmTransaction({
      signature,
      ...block,
    });
    return signature;
  };

  let house = new Keypair();
  let player = new Keypair();
  const seed = new BN(randomBytes(16));

  // seeds = [b"bet" ,  vault.key().as_ref(),seed.to_le_bytes().as_ref()],

  const vault = PublicKey.findProgramAddressSync(
    [Buffer.from("vault"), house.publicKey.toBuffer()],
    program.programId
  )[0];

  const bet = PublicKey.findProgramAddressSync(
    [Buffer.from("bet"), vault.toBuffer(), seed.toBuffer("le", 16)],
    program.programId
  )[0];

  const roll = 50;
  const topup = new BN(40 * LAMPORTS_PER_SOL);

  const betAmount = LAMPORTS_PER_SOL / 100;

  const log = async (signature: string): Promise<string> => {
    console.log(
      `Your transaction signature: https://explorer.solana.com/transaction/${signature}?cluster=custom&customUrl=${connection.rpcEndpoint}`
    );
    return signature;
  };

  it("should airdrop house and player!", async () => {
    // Add your test here.
    await Promise.all(
      [house, player].map(async (k) => {
        return await connection
          .requestAirdrop(k.publicKey, 1000 * LAMPORTS_PER_SOL)
          .then(confirm)
          .then(log);
      })
    );
  });

  it("should initialize the vault", async () => {
    const tx = await program.methods
      .initialize(topup)
      .accounts({ house: house.publicKey })
      .signers([house])
      .rpc()
      .then(confirm)
      .then(log);

    const vaultBalance = await connection.getBalance(vault);

    expect(vaultBalance.toString()).to.eql(topup.toString());
  });

  it("should place a bet", async () => {
    const tx = await program.methods
      .placeBet(seed, topup, roll)
      .accounts({
        player: player.publicKey,
        house: house.publicKey,
      })
      .signers([player])
      .rpc()
      .then(confirm)
      .then(log);

    const betAccount = await program.account.bet.fetch(bet);

    const vaultBalance = await connection.getBalance(vault);

    expect(betAccount.roll).to.eq(roll);
    expect(betAccount.amount.toString()).to.eq(topup.toString());
    expect(betAccount.player.toBase58()).to.eq(player.publicKey.toBase58());
    expect(vaultBalance.toString()).to.eq(topup.muln(2).toString());
  });

  it("should refund bet", async () => {
    const vaultAccountBefore = await connection.getBalance(vault);

    console.log(vaultAccountBefore);

    const tx = await program.methods
      .refundBet()
      .accounts({
        player: player.publicKey,
        house: house.publicKey,
        bet
      })
      .signers([player])
      .rpc()
      .then(confirm)
      .then(log);

    const vaultAccountAfter = await connection.getBalance(vault);

    expect((vaultAccountBefore - vaultAccountAfter).toString()).to.eq(topup.toString());
  });

  
});
