use {
    crate::utils::try_from_slice_checked,
    borsh::{BorshDeserialize, BorshSerialize},
    solana_program::{account_info::AccountInfo, program_error::ProgramError, pubkey::Pubkey},
};
pub const PREFIX: &str = "vault";

#[repr(C)]
#[derive(Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub enum Key {
    Uninitialized,
    SafetyDepositBoxV1,
    ExternalAccountKeyV1,
    VaultV1,
}

pub const MAX_SAFETY_DEPOSIT_SIZE: usize = 1 + 32 + 32 + 32 + 1;
pub const MAX_VAULT_SIZE: usize = 1 + 32 + 32 + 32 + 32 + 1 + 32 + 1 + 32 + 1 + 1 + 8;
pub const MAX_EXTERNAL_ACCOUNT_SIZE: usize = 1 + 8 + 32 + 1;
#[repr(C)]
#[derive(Clone, BorshSerialize, BorshDeserialize, PartialEq)]
pub enum VaultState {
    Inactive,
    Active,
    Combined,
    Deactivated,
}

#[repr(C)]
#[derive(Clone, BorshSerialize, BorshDeserialize)]
pub struct Vault {
    pub key: Key,
    pub token_program: Pubkey,
    pub fraction_mint: Pubkey,
    pub authority: Pubkey,
    pub fraction_treasury: Pubkey,
    pub redeem_treasury: Pubkey,
    pub allow_further_share_creation: bool,

    pub pricing_lookup_address: Pubkey,
    
    pub token_type_count: u8,
    pub state: VaultState,

  
    pub locked_price_per_share: u64,
}

impl Vault {
    pub fn from_account_info(a: &AccountInfo) -> Result<Vault, ProgramError> {
        let vt: Vault = try_from_slice_checked(&a.data.borrow_mut(), Key::VaultV1, MAX_VAULT_SIZE)?;

        Ok(vt)
    }

    pub fn get_token_type_count(a: &AccountInfo) -> u8 {
        return a.data.borrow()[194];
    }
}

#[repr(C)]
#[derive(Clone, BorshSerialize, BorshDeserialize)]
pub struct SafetyDepositBox {
   
    pub key: Key,
    pub vault: Pubkey,
    pub token_mint: Pubkey,
    pub store: Pubkey,
    pub order: u8,
}

impl SafetyDepositBox {
    pub fn from_account_info(a: &AccountInfo) -> Result<SafetyDepositBox, ProgramError> {
        let sd: SafetyDepositBox = try_from_slice_checked(
            &a.data.borrow_mut(),
            Key::SafetyDepositBoxV1,
            MAX_SAFETY_DEPOSIT_SIZE,
        )?;

        Ok(sd)
    }

    pub fn get_order(a: &AccountInfo) -> u8 {
        a.data.borrow()[97]
    }
}

#[repr(C)]
#[derive(Clone, BorshSerialize, BorshDeserialize)]
pub struct ExternalPriceAccount {
    pub key: Key,
    pub price_per_share: u64,
  
    pub price_mint: Pubkey,
    pub allowed_to_combine: bool,
}

impl ExternalPriceAccount {
    pub fn from_account_info(a: &AccountInfo) -> Result<ExternalPriceAccount, ProgramError> {
        let sd: ExternalPriceAccount = try_from_slice_checked(
            &a.data.borrow_mut(),
            Key::ExternalAccountKeyV1,
            MAX_EXTERNAL_ACCOUNT_SIZE,
        )?;

        Ok(sd)
    }
}
