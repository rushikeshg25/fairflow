import * as anchor from '@coral-xyz/anchor';
import { Program } from '@coral-xyz/anchor';
import { Fairflow } from '../target/types/fairflow';
import { assert } from 'chai';

describe('fairflow', () => {
  const ENCRYPTION_KEY = 0xabcd;
  anchor.setProvider(anchor.AnchorProvider.env());
  const program = anchor.workspace.fairflow as Program<Fairflow>;
  const employer = anchor.web3.Keypair.generate();
  const companyName = 'companyt';
  const teamName = 'teamt';

  //feedback for/to
  const employeeName1 = 'employee1';
  const employee1_owned_salary_wallet = anchor.web3.Keypair.generate();

  //feedback from
  const employeeName2 = 'employee2';
  const employee2_owned_salary_wallet = anchor.web3.Keypair.generate();

  it('Company State initialized', async () => {
    try {
      const signature = await anchor
        .getProvider()
        .connection.requestAirdrop(
          employer.publicKey,
          anchor.web3.LAMPORTS_PER_SOL * 30
        );
      await anchor.getProvider().connection.confirmTransaction(signature);

      const tx = await program.methods
        .initializeCompanyState(companyName, 10, 5)
        .signers([employer])
        .accounts({
          employer: employer.publicKey,
        })
        .rpc();
      // console.log('Your transaction signature', tx);
    } catch (error) {
      console.log('An error occurred:', error);
    }
  });

  it('Funds Treasury', async () => {
    try {
      const companyPDA = anchor.web3.PublicKey.findProgramAddressSync(
        [
          Buffer.from('company'),
          Buffer.from(companyName),
          employer.publicKey.toBuffer(),
        ],
        program.programId
      )[0];

      const companyAccount = await program.account.company.fetch(companyPDA);

      const treasuryPDA = anchor.web3.PublicKey.findProgramAddressSync(
        [Buffer.from('treasury'), companyPDA.toBuffer()],
        program.programId
      )[0];

      const treasuryInitialBalance =
        await program.provider.connection.getBalance(treasuryPDA);

      const tx = await program.methods
        .fundTreasury(companyName, new anchor.BN(20))
        .accounts({
          employer: employer.publicKey,
        })
        .signers([employer])
        .rpc();

      // Check the new balance of the treasury
      const treasuryNewBalance = await program.provider.connection.getBalance(
        treasuryPDA
      );

      assert.equal(
        treasuryNewBalance - treasuryInitialBalance,
        20 * anchor.web3.LAMPORTS_PER_SOL
      );
    } catch (error) {
      console.log('An error occurred:', error);
      assert.fail(error.toString());
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
          employee1_owned_salary_wallet.publicKey.toBuffer(),
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
          employeeName1,
          employee1_owned_salary_wallet.publicKey,
          2,
          ENCRYPTION_KEY
        )
        .signers([employer])
        .accounts({
          employer: employer.publicKey,
        })
        .rpc();

      const employeeAccount = await program.account.employee.fetch(employeePDA);
      const teamAccount = await program.account.team.fetch(teamPDA);
      assert.equal(employeeAccount.employeeName, employeeName1);
      assert.equal(teamAccount.employees.length, 1);
      assert.equal(
        employeeAccount.encryptedCurrentSalary,
        encryptDecrypt(2, ENCRYPTION_KEY)
      );
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
          employeeName2,
          employee2_owned_salary_wallet.publicKey,
          3,
          ENCRYPTION_KEY
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
          employee1_owned_salary_wallet.publicKey.toBuffer(),
        ],
        program.programId
      );

      //Feedback from
      const [employeePDA2] = anchor.web3.PublicKey.findProgramAddressSync(
        [
          Buffer.from('employee'),
          Buffer.from(companyName),
          employee2_owned_salary_wallet.publicKey.toBuffer(),
        ],
        program.programId
      );

      const [teamPDA] = anchor.web3.PublicKey.findProgramAddressSync(
        [Buffer.from('team'), Buffer.from(teamName), Buffer.from(companyName)],
        program.programId
      );

      const tx1 = await program.methods
        .submitFeedback(
          employee1_owned_salary_wallet.publicKey,
          teamName,
          companyName,
          5
        )
        .accounts({
          employeeProvidingFeedback: employee2_owned_salary_wallet.publicKey,
        })
        .signers([employee2_owned_salary_wallet])
        .rpc();

      const employeeAccount1 = await program.account.employee.fetch(
        employeePDA1
      );

      assert.equal(employeeAccount1.currentTotalFeedbacks, 1);
      assert.equal(employeeAccount1.currentTotalFeedbackScore, 5);
    } catch (error) {
      console.log(error);
    }
  });

  it('Processes Payroll', async () => {
    try {
      const employerBalance = await program.provider.connection.getBalance(
        employer.publicKey
      );
      const requiredAmount = 10 * anchor.web3.LAMPORTS_PER_SOL;

      if (employerBalance < requiredAmount) {
        const additionalAmount =
          Math.ceil(
            (requiredAmount - employerBalance) / anchor.web3.LAMPORTS_PER_SOL
          ) + 1;
        const signature = await anchor
          .getProvider()
          .connection.requestAirdrop(
            employer.publicKey,
            additionalAmount * anchor.web3.LAMPORTS_PER_SOL
          );
        await anchor.getProvider().connection.confirmTransaction(signature);
      }

      const tx1 = await program.methods
        .fundTreasury(companyName, new anchor.BN(10))
        .accounts({
          employer: employer.publicKey,
        })
        .signers([employer])
        .rpc();

      const companyPDA = await anchor.web3.PublicKey.findProgramAddressSync(
        [
          Buffer.from('company'),
          Buffer.from(companyName),
          employer.publicKey.toBuffer(),
        ],
        program.programId
      )[0];

      const treasuryPDA = await anchor.web3.PublicKey.findProgramAddressSync(
        [Buffer.from('treasury'), companyPDA.toBuffer()],
        program.programId
      )[0];

      const treasuryInitialBalance =
        await program.provider.connection.getBalance(treasuryPDA);
      //Finding Employee PDA
      const [employeePDA] = await anchor.web3.PublicKey.findProgramAddressSync(
        [
          Buffer.from('employee'),
          Buffer.from(companyName),
          employee1_owned_salary_wallet.publicKey.toBuffer(),
        ],
        program.programId
      );

      // Process payroll
      const tx = await program.methods
        .processPayroll(
          teamName,
          companyName,
          employee1_owned_salary_wallet.publicKey,
          ENCRYPTION_KEY
        )
        .accounts({
          employer: employer.publicKey,
        })
        .signers([employer])
        .rpc();

      // Verify results
      const updatedEmployeeAccount = await program.account.employee.fetch(
        employeePDA
      );
      const finalTreasuryBalance = await program.provider.connection.getBalance(
        treasuryPDA
      );

      assert.equal(updatedEmployeeAccount.lastPayrollFeedback, 5);
      assert.equal(updatedEmployeeAccount.currentTotalFeedbackScore, 0);
      assert.equal(updatedEmployeeAccount.currentTotalFeedbacks, 0);
      assert.isBelow(finalTreasuryBalance, treasuryInitialBalance);
    } catch (error) {
      console.log('An error occurred:', error);
    }
  });
});
function encryptDecrypt(number, key) {
  return number ^ key;
}

const ENCRYPTION_KEY = 0xabcd;
