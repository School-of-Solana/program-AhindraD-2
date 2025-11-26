use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCodes {
    #[msg("User name cannot be empty or too long")]
    UserNameInvalid,

    #[msg("Research Paper Title cannot be empty or too long")]
    PaperTitleInvalid,

    #[msg("Research Paper Description cannot be empty or too long")]
    PaperDescriptionInvalid,

    #[msg("Research Paper URL/CID cannot be empty")]
    PaperUrlEmptyOrTooLong,

    #[msg("Protocol Encryption Key cannot be empty")]
    EncryptionKeyEmptyOrTooLong,

    #[msg("Review Link/CID cannot be empty")]
    ReviewUrlEmpty,

    #[msg("Price must be greater than zero")]
    ResearchPriceInvalid,

    #[msg("Mathematical Operation Overflow")]
    MathOverflow,

    #[msg("The vault does not have enough SOL to fulfill this request")]
    InsufficientFundsInVault,

    #[msg("User does not have enough accrued earnings for this withdrawal")]
    InsufficientUserEarnings,

    #[msg("The buyer/reviewer/author does not have enough SOL in their wallet")]
    InsufficientFundsInWallet,

    #[msg("You are not authorized to perform this action (Admin Only)")]
    UnauthorizedAdmin,

    #[msg("Only the original author can update this paper")]
    UnauthorizedUpdate,

    #[msg("You cannot buy your own research paper")]
    AuthorCantBuySelf,

    #[msg("You cannot review your own research paper")]
    AuthorCantReviewSelf,

    #[msg("You have already purchased this paper")]
    AlreadyPurchased,

    #[msg("You have already submitted a review for this paper")]
    AlreadyReviewed,

    #[msg("This review has already been processed (Accepted/Rejected)")]
    ReviewNotPending,

    #[msg("You must purchase the paper before reviewing it")]
    PaperNotPurchased,
}
