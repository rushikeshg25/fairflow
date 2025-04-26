import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Fairflow } from "../target/types/fairflow";

describe("fairflow", () => {
  anchor.setProvider(anchor.AnchorProvider.env());
  const program = anchor.workspace.fairflow as Program<Fairflow>;
  const treasuryWallet = anchor.web3.Keypair.generate();
  const treasury = treasuryWallet.publicKey;
  const employer = anchor.web3.Keypair.generate();

  it("Company State initialized", async () => {
    try {
      const signature = await anchor
        .getProvider()
        .connection.requestAirdrop(
          employer.publicKey,
          anchor.web3.LAMPORTS_PER_SOL
        );
      await anchor.getProvider().connection.confirmTransaction(signature);

      const tx = await program.methods
        .initializeCompanyState("Test Company", 10, 5, treasury)
        .signers([employer])
        .accounts({
          employer: employer.publicKey,
          treasury: treasury,
        })
        .rpc();
      console.log("Your transaction signature", tx);
    } catch (error) {
      console.error("An error occurred:", error);
    }
  });
});
