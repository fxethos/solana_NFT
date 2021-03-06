use {
    crate::ErrorCode,
    anchor_lang::{
        prelude::{AccountInfo, ProgramError, ProgramResult, Pubkey},
        solana_program::{
            program::invoke_signed,
            program_pack::{IsInitialized, Pack},
        },
    },
};

pub fn assert_initialized<T: Pack + IsInitialized>(
    account_info: &AccountInfo,
) -> Result<T, ProgramError> {
    let account: T = T::unpack_unchecked(&account_info.data.borrow())?;
    if !account.is_initialized() {
        Err(ErrorCode::Uninitialized.into())
    } else {
        Ok(account)
    }
}

pub fn assert_owned_by(account: &AccountInfo, owner: &Pubkey) -> ProgramResult {
    if account.owner != owner {
        Err(ErrorCode::IncorrectOwner.into())
    } else {
        Ok(())
    }
}
pub struct TokenTransferParams<'a: 'b, 'b> {
    pub source: AccountInfo<'a>,
    pub destination: AccountInfo<'a>,
    pub amount: u64,
    pub authority: AccountInfo<'a>,
    pub authority_signer_seeds: &'b [&'b [u8]],
    pub token_program: AccountInfo<'a>,
}

#[inline(always)]
pub fn spl_token_transfer(params: TokenTransferParams<'_, '_>) -> ProgramResult {
    let TokenTransferParams {
        source,
        destination,
        authority,
        token_program,
        amount,
        authority_signer_seeds,
    } = params;

    let result = invoke_signed(
        &spl_token::instruction::transfer(
            token_program.key,
            source.key,
            destination.key,
            authority.key,
            &[],
            amount,
        )?,
        &[source, destination, authority, token_program],
        &[authority_signer_seeds],
    );

    result.map_err(|_| ErrorCode::TokenTransferFailed.into())
}
