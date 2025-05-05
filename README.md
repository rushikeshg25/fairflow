# FairFlow - Decentralized Feedback Payroll

##  Project Overview

FairFlow is a decentralized, feedback-driven payroll management system built on **Solana**. It reimagines traditional compensation models by introducing transparent, on-chain salary distribution based on **peer feedback**. For Privacy, Salary is stored in **encrypted** form on chain.

Rather than depending solely on managerial assessments, FairFlow empowers teammates to contribute performance feedback. The **average feedback score** determines salary raises or deductions. Funds are held in **escrow via Program Derived Addresses (PDAs)** and released automatically to employees’ wallets based on **on-chain voting results**.

**Why FairFlow?**  
Current payroll systems are rigid and opaque. FairFlow introduces:
- Transparent, **performance-based compensation**
- **Democratized evaluation** process through peer feedback
- Trustless and **publicly verifiable** payroll logic

## Technical Details

### Tech Stack
- **Blockchain Platform**: Solana
- **Smart Contracts**: Rust (via Anchor)
- **Frontend**: Next.js (In progress)
- **Backend**: Anchor, Next.js Server Actions
- **Authentication**: Solana Wallet Adapter Login


### Key Program Modules
States and Instructions are seperated to provide good Dev experience
#### `/instructions`
Contains the core instructions (smart contract entrypoints) responsible for executing business logic. Each file maps to a specific user or system action:

- `initialize_company.rs` – Creates the company PDA and initialises Treasury
- `create_team.rs` – Initializes a new team under the company
- `register_employee.rs` – Registers an employee PDA and assigns it to a team
- `submit_feedback.rs` – Allows team members to submit peer feedback
- `fund_treasury.rs` – Adds funds to the company or team’s payroll treasury
- `process_payroll.rs` – Releases salary based on feedback scores and on-chain voting

#### `/states`
Defines the custom data structures stored on-chain. Each struct is designed to be used with Anchor's account system:

- `company.rs` – Company account holding treasury and config
- `team.rs` – Team-level metadata and configuration
- `employee.rs` – Employee data including feedback score and payout logic


## Testing
All instructions are tested 
</br>
![image](https://github.com/user-attachments/assets/b1200962-52d9-49f3-844c-e860c6015bc9)

