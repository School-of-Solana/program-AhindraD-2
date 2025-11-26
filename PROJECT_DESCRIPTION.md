![School of Solana](https://github.com/Ackee-Blockchain/school-of-solana/blob/master/.banner/banner.png?raw=true)

# Project Description

- ## Deployed Frontend URL: [https://prism-papers-dapp.vercel.app/](https://prism-papers-dapp.vercel.app/)

- ## Solana Program ID: [2nvhRn83KBxkkAfLH64meTq8cYB5aRLnZVbsxZdgfPTv](https://solscan.io/account/2nvhRn83KBxkkAfLH64meTq8cYB5aRLnZVbsxZdgfPTv?cluster=devnet)

## Project Overview

### Description

PrismPapers is a Decentralized Science (DeSci) platform built on Solana that revolutionizes academic publishing by making research transparent, accessible, and fairly rewarded. It tackles the inefficiencies of traditional publishing—such as opaque review processes, high access fees that don't benefit authors, and slow publication times—by leveraging blockchain technology.

The platform allows researchers to publish papers as encrypted assets using Lit Protocol for access control and Arweave for permanent storage. It introduces a "Token Gated" read mechanism where access is granted only upon purchase, ensuring authors are directly compensated. Additionally, it features an incentivized peer review system where reviewers are rewarded for their contributions, fostering a collaborative and high-quality academic ecosystem.

### Key Features

PrismPapers enables a fully decentralized loop of publishing, purchasing, and reviewing:

  - **Encrypted Publishing:** Authors can publish research papers where the content is encrypted. The decryption key is securely managed via Lit Protocol and only released to verified buyers.
  - **Direct Author Compensation:** When a user purchases a paper, the majority of the funds are routed directly to the author's vault, bypassing traditional intermediary publishers.
  - **Token-Gated Access:** Access to read a paper is strictly controlled by on-chain `AccessReceipt` PDAs. Only users who have purchased the paper can decrypt and view the content.
  - **Incentivized Peer Review:** Users who have purchased a paper can submit reviews. Authors can accept these reviews, which triggers an automatic reward payout to the reviewer from the author's earnings.
  - **Transparent Fee Model:** The platform takes a small, transparent percentage fee (5%) on transactions to sustain operations, while the rest goes to the content creators.

### How to Use the dApp

1.  **Connect Wallet:** Click the "Select Wallet" button in the top right corner to connect your Solana wallet (e.g., Phantom, Backpack).
2.  **Initialize Profile:** Before interacting, create your Researcher Profile by entering your name. This initializes your earning vaults on-chain.
3.  **Publish Research:** - Navigate to the "Publish" tab.
      - Upload your PDF (it gets encrypted automatically).
      - Enter the title, description, and price (in SOL).
      - Sign the transaction to create the on-chain Research Paper account.
4.  **Buy & Read:** - Browse the dashboard for interesting papers.
      - Click "Buy Access" to pay the listed price.
      - Once the transaction is confirmed, the "Read" button becomes active, decrypting the PDF for your eyes only.
5.  **Submit a Review:**
      - After reading, you can submit a peer review by providing a link to your feedback (e.g., IPFS/Arweave link) and a proposed reward amount.
6.  **Withdraw Earnings:**
      - Check your profile dashboard to see your accumulated earnings from sales or reviews.
      - Click "Withdraw" to move funds from the program vault to your personal wallet.

## Program Architecture

The PrismPapers program is architected around a dual-vault system (User Vaults vs. Admin Vault) to separate user funds from platform fees, ensuring security and trust. It uses a "Pull Payment" pattern where earnings accumulate in a vault and must be explicitly withdrawn by the user.

### PDA Usage

We use Program Derived Addresses (PDAs) extensively to map relationships between users, papers, and reviews deterministically without needing a centralized database.

**PDAs Used:**

  - **User Profile (`"user"`, owner):** Stores user stats (papers sold, purchased, reputation) and acts as the anchor for their identity.
  - **User Vault (`"vault_user"`, owner):** A System Account derived from the user's key. This holds the SOL earned by the user (from sales) until they withdraw it.
  - **Research Paper (`"paper"`, author):** Stores metadata, the price, and the Lit Protocol encrypted key required to read the file.
  - **Access Receipt (`"receipt"`, buyer, paper):** The "Ticket" PDA. Its existence proves a specific user purchased a specific paper. This is checked before granting decryption access or allowing a review.
  - **Peer Review (`"review"`, reviewer, paper):** Stores the review content URL and status (Pending/Accepted/Rejected).
  - **Admin Vault (`"vault_admin"`):** Collects the 5% platform fee from all transactions.

### Program Instructions

**Instructions Implemented:**

  - **`init_user`:** Creates a User Profile and their associated User Vault.
  - **`init_research`:** Publishes a new paper. Sets the price, stores the encrypted URL, and the Lit Protocol decryption key.
  - **`purchase_access`:** Handles the logic of buying a paper. It splits the payment (95% to Author Vault, 5% to Admin Vault), creates an `AccessReceipt` PDA for the buyer, and updates sales stats.
  - **`review_paper`:** Allows a verified buyer (checked via `AccessReceipt`) to submit a review proposal with a requested reward.
  - **`verify_review`:** Allows the Author to accept a review. If accepted, it transfers the proposed reward from the Author's Vault to the Reviewer's Vault and marks the review as Accepted.
  - **`update_research`:** Allows the author to update metadata (title, description) or price.
  - **`user_withdraw`:** Allows a user to pull their accumulated earnings from their User Vault to their wallet.
  - **`admin_withdraw`:** Allows the platform admin to withdraw accumulated fees.

### Account Structure

Below are the primary data structures used in the program:

```rust
pub struct User {
    pub owner: Pubkey,
    #[max_len(USER_NAME_MAX_LENGTH)]
    pub name: String,
    pub published: u16,
    pub purchased: u16,
    pub sold: u16,
    pub reviewed: u16,
    pub earning: u64,
    pub timestamp: i64,
    pub bump: u8,
}

pub struct ResearchPaper {
    pub author: Pubkey,
    #[max_len(PAPER_TITLE_MAX_LENGTH)]
    pub title: String,
    #[max_len(PAPER_DESCRIPTION_MAX_LENGTH)]
    pub description: String,
    pub price: u64,
    pub sales: u32,
    pub reviews: u32,
    #[max_len(PAPER_URL_MAX_LENGTH)]
    pub encrypted_url: String,
    #[max_len(ENCRYPTION_KEY_MAX_LENGTH)]
    pub encryption_key: String,
    pub timestamp: i64,
    pub bump: u8,
}

pub struct AccessReceipt {
    pub buyer: Pubkey,
    pub purchased_paper: Pubkey,
    pub timestamp: i64,
    pub bump: u8,
}

pub struct PeerReview {
    pub reviewer: Pubkey,
    pub reviewed_paper: Pubkey,
    #[max_len(REVIEW_URL_MAX_LENGTH)]
    pub review_url: String,
    pub status: ReviewStatus,
    pub proposed_reward: u64,
    pub timestamp: i64,
    pub bump: u8,
}
```

## Testing

### Test Coverage

We have implemented a comprehensive test suite using **Vitest** and **Gill** (Solana Kit) to verify the integrity of the financial flows and access controls.

**Happy Path Tests:**

  - **User Initialization:** Verifies a user can create a profile and vault.
  - **Publishing:** Verifies an author can create a paper with valid metadata and price.
  - **Purchasing:** Verifies a buyer can purchase a paper, ensuring the 5% fee goes to the Admin Vault and 95% goes to the Author Vault. Checks that the `AccessReceipt` is created.
  - **Reviewing:** Verifies a buyer can submit a review.
  - **Verifying Review:** Verifies an author can accept a review, triggering a fund transfer from Author Vault to Reviewer Vault.
  - **Withdrawals:** Verifies users and admins can withdraw their respective earnings.

**Unhappy Path Tests:**

  - **Self-Purchase:** Verifies an author cannot buy their own paper.
  - **Unfunded Purchase:** Verifies transaction fails if the buyer lacks SOL.
  - **Unauthorized Review:** Verifies a user cannot review a paper they haven't purchased (checks for missing `AccessReceipt`).
  - **Over-withdrawal:** Verifies a user cannot withdraw more funds than recorded in their `earning` ledger, even if the vault physically has more SOL.
  - **Unauthorized Admin:** Verifies non-admin wallets cannot withdraw from the Admin Vault.
  - **Zero Price:** Verifies a paper cannot be published with a price of 0.

### Running Tests

To run the test suite locally:

```bash
# Install dependencies if needed
pnpm install


# Run the tests
pnpm run anchor-test
```

### Additional Notes for Evaluators

  - **Lit Protocol Integration:** The frontend handles the actual encryption/decryption. The Solana program stores the *keys* to this encryption. During testing/localnet, dummy strings are used for the `encryption_key` field, but on the live Devnet dApp, these are real keys generated by the Lit SDK.
  - **Vault Logic:** We use a "Ledger vs. Physical" check. `user.earning` acts as the ledger. A user can only withdraw if `earning >= amount` AND `vault.lamports >= amount`. This prevents any potential math errors or re-entrancy issues from draining funds.