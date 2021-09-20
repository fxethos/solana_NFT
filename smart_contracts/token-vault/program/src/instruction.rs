use {
    crate::state::{ExternalPriceAccount, Key},
    borsh::{BorshDeserialize, BorshSerialize},
    solana_program::{
        instruction::{AccountMeta, Instruction},
        pubkey::Pubkey,
        sysvar,
    },
};

#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, Clone)]
pub struct InitVaultArgs {
    pub allow_further_share_creation: bool,
}

#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, Clone)]
pub struct AmountArgs {
    pub amount: u64,
}

#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, Clone)]
pub struct NumberOfShareArgs {
    pub number_of_shares: u64,
}

#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone)]
pub struct MintEditionProxyArgs {
    pub edition: u64,
}

#[derive(BorshSerialize, BorshDeserialize, Clone)]
pub enum VaultInstruction {
   
    InitVault(InitVaultArgs),

   
    AddTokenToInactiveVault(AmountArgs),

   
    ActivateVault(NumberOfShareArgs),

    CombineVault,

    RedeemShares,

  
    WithdrawTokenFromSafetyDepositBox(AmountArgs),

 
    MintFractionalShares(NumberOfShareArgs),

  
    WithdrawSharesFromTreasury(NumberOfShareArgs),

    
    AddSharesToTreasury(NumberOfShareArgs),

   
    UpdateExternalPriceAccount(ExternalPriceAccount),

    SetAuthority,
}

#[allow(clippy::too_many_arguments)]
pub fn create_init_vault_instruction(
    program_id: Pubkey,
    fraction_mint: Pubkey,
    redeem_treasury: Pubkey,
    fraction_treasury: Pubkey,
    vault: Pubkey,
    vault_authority: Pubkey,
    external_price_account: Pubkey,
    allow_further_share_creation: bool,
) -> Instruction {
    Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new(fraction_mint, false),
            AccountMeta::new(redeem_treasury, false),
            AccountMeta::new(fraction_treasury, false),
            AccountMeta::new(vault, false),
            AccountMeta::new_readonly(vault_authority, false),
            AccountMeta::new_readonly(external_price_account, false),
            AccountMeta::new_readonly(spl_token::id(), false),
            AccountMeta::new_readonly(sysvar::rent::id(), false),
        ],
        data: VaultInstruction::InitVault(InitVaultArgs {
            allow_further_share_creation,
        })
        .try_to_vec()
        .unwrap(),
    }
}

#[allow(clippy::too_many_arguments)]
pub fn create_update_external_price_account_instruction(
    program_id: Pubkey,
    external_price_account: Pubkey,
    price_per_share: u64,
    price_mint: Pubkey,
    allowed_to_combine: bool,
) -> Instruction {
    Instruction {
        program_id,
        accounts: vec![AccountMeta::new(external_price_account, true)],
        data: VaultInstruction::UpdateExternalPriceAccount(ExternalPriceAccount {
            key: Key::ExternalAccountKeyV1,
            price_per_share,
            price_mint,
            allowed_to_combine,
        })
        .try_to_vec()
        .unwrap(),
    }
}

#[allow(clippy::too_many_arguments)]
pub fn create_add_token_to_inactive_vault_instruction(
    program_id: Pubkey,
    safety_deposit_box: Pubkey,
    token_account: Pubkey,
    store: Pubkey,
    vault: Pubkey,
    vault_authority: Pubkey,
    payer: Pubkey,
    transfer_authority: Pubkey,
    amount: u64,
) -> Instruction {
    Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new(safety_deposit_box, false),
            AccountMeta::new(token_account, false),
            AccountMeta::new(store, false),
            AccountMeta::new(vault, false),
            AccountMeta::new_readonly(vault_authority, true),
            AccountMeta::new_readonly(payer, true),
            AccountMeta::new_readonly(transfer_authority, true),
            AccountMeta::new_readonly(spl_token::id(), false),
            AccountMeta::new_readonly(sysvar::rent::id(), false),
            AccountMeta::new_readonly(solana_program::system_program::id(), false),
        ],
        data: VaultInstruction::AddTokenToInactiveVault(AmountArgs { amount })
            .try_to_vec()
            .unwrap(),
    }
}

#[allow(clippy::too_many_arguments)]
pub fn create_activate_vault_instruction(
    program_id: Pubkey,
    vault: Pubkey,
    fraction_mint: Pubkey,
    fraction_treasury: Pubkey,
    fraction_mint_authority: Pubkey,
    vault_authority: Pubkey,
    number_of_shares: u64,
) -> Instruction {
    Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new(vault, false),
            AccountMeta::new(fraction_mint, false),
            AccountMeta::new(fraction_treasury, false),
            AccountMeta::new_readonly(fraction_mint_authority, false),
            AccountMeta::new_readonly(vault_authority, true),
            AccountMeta::new_readonly(spl_token::id(), false),
        ],
        data: VaultInstruction::ActivateVault(NumberOfShareArgs { number_of_shares })
            .try_to_vec()
            .unwrap(),
    }
}

#[allow(clippy::too_many_arguments)]
pub fn create_combine_vault_instruction(
    program_id: Pubkey,
    vault: Pubkey,
    outstanding_share_token_account: Pubkey,
    paying_token_account: Pubkey,
    fraction_mint: Pubkey,
    fraction_treasury: Pubkey,
    redeem_treasury: Pubkey,
    new_authority: Pubkey,
    vault_authority: Pubkey,
    paying_transfer_authority: Pubkey,
    uncirculated_burn_authority: Pubkey,
    external_pricing_account: Pubkey,
) -> Instruction {
    Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new(vault, false),
            AccountMeta::new(outstanding_share_token_account, false),
            AccountMeta::new(paying_token_account, false),
            AccountMeta::new(fraction_mint, false),
            AccountMeta::new(fraction_treasury, false),
            AccountMeta::new(redeem_treasury, false),
            AccountMeta::new(new_authority, false),
            AccountMeta::new_readonly(vault_authority, true),
            AccountMeta::new_readonly(paying_transfer_authority, true),
            AccountMeta::new_readonly(uncirculated_burn_authority, false),
            AccountMeta::new_readonly(external_pricing_account, false),
            AccountMeta::new_readonly(spl_token::id(), false),
        ],
        data: VaultInstruction::CombineVault.try_to_vec().unwrap(),
    }
}

#[allow(clippy::too_many_arguments)]
pub fn create_redeem_shares_instruction(
    program_id: Pubkey,
    outstanding_shares_account: Pubkey,
    proceeds_account: Pubkey,
    fraction_mint: Pubkey,
    redeem_treasury: Pubkey,
    transfer_authority: Pubkey,
    burn_authority: Pubkey,
    vault: Pubkey,
) -> Instruction {
    Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new(outstanding_shares_account, false),
            AccountMeta::new(proceeds_account, false),
            AccountMeta::new(fraction_mint, false),
            AccountMeta::new(redeem_treasury, false),
            AccountMeta::new_readonly(transfer_authority, false),
            AccountMeta::new_readonly(burn_authority, true),
            AccountMeta::new_readonly(vault, false),
            AccountMeta::new_readonly(spl_token::id(), false),
            AccountMeta::new_readonly(sysvar::rent::id(), false),
        ],
        data: VaultInstruction::RedeemShares.try_to_vec().unwrap(),
    }
}

#[allow(clippy::too_many_arguments)]
pub fn create_withdraw_tokens_instruction(
    program_id: Pubkey,
    destination: Pubkey,
    safety_deposit_box: Pubkey,
    store: Pubkey,
    vault: Pubkey,
    fraction_mint: Pubkey,
    vault_authority: Pubkey,
    transfer_authority: Pubkey,
    amount: u64,
) -> Instruction {
    Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new(destination, false),
            AccountMeta::new(safety_deposit_box, false),
            AccountMeta::new(store, false),
            AccountMeta::new(vault, false),
            AccountMeta::new_readonly(fraction_mint, false),
            AccountMeta::new_readonly(vault_authority, true),
            AccountMeta::new_readonly(transfer_authority, false),
            AccountMeta::new_readonly(spl_token::id(), false),
            AccountMeta::new_readonly(sysvar::rent::id(), false),
        ],
        data: VaultInstruction::WithdrawTokenFromSafetyDepositBox(AmountArgs { amount })
            .try_to_vec()
            .unwrap(),
    }
}

#[allow(clippy::too_many_arguments)]
pub fn create_mint_shares_instruction(
    program_id: Pubkey,
    fraction_treasury: Pubkey,
    fraction_mint: Pubkey,
    vault: Pubkey,
    fraction_mint_authority: Pubkey,
    vault_authority: Pubkey,
    number_of_shares: u64,
) -> Instruction {
    Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new(fraction_treasury, false),
            AccountMeta::new(fraction_mint, false),
            AccountMeta::new_readonly(vault, false),
            AccountMeta::new_readonly(fraction_mint_authority, false),
            AccountMeta::new_readonly(vault_authority, true),
            AccountMeta::new_readonly(spl_token::id(), false),
        ],
        data: VaultInstruction::MintFractionalShares(NumberOfShareArgs { number_of_shares })
            .try_to_vec()
            .unwrap(),
    }
}

#[allow(clippy::too_many_arguments)]
pub fn create_withdraw_shares_instruction(
    program_id: Pubkey,
    destination: Pubkey,
    fraction_treasury: Pubkey,
    vault: Pubkey,
    transfer_authority: Pubkey,
    vault_authority: Pubkey,
    number_of_shares: u64,
) -> Instruction {
    Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new(destination, false),
            AccountMeta::new(fraction_treasury, false),
            AccountMeta::new_readonly(vault, false),
            AccountMeta::new_readonly(transfer_authority, false),
            AccountMeta::new_readonly(vault_authority, true),
            AccountMeta::new_readonly(spl_token::id(), false),
            AccountMeta::new_readonly(sysvar::rent::id(), false),
        ],
        data: VaultInstruction::WithdrawSharesFromTreasury(NumberOfShareArgs { number_of_shares })
            .try_to_vec()
            .unwrap(),
    }
}

#[allow(clippy::too_many_arguments)]
pub fn create_add_shares_instruction(
    program_id: Pubkey,
    source: Pubkey,
    fraction_treasury: Pubkey,
    vault: Pubkey,
    transfer_authority: Pubkey,
    vault_authority: Pubkey,
    number_of_shares: u64,
) -> Instruction {
    Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new(source, false),
            AccountMeta::new(fraction_treasury, false),
            AccountMeta::new_readonly(vault, false),
            AccountMeta::new_readonly(transfer_authority, true),
            AccountMeta::new_readonly(vault_authority, true),
            AccountMeta::new_readonly(spl_token::id(), false),
            AccountMeta::new_readonly(sysvar::rent::id(), false),
        ],
        data: VaultInstruction::AddSharesToTreasury(NumberOfShareArgs { number_of_shares })
            .try_to_vec()
            .unwrap(),
    }
}

pub fn create_set_authority_instruction(
    program_id: Pubkey,
    vault: Pubkey,
    current_authority: Pubkey,
    new_authority: Pubkey,
) -> Instruction {
    Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new(vault, false),
            AccountMeta::new_readonly(current_authority, true),
            AccountMeta::new_readonly(new_authority, false),
        ],
        data: VaultInstruction::SetAuthority.try_to_vec().unwrap(),
    }
}
