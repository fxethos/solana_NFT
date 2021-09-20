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
pub enum MetadataError {
    #[error("Failed to unpack instruction data")]
    InstructionUnpackError,

    #[error("Failed to pack instruction data")]
    InstructionPackError,

    #[error("Lamport balance below rent-exempt threshold")]
    NotRentExempt,

    #[error("Already initialized")]
    AlreadyInitialized,

    #[error("Uninitialized")]
    Uninitialized,

    #[error(" Metadata's key must match seed of ['metadata', program id, mint] provided")]
    InvalidMetadataKey,

    #[error("Edition's key must match seed of ['metadata', program id, name, 'edition'] provided")]
    InvalidEditionKey,

    #[error("Update Authority given does not match")]
    UpdateAuthorityIncorrect,

    #[error("Update Authority needs to be signer to update metadata")]
    UpdateAuthorityIsNotSigner,

    #[error("You must be the mint authority and signer on this transaction")]
    NotMintAuthority,

    #[error("Mint authority provided does not match the authority on the mint")]
    InvalidMintAuthority,

    #[error("Name too long")]
    NameTooLong,

    #[error("Symbol too long")]
    SymbolTooLong,

    #[error("URI too long")]
    UriTooLong,

    #[error("Update authority must be equivalent to the metadata's authority and also signer of this transaction")]
    UpdateAuthorityMustBeEqualToMetadataAuthorityAndSigner,

    #[error("Mint given does not match mint on Metadata")]
    MintMismatch,

    #[error("Editions must have exactly one token")]
    EditionsMustHaveExactlyOneToken,

    #[error("Maximum editions printed already")]
    MaxEditionsMintedAlready,

    #[error("Token mint to failed")]
    TokenMintToFailed,

    #[error("The master edition record passed must match the master record on the edition given")]
    MasterRecordMismatch,

    #[error("The destination account does not have the right mint")]
    DestinationMintMismatch,

    #[error("An edition can only mint one of its kind!")]
    EditionAlreadyMinted,

    #[error("Printing mint decimals should be zero")]
    PrintingMintDecimalsShouldBeZero,

    #[error("OneTimePrintingAuthorization mint decimals should be zero")]
    OneTimePrintingAuthorizationMintDecimalsShouldBeZero,

    #[error("EditionMintDecimalsShouldBeZero")]
    EditionMintDecimalsShouldBeZero,

    #[error("Token burn failed")]
    TokenBurnFailed,

    #[error("The One Time authorization mint does not match that on the token account!")]
    TokenAccountOneTimeAuthMintMismatch,

    #[error("Derived key invalid")]
    DerivedKeyInvalid,

    #[error("The Printing mint does not match that on the master edition!")]
    PrintingMintMismatch,

    #[error("The One Time Printing Auth mint does not match that on the master edition!")]
    OneTimePrintingAuthMintMismatch,

    #[error("The mint of the token account does not match the Printing mint!")]
    TokenAccountMintMismatch,

    #[error("The mint of the token account does not match the master metadata mint!")]
    TokenAccountMintMismatchV2,

    #[error("Not enough tokens to mint a limited edition")]
    NotEnoughTokens,

    #[error(
        "The mint on your authorization token holding account does not match your Printing mint!"
    )]
    PrintingMintAuthorizationAccountMismatch,

    #[error("The authorization token account has a different owner than the update authority for the master edition!")]
    AuthorizationTokenAccountOwnerMismatch,

    #[error("This feature is currently disabled.")]
    Disabled,

    #[error("Creators list too long")]
    CreatorsTooLong,

    #[error("Creators must be at least one if set")]
    CreatorsMustBeAtleastOne,

    #[error("If using a creators array, you must be one of the creators listed")]
    MustBeOneOfCreators,

    #[error("This metadata does not have creators")]
    NoCreatorsPresentOnMetadata,

    #[error("This creator address was not found")]
    CreatorNotFound,

    #[error("Basis points cannot be more than 10000")]
    InvalidBasisPoints,

    #[error("Primary sale can only be flipped to true and is immutable")]
    PrimarySaleCanOnlyBeFlippedToTrue,

    #[error("Owner does not match that on the account given")]
    OwnerMismatch,

    #[error("This account has no tokens to be used for authorization")]
    NoBalanceInAccountForAuthorization,

    #[error("Share total must equal 100 for creator array")]
    ShareTotalMustBe100,

    #[error("This reservation list already exists!")]
    ReservationExists,

    #[error("This reservation list does not exist!")]
    ReservationDoesNotExist,

    #[error("This reservation list exists but was never set with reservations")]
    ReservationNotSet,

    #[error("This reservation list has already been set!")]
    ReservationAlreadyMade,

    #[error("Provided more addresses than max allowed in single reservation")]
    BeyondMaxAddressSize,

    #[error("NumericalOverflowError")]
    NumericalOverflowError,

    #[error("This reservation would go beyond the maximum supply of the master edition!")]
    ReservationBreachesMaximumSupply,

    #[error("Address not in reservation!")]
    AddressNotInReservation,

    #[error("You cannot unilaterally verify another creator, they must sign")]
    CannotVerifyAnotherCreator,

    #[error("You cannot unilaterally unverify another creator")]
    CannotUnverifyAnotherCreator,

    #[error("In initial reservation setting, spots remaining should equal total spots")]
    SpotMismatch,

    #[error("Incorrect account owner")]
    IncorrectOwner,

    #[error("printing these tokens would breach the maximum supply limit of the master edition")]
    PrintingWouldBreachMaximumSupply,

    #[error("Data is immutable")]
    DataIsImmutable,

    #[error("No duplicate creator addresses")]
    DuplicateCreatorAddress,

    #[error("Reservation spots remaining should match total spots when first being created")]
    ReservationSpotsRemainingShouldMatchTotalSpotsAtStart,

    #[error("Invalid token program")]
    InvalidTokenProgram,

    #[error("Data type mismatch")]
    DataTypeMismatch,

    #[error("Beyond alotted address size in reservation!")]
    BeyondAlottedAddressSize,

    #[error("The reservation has only been partially alotted")]
    ReservationNotComplete,

    #[error("You cannot splice over an existing reservation!")]
    TriedToReplaceAnExistingReservation,

    #[error("Invalid operation")]
    InvalidOperation,

    #[error("Invalid Owner")]
    InvalidOwner,

    #[error("Printing mint supply must be zero for conversion")]
    PrintingMintSupplyMustBeZeroForConversion,

    #[error("One Time Auth mint supply must be zero for conversion")]
    OneTimeAuthMintSupplyMustBeZeroForConversion,

    #[error("You tried to insert one edition too many into an edition mark pda")]
    InvalidEditionIndex,

    #[error("In the legacy system the reservation needs to be of size one for cpu limit reasons")]
    ReservationArrayShouldBeSizeOne,
}

impl PrintProgramError for MetadataError {
    fn print<E>(&self) {
        msg!(&self.to_string());
    }
}

impl From<MetadataError> for ProgramError {
    fn from(e: MetadataError) -> Self {
        ProgramError::Custom(e as u32)
    }
}

impl<T> DecodeError<T> for MetadataError {
    fn type_of() -> &'static str {
        "Metadata Error"
    }
}
