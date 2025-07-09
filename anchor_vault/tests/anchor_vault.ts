import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { AnchorVault } from "../target/types/anchor_vault";
import { expect } from "chai";
import { LAMPORTS_PER_SOL } from "@solana/web3.js";

describe("anchor_vault", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  // space = VaultState::INIT_SPACE,
  //       seeds = [b"vaultState" , user.key().as_ref()],

  const program = anchor.workspace.anchorVault as Program<AnchorVault>;

  const vaultState = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("vaultState"), provider.wallet.publicKey.toBuffer()],
    program.programId
  )[0];

  const vault = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("vault"), vaultState.toBuffer()],
    program.programId
  )[0];

  it("should initialize vault!", async () => {
    // Add your test here.
    const tx = await program.methods
      .initialize()
      .accountsPartial({
        user: provider.wallet.publicKey,
        vaultState,
        vault,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();

    console.log(`Transaction signature:${tx}`);

    const vaultStateAccount = await program.account.vaultState.fetch(
      vaultState
    );

    expect(vaultStateAccount.stateBump).to.be.a("number");
    expect(vaultStateAccount.vaultBump).to.be.a("number");
  });

  it("should deposit Sol", async () => {
    const depositAmount = 0.2 * LAMPORTS_PER_SOL;
    const initialAmt = await provider.connection.getBalance(vault);

    const tx = await program.methods
      .deposit(new anchor.BN(depositAmount))
      .accountsPartial({
        user: provider.wallet.publicKey,
        vaultState,
        vault,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();

    console.log(`Transaction signature:${tx}`);

    const finalAmt = await provider.connection.getBalance(vault);

    expect(finalAmt - initialAmt).to.be.equal(depositAmount);
  });
});
