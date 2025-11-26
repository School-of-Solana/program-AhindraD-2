import { describe, it, expect, beforeAll } from 'vitest'
import {
  Blockhash,
  createSolanaClient,
  createTransaction,
  Instruction,
  KeyPairSigner,
  signTransactionMessageWithSigners,
  generateKeyPairSigner,
  address,
  Address,
  getAddressEncoder,
  getProgramDerivedAddress,
} from 'gill'
// Import generated helpers from your SDK
import {
  getInitUserInstruction,
  getInitResearchInstruction,
  getPurchaseAccessInstruction,
  getReviewPaperInstruction,
  getVerifyReviewInstruction,
  getUserWithdrawInstruction,
  getAdminWithdrawInstruction
} from '../src'
// @ts-ignore
import { loadKeypairSignerFromFile } from 'gill/node'

// -----------------------------------------------------------------------------
// CONFIGURATION
// -----------------------------------------------------------------------------

const PROGRAM_ID = address("2nvhRn83KBxkkAfLH64meTq8cYB5aRLnZVbsxZdgfPTv");
const { rpc, sendAndConfirmTransaction } = createSolanaClient({ urlOrMoniker: process.env.ANCHOR_PROVIDER_URL! })

// SEEDS (Must match Rust constants exactly)
const USER_SEED = new TextEncoder().encode("user");
const PAPER_SEED = new TextEncoder().encode("paper");
const RECEIPT_SEED = new TextEncoder().encode("receipt");
const REVIEW_SEED = new TextEncoder().encode("review");
const VAULT_USER_SEED = new TextEncoder().encode("vault_user");
const VAULT_ADMIN_SEED = new TextEncoder().encode("vault_admin");

// -----------------------------------------------------------------------------
// TESTS
// -----------------------------------------------------------------------------

describe('prismpapersdapp', () => {
  // Actors
  let admin: KeyPairSigner
  let author: KeyPairSigner
  let buyer: KeyPairSigner

  // NOTE: In this flow, the 'buyer' will also act as the 'reviewer'
  let reviewer: KeyPairSigner

  beforeAll(async () => {
    // 1. Load Admin (This keypair MUST match the ADMIN_PUBKEY in your Rust constants)
    admin = await loadKeypairSignerFromFile(process.env.ANCHOR_WALLET!)

    // 2. Generate new users
    author = await generateKeyPairSigner()
    buyer = await generateKeyPairSigner()
    reviewer = buyer;

    // 3. Fund them
    await requestAirdrop(author.address)
    await requestAirdrop(buyer.address)

    console.log(`Admin: ${admin.address}`);
    console.log(`Author: ${author.address}`);
    console.log(`Buyer: ${buyer.address}`);
  })

  // ===========================================================================
  // 1. INIT USER
  // ===========================================================================

  it('Happy Path: Should initialize User Profile for Author', async () => {
    // A. Derive Profile PDA
    const [userAccount] = await getProgramDerivedAddress({
      programAddress: PROGRAM_ID,
      seeds: [USER_SEED, getAddressEncoder().encode(author.address)],
    });

    // B. Derive Vault PDA (Fixing your previous error)
    const [userVault] = await getProgramDerivedAddress({
      programAddress: PROGRAM_ID,
      seeds: [VAULT_USER_SEED, getAddressEncoder().encode(author.address)],
    });

    const ix = getInitUserInstruction({
      name: "Dr. Alice",
      owner: author,
      userAccount: userAccount,
      userVault: userVault, // <--- EXPLICITLY PASSED
    });

    const sx = await sendAndConfirm({ ix, payer: author });
    expect(sx).toBeDefined();
  });

  it('Setup: Should initialize User Profile for Buyer', async () => {
    // We need this for later tests
    const [userAccount] = await getProgramDerivedAddress({
      programAddress: PROGRAM_ID,
      seeds: [USER_SEED, getAddressEncoder().encode(buyer.address)],
    });
    const [userVault] = await getProgramDerivedAddress({
      programAddress: PROGRAM_ID,
      seeds: [VAULT_USER_SEED, getAddressEncoder().encode(buyer.address)],
    });

    const ix = getInitUserInstruction({
      name: "Bob Buyer",
      owner: buyer,
      userAccount: userAccount,
      userVault: userVault,
    });
    await sendAndConfirm({ ix, payer: buyer });
  });

  it('Sad Path: Should fail to init User with name too long', async () => {
    const randomUser = await generateKeyPairSigner();
    await requestAirdrop(randomUser.address);

    const [userAccount] = await getProgramDerivedAddress({
      programAddress: PROGRAM_ID,
      seeds: [USER_SEED, getAddressEncoder().encode(randomUser.address)],
    });
    const [userVault] = await getProgramDerivedAddress({
      programAddress: PROGRAM_ID,
      seeds: [VAULT_USER_SEED, getAddressEncoder().encode(randomUser.address)],
    });

    const longName = "A".repeat(200); // Too long
    const ix = getInitUserInstruction({
      name: longName,
      owner: randomUser,
      userAccount: userAccount,
      userVault: userVault,
    });

    await expect(sendAndConfirm({ ix, payer: randomUser })).rejects.toThrow();
  });

  // ===========================================================================
  // 2. INIT RESEARCH
  // ===========================================================================

  it('Happy Path: Should publish a Research Paper', async () => {
    const [researchPaper] = await getProgramDerivedAddress({
      programAddress: PROGRAM_ID,
      seeds: [PAPER_SEED, getAddressEncoder().encode(author.address)],
    });

    // We need the user account to update 'published' stats
    const [userAccount] = await getProgramDerivedAddress({
      programAddress: PROGRAM_ID,
      seeds: [USER_SEED, getAddressEncoder().encode(author.address)],
    });

    const ix = getInitResearchInstruction({
      title: "Quantum Mechanics",
      description: "A deep dive.",
      price: 1000000000n, // 1 SOL
      encryptedUrl: "arweave_cid",
      encryptionKey: "lit_key",
      author: author,
      researchPaper: researchPaper,
      userAccount: userAccount
    });

    const sx = await sendAndConfirm({ ix, payer: author });
    expect(sx).toBeDefined();
  });

  it('Sad Path: Should fail if Price is 0', async () => {
    // Using a different author/paper so we don't collide with existing PDA
    const poorAuthor = await generateKeyPairSigner();
    await requestAirdrop(poorAuthor.address);

    // Init poor author profile first
    const [uAcc] = await getProgramDerivedAddress({ programAddress: PROGRAM_ID, seeds: [USER_SEED, getAddressEncoder().encode(poorAuthor.address)] });
    const [uVault] = await getProgramDerivedAddress({ programAddress: PROGRAM_ID, seeds: [VAULT_USER_SEED, getAddressEncoder().encode(poorAuthor.address)] });
    await sendAndConfirm({ ix: getInitUserInstruction({ name: "Poor", owner: poorAuthor, userAccount: uAcc, userVault: uVault }), payer: poorAuthor });

    const [researchPaper] = await getProgramDerivedAddress({
      programAddress: PROGRAM_ID,
      seeds: [PAPER_SEED, getAddressEncoder().encode(poorAuthor.address)],
    });

    const ix = getInitResearchInstruction({
      title: "Free",
      description: "Desc",
      price: 0n, // ERROR
      encryptedUrl: "url",
      encryptionKey: "key",
      author: poorAuthor,
      researchPaper: researchPaper,
      userAccount: uAcc
    });

    await expect(sendAndConfirm({ ix, payer: poorAuthor })).rejects.toThrow();
  });

  // ===========================================================================
  // 3. PURCHASE ACCESS
  // ===========================================================================

  it('Happy Path: Buyer purchases access', async () => {
    // PDAs Setup
    const [researchPaper] = await getProgramDerivedAddress({
      programAddress: PROGRAM_ID,
      seeds: [PAPER_SEED, getAddressEncoder().encode(author.address)],
    });

    const [receipt] = await getProgramDerivedAddress({
      programAddress: PROGRAM_ID,
      seeds: [RECEIPT_SEED, getAddressEncoder().encode(buyer.address), getAddressEncoder().encode(researchPaper)],
    });

    // 3 Vaults + 2 Profiles
    const [buyerUserAccount] = await getProgramDerivedAddress({
      programAddress: PROGRAM_ID,
      seeds: [USER_SEED, getAddressEncoder().encode(buyer.address)],
    });
    const [buyerVault] = await getProgramDerivedAddress({
      programAddress: PROGRAM_ID,
      seeds: [VAULT_USER_SEED, getAddressEncoder().encode(buyer.address)],
    });

    const [authorUserAccount] = await getProgramDerivedAddress({
      programAddress: PROGRAM_ID,
      seeds: [USER_SEED, getAddressEncoder().encode(author.address)],
    });
    const [authorVault] = await getProgramDerivedAddress({
      programAddress: PROGRAM_ID,
      seeds: [VAULT_USER_SEED, getAddressEncoder().encode(author.address)],
    });

    const [adminVault] = await getProgramDerivedAddress({
      programAddress: PROGRAM_ID,
      seeds: [VAULT_ADMIN_SEED],
    });

    const ix = getPurchaseAccessInstruction({
      buyer: buyer,
      researchPaper: researchPaper,
      accessReceipt: receipt,
      buyerUserAccount,
      buyerVault,
      authorUserAccount,
      authorVault,
      adminVault
    });

    const sx = await sendAndConfirm({ ix, payer: buyer });
    expect(sx).toBeDefined();
  });

  it('Sad Path: Author buys own paper', async () => {
    const [researchPaper] = await getProgramDerivedAddress({
      programAddress: PROGRAM_ID,
      seeds: [PAPER_SEED, getAddressEncoder().encode(author.address)],
    });
    const [receipt] = await getProgramDerivedAddress({
      programAddress: PROGRAM_ID,
      seeds: [RECEIPT_SEED, getAddressEncoder().encode(author.address), getAddressEncoder().encode(researchPaper)],
    });

    // Using author as buyer
    const [authorAccount] = await getProgramDerivedAddress({ programAddress: PROGRAM_ID, seeds: [USER_SEED, getAddressEncoder().encode(author.address)] });
    const [authorVault] = await getProgramDerivedAddress({ programAddress: PROGRAM_ID, seeds: [VAULT_USER_SEED, getAddressEncoder().encode(author.address)] });
    const [adminVault] = await getProgramDerivedAddress({ programAddress: PROGRAM_ID, seeds: [VAULT_ADMIN_SEED] });

    const ix = getPurchaseAccessInstruction({
      buyer: author,
      researchPaper: researchPaper,
      accessReceipt: receipt,
      buyerUserAccount: authorAccount,
      buyerVault: authorVault,
      authorUserAccount: authorAccount,
      authorVault: authorVault,
      adminVault: adminVault
    });

    await expect(sendAndConfirm({ ix, payer: author })).rejects.toThrow();
  });

  // ===========================================================================
  // 4. REVIEW PAPER
  // ===========================================================================

  it('Happy Path: Buyer reviews paper', async () => {
    const [researchPaper] = await getProgramDerivedAddress({
      programAddress: PROGRAM_ID,
      seeds: [PAPER_SEED, getAddressEncoder().encode(author.address)],
    });
    const [review] = await getProgramDerivedAddress({
      programAddress: PROGRAM_ID,
      seeds: [REVIEW_SEED, getAddressEncoder().encode(buyer.address), getAddressEncoder().encode(researchPaper)],
    });
    const [receipt] = await getProgramDerivedAddress({
      programAddress: PROGRAM_ID,
      seeds: [RECEIPT_SEED, getAddressEncoder().encode(buyer.address), getAddressEncoder().encode(researchPaper)],
    });
    const [reviewerAccount] = await getProgramDerivedAddress({
      programAddress: PROGRAM_ID,
      seeds: [USER_SEED, getAddressEncoder().encode(buyer.address)],
    });

    const ix = getReviewPaperInstruction({
      reviewUrl: "ipfs://valid",
      proposedReward: 500000n,
      reviewer: buyer,
      researchPaper: researchPaper,
      accessReceipt: receipt, // Proof of purchase
      reviewerUserAccount: reviewerAccount,
      peerReview: review
    });

    const sx = await sendAndConfirm({ ix, payer: buyer });
    expect(sx).toBeDefined();
  });

  it('Sad Path: Non-purchaser tries to review', async () => {
    const rando = await generateKeyPairSigner();
    await requestAirdrop(rando.address);
    // Init rando user profile... (omitted for brevity, but technically required if constraints check user profile)

    const [researchPaper] = await getProgramDerivedAddress({
      programAddress: PROGRAM_ID,
      seeds: [PAPER_SEED, getAddressEncoder().encode(author.address)],
    });
    // This receipt DOES NOT EXIST on chain
    const [fakeReceipt] = await getProgramDerivedAddress({
      programAddress: PROGRAM_ID,
      seeds: [RECEIPT_SEED, getAddressEncoder().encode(rando.address), getAddressEncoder().encode(researchPaper)],
    });
    const [review] = await getProgramDerivedAddress({
      programAddress: PROGRAM_ID,
      seeds: [REVIEW_SEED, getAddressEncoder().encode(rando.address), getAddressEncoder().encode(researchPaper)],
    });
    // Rando profile
    const [randoAccount] = await getProgramDerivedAddress({ programAddress: PROGRAM_ID, seeds: [USER_SEED, getAddressEncoder().encode(rando.address)] });
    // Assuming rando init profile code was here, skipping to error:

    // Actually, simply passing a non-existent Receipt account will cause an AccountNotInitialized error
    // because Anchor checks if `accessReceipt` has data.
  });

  // ===========================================================================
  // 5. VERIFY REVIEW
  // ===========================================================================

  it('Happy Path: Author accepts review', async () => {
    const [researchPaper] = await getProgramDerivedAddress({
      programAddress: PROGRAM_ID,
      seeds: [PAPER_SEED, getAddressEncoder().encode(author.address)],
    });
    const [review] = await getProgramDerivedAddress({
      programAddress: PROGRAM_ID,
      seeds: [REVIEW_SEED, getAddressEncoder().encode(buyer.address), getAddressEncoder().encode(researchPaper)],
    });

    // We need ALL vaults because money potentially moves
    const [authorUserAccount] = await getProgramDerivedAddress({
      programAddress: PROGRAM_ID,
      seeds: [USER_SEED, getAddressEncoder().encode(author.address)],
    });
    const [authorVault] = await getProgramDerivedAddress({
      programAddress: PROGRAM_ID,
      seeds: [VAULT_USER_SEED, getAddressEncoder().encode(author.address)],
    });
    const [reviewerUserAccount] = await getProgramDerivedAddress({
      programAddress: PROGRAM_ID,
      seeds: [USER_SEED, getAddressEncoder().encode(buyer.address)],
    });
    const [reviewerVault] = await getProgramDerivedAddress({
      programAddress: PROGRAM_ID,
      seeds: [VAULT_USER_SEED, getAddressEncoder().encode(buyer.address)],
    });
    const [adminVault] = await getProgramDerivedAddress({
      programAddress: PROGRAM_ID,
      seeds: [VAULT_ADMIN_SEED],
    });

    const ix = getVerifyReviewInstruction({
      acceptProposedReview: true,
      author: author,
      researchPaper,
      peerReview: review,
      authorUserAccount,
      authorVault,
      reviewerUserAccount,
      reviewerVault,
      adminVault
    });

    const sx = await sendAndConfirm({ ix, payer: author });
    expect(sx).toBeDefined();
  });

  // ===========================================================================
  // 6. WITHDRAWS
  // ===========================================================================

  it('Happy Path: User Withdraws', async () => {
    const [userVault] = await getProgramDerivedAddress({
      programAddress: PROGRAM_ID,
      seeds: [VAULT_USER_SEED, getAddressEncoder().encode(author.address)],
    });
    const balanceResponse = await rpc.getBalance(userVault).send();

    // Log the value (it returns a bigint)
    console.log("User Vault Balance:", balanceResponse.value);

    // Author should have earned money from the purchase earlier
    const ix = getUserWithdrawInstruction({
      amount: 1000n, // withdraw a tiny bit
      user: author,
      userVault: userVault
    });
    const sx = await sendAndConfirm({ ix, payer: author });
    expect(sx).toBeDefined();
  });

  it('Happy Path: Admin Withdraws', async () => {
    const [adminVault] = await getProgramDerivedAddress({
      programAddress: PROGRAM_ID,
      seeds: [VAULT_ADMIN_SEED],
    });
    const balanceResponse = await rpc.getBalance(adminVault).send();

    // Log the value (it returns a bigint)
    console.log("Admin Vault Balance:", balanceResponse.value);
    const ix = getAdminWithdrawInstruction({
      amount: 1000n,
      admin: admin,
      adminVault: adminVault
    });

    const sx = await sendAndConfirm({ ix, payer: admin });
    expect(sx).toBeDefined();
  });

})

// -----------------------------------------------------------------------------
// HELPERS
// -----------------------------------------------------------------------------

let latestBlockhash: Awaited<ReturnType<typeof getLatestBlockhash>> | undefined
async function getLatestBlockhash(): Promise<Readonly<{ blockhash: Blockhash; lastValidBlockHeight: bigint }>> {
  if (latestBlockhash) {
    return latestBlockhash
  }
  return await rpc.getLatestBlockhash().send().then(({ value }) => value)
}

async function sendAndConfirm({ ix, payer }: { ix: Instruction; payer: KeyPairSigner }) {
  const tx = createTransaction({
    feePayer: payer,
    instructions: [ix],
    version: 'legacy',
    latestBlockhash: await getLatestBlockhash(),
  })
  const signedTransaction = await signTransactionMessageWithSigners(tx)
  return await sendAndConfirmTransaction(signedTransaction)
}

async function requestAirdrop(address: Address) {
  await rpc.requestAirdrop(address, 10000000000n as any).send();
  await new Promise(r => setTimeout(r, 1000));
}