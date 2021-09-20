//! Error types

use {
    num_derive::FromPrimitive,
    solana_program::{
        decode_error::DecodeError,
        msg,
        program_error::{PrintProgramError, ProgramError},
    },
    thiserror::Error,
};

#[derive(Clone, Debug, Eq, Error, FromPrimitive, PartialEq)]
pub enum VaultError {
    #[error("Failed to unpack instruction data")]
    InstructionUnpackError,

    #[error("Lamport balance below rent-exempt threshold")]
    NotRentExempt,

    #[error("Already initialized")]
    AlreadyInitialized,

    #[error("Uninitialized")]
    Uninitialized,

    #[error("Account does not have correct owner")]
    IncorrectOwner,

    #[error("NumericalOverflowError")]
    NumericalOverflowError,

    #[error("Provided token account contains no tokens")]
    TokenAccountContainsNoTokens,

    #[error("Provided token account cannot provide amount specified")]
    TokenAccountAmountLessThanAmountSpecified,

    #[error("Provided vault account contains is not empty")]
    VaultAccountIsNotEmpty,

    #[error("Provided vault account is not owned by program derived address with seed of prefix and program id")]
    VaultAccountIsNotOwnedByProgram,

    #[error(
        "The provided safety deposit account address does not match the expected program derived address"
    )]
    SafetyDepositAddressInvalid,

    #[error("Token transfer failed")]
    TokenTransferFailed,
    #[error("Token mint to failed")]
    TokenMintToFailed,
    #[error("Token burn failed")]
    TokenBurnFailed,

    #[error("Vault mint not empty on init")]
    VaultMintNotEmpty,

    #[error("Vault mint's authority not set to program PDA with seed of prefix and program id")]
    VaultAuthorityNotProgram,

    #[error("Vault treasury not empty on init")]
    TreasuryNotEmpty,

    #[error("Vault treasury's owner not set to program pda with seed of prefix and program id")]
    TreasuryOwnerNotProgram,

    #[error("Vault should be inactive")]
    VaultShouldBeInactive,

    #[error("Vault should be active")]
    VaultShouldBeActive,

    #[error("Vault should be combined")]
    VaultShouldBeCombined,

    #[error("Vault treasury needs to match fraction mint")]
    VaultTreasuryMintDoesNotMatchVaultMint,

    #[error("Redeem Treasury cannot be same mint as fraction")]
    RedeemTreasuryCantShareSameMintAsFraction,

    #[error("Invalid program authority provided")]
    InvalidAuthority,

    #[error("Redeem treasury mint must match lookup mint")]
    RedeemTreasuryMintMustMatchLookupMint,

    #[error("You must pay with the same mint as the external pricing oracle")]
    PaymentMintShouldMatchPricingMint,

    #[error("Your share account should match the mint of the fractional mint")]
    ShareMintShouldMatchFractionalMint,

    #[error("Vault mint provided does not match that on the token vault")]
    VaultMintNeedsToMatchVault,

    #[error("Redeem treasury provided does not match that on the token vault")]
    RedeemTreasuryNeedsToMatchVault,

    #[error("Fraction treasury provided does not match that on the token vault")]
    FractionTreasuryNeedsToMatchVault,

    #[error("Not allowed to combine at this time")]
    NotAllowedToCombine,

    #[error("You cannot afford to combine this vault")]
    CannotAffordToCombineThisVault,

    #[error("You have no shares to redeem")]
    NoShares,

    #[error("Your outstanding share account is the incorrect mint")]
    OutstandingShareAccountNeedsToMatchFractionalMint,

    #[error("Your destination account is the incorrect mint")]
    DestinationAccountNeedsToMatchRedeemMint,

    #[error("Fractional mint is empty")]
    FractionSupplyEmpty,

    #[error("Token Program Provided Needs To Match Vault")]
    TokenProgramProvidedDoesNotMatchVault,

    #[error("Authority of vault needs to be signer for this action")]
    AuthorityIsNotSigner,

    #[error("Authority of vault does not match authority provided")]
    AuthorityDoesNotMatch,

    #[error("This safety deposit box does not belong to this vault!")]
    SafetyDepositBoxVaultMismatch,

    #[error("The store provided does not match the store key on the safety deposit box!")]
    StoreDoesNotMatchSafetyDepositBox,

    #[error("This safety deposit box is empty!")]
    StoreEmpty,

    #[error("The destination account to receive your token needs to be the same mint as the token's mint")]
    DestinationAccountNeedsToMatchTokenMint,

    #[error("The destination account to receive your shares needs to be the same mint as the vault's fraction mint")]
    DestinationAccountNeedsToMatchFractionMint,

    #[error("The source account to send your shares from needs to be the same mint as the vault's fraction mint")]
    SourceAccountNeedsToMatchFractionMint,

    #[error("This vault does not allow the minting of new shares!")]
    VaultDoesNotAllowNewShareMinting,

    #[error("There are not enough shares")]
    NotEnoughShares,

    #[error("External price account must be signer")]
    ExternalPriceAccountMustBeSigner,

    #[error("Very bad, someone changed external account's price mint after vault creation!")]
    RedeemTreasuryMintShouldMatchPricingMint,

    #[error("Store has less than amount desired")]
    StoreLessThanAmount,

    #[error("Invalid token program")]
    InvalidTokenProgram,

    #[error("Data type mismatch")]
    DataTypeMismatch,

    #[error("Accept payment delegate should be none")]
    DelegateShouldBeNone,

    #[error("Accept payment close authority should be none")]
    CloseAuthorityShouldBeNone,

    #[error("Derived key invalid")]
    DerivedKeyInvalid,
}

impl PrintProgramError for VaultError {
    fn print<E>(&self) {
        msg!(&self.to_string());
    }
}

impl From<VaultError> for ProgramError {
    fn from(e: VaultError) -> Self {
        ProgramError::Custom(e as u32)
    }
}

impl<T> DecodeError<T> for VaultError {
    fn type_of() -> &'static str {
        "Vault Error"
    }
}
