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

  //feedback for/to
  const salaryAccount1 = anchor.web3.Keypair.generate();
  const employeeName1 = 'employee1';

  //feedback from
  const salaryAccount2 = anchor.web3.Keypair.generate();
  const employeeName2 = 'employee2';

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
      // console.log('Your transaction signature', tx);
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
        [Buffer.from('team'), Buffer.from(teamName), Buffer.from(companyName)],
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

      // console.log('Is team in company:', isTeamInCompany);
      // console.log(
      //   'Company teams:',
      //   companyAccount.teams.map((pk) => pk.toString())
      // );
      // console.log('Team PDA:', teamPDA.toString());

      assert.isTrue(
        isTeamInCompany,
        "Team should be added to company's teams array"
      );
    } catch (error) {
      console.log('An error occurred:', error);
    }
  });

  it('Registers Employee', async () => {
    try {
      const [employeePDA] = anchor.web3.PublicKey.findProgramAddressSync(
        [
          Buffer.from('employee'),
          Buffer.from(companyName),
          salaryAccount1.publicKey.toBuffer(),
        ],
        program.programId
      );

      const [teamPDA] = anchor.web3.PublicKey.findProgramAddressSync(
        [Buffer.from('team'), Buffer.from(teamName), Buffer.from(companyName)],
        program.programId
      );

      const tx = await program.methods
        .registerEmployee(
          teamName,
          companyName,
          salaryAccount1.publicKey,
          employeeName1
        )
        .signers([employer])
        .accounts({
          employer: employer.publicKey,
        })
        .rpc();

      const employeeAccount = await program.account.employee.fetch(employeePDA);
      const teamAccount = await program.account.team.fetch(teamPDA);
      // console.log('Team Account:', teamAccount);
      // console.log('Employee Account:', employeeAccount);
      // console.log('Employee PDA:', employeePDA.toString());
      assert.equal(employeeAccount.employeeName, employeeName1);
      assert.equal(teamAccount.employees.length, 1);
    } catch (error) {
      console.log('An error occurred:', error);
    }
  });

  it('Submits Feedback', async () => {
    try {
      //registering employee2
      const tx = await program.methods
        .registerEmployee(
          teamName,
          companyName,
          salaryAccount2.publicKey,
          employeeName2
        )
        .signers([employer])
        .accounts({
          employer: employer.publicKey,
        })
        .rpc();

      //Feedback for/to
      const [employeePDA1] = anchor.web3.PublicKey.findProgramAddressSync(
        [
          Buffer.from('employee'),
          Buffer.from(companyName),
          salaryAccount1.publicKey.toBuffer(),
        ],
        program.programId
      );

      //Feedback from
      const [employeePDA2] = anchor.web3.PublicKey.findProgramAddressSync(
        [
          Buffer.from('employee'),
          Buffer.from(companyName),
          salaryAccount2.publicKey.toBuffer(),
        ],
        program.programId
      );

      const [teamPDA] = anchor.web3.PublicKey.findProgramAddressSync(
        [Buffer.from('team'), Buffer.from(teamName), Buffer.from(companyName)],
        program.programId
      );
      // console.log(employer.publicKey.toString());
      // console.log(employeePDA2.toString());
      // console.log(salaryAccount2.publicKey.toString());
      const tx1 = await program.methods
        .submitFeedback(salaryAccount1.publicKey, teamName, companyName, 2)
        .accounts({
          employeeProvidingFeedback: salaryAccount2.publicKey,
        })
        .signers([salaryAccount2])
        .rpc();

      const employeeAccount1 = await program.account.employee.fetch(
        employeePDA1
      );
      const employeeAccount2 = await program.account.employee.fetch(
        employeePDA2
      );
      const teamAccount = await program.account.team.fetch(teamPDA);

      assert.equal(employeeAccount1.currentTotalFeedbacks, 1);
      assert.equal(employeeAccount1.currentTotalFeedbackScore, 2);
    } catch (error) {
      console.log(error);
    }
  });
});
