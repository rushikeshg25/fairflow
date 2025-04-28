import * as anchor from '@coral-xyz/anchor';
import { Program } from '@coral-xyz/anchor';
import { Fairflow } from '../target/types/fairflow';
import { assert } from 'chai';

describe('fairflow', () => {
  anchor.setProvider(anchor.AnchorProvider.env());
  const program = anchor.workspace.fairflow as Program<Fairflow>;
  const treasuryWallet = anchor.web3.Keypair.generate();
  const treasury = treasuryWallet.publicKey;
  const employer = anchor.web3.Keypair.generate();
  const companyName = 'companyt';
  const teamName = 'teamt';

  it('Company State initialized', async () => {
    try {
      const signature = await anchor
        .getProvider()
        .connection.requestAirdrop(
          employer.publicKey,
          anchor.web3.LAMPORTS_PER_SOL
        );
      await anchor.getProvider().connection.confirmTransaction(signature);

      const tx = await program.methods
        .initializeCompanyState(companyName, 10, 5, treasury)
        .signers([employer])
        .accounts({
          employer: employer.publicKey,
          treasury: treasury,
        })
        .rpc();
      console.log('Your transaction signature', tx);
    } catch (error) {
      console.log('An error occurred:', error);
    }
  });

  it('Creates Team', async () => {
    try {
      const [companyPDA] = anchor.web3.PublicKey.findProgramAddressSync(
        [
          Buffer.from('company'),
          Buffer.from(companyName),
          employer.publicKey.toBuffer(),
        ],
        program.programId
      );

      const [teamPDA] = anchor.web3.PublicKey.findProgramAddressSync(
        [
          Buffer.from('team'),
          Buffer.from(teamName),
          Buffer.from(companyName),
          employer.publicKey.toBuffer(),
        ],
        program.programId
      );

      const tx = await program.methods
        .createTeamState(teamName, companyName)
        .signers([employer])
        .accounts({
          employer: employer.publicKey,
        })
        .rpc();

      const companyAccount = await program.account.company.fetch(companyPDA);
      const isTeamInCompany = companyAccount.teams.some((pubkey) =>
        pubkey.equals(teamPDA)
      );

      console.log('Is team in company:', isTeamInCompany);
      console.log(
        'Company teams:',
        companyAccount.teams.map((pk) => pk.toString())
      );
      console.log('Team PDA:', teamPDA.toString());

      assert.isTrue(
        isTeamInCompany,
        "Team should be added to company's teams array"
      );
    } catch (error) {
      console.log('An error occurred:', error);
    }
  });
});
