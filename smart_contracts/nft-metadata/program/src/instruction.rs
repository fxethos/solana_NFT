use {
    crate::{
        deprecated_instruction::{MintPrintingTokensViaTokenArgs, SetReservationListArgs},
        state::{Creator, Data, EDITION, EDITION_MARKER_BIT_SIZE, PREFIX},
    },
    borsh::{BorshDeserialize, BorshSerialize},
    solana_program::{
        instruction::{AccountMeta, Instruction},
        pubkey::Pubkey,
        sysvar,
    },
};

#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone)]
/// Args for update call
pub struct UpdateMetadataAccountArgs {
    pub data: Option<Data>,
    pub update_authority: Option<Pubkey>,
    pub primary_sale_happened: Option<bool>,
}

#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone)]
/// Args for create call
pub struct CreateMetadataAccountArgs {
    /// Note that unique metadatas are disabled for now.
    pub data: Data,
    /// Whether you want your metadata to be updateable in the future.
    pub is_mutable: bool,
}

#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone)]
pub struct CreateMasterEditionArgs {
    pub max_supply: Option<u64>,
}

#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone)]
pub struct MintNewEditionFromMasterEditionViaTokenArgs {
    pub edition: u64,
}

#[derive(BorshSerialize, BorshDeserialize, Clone)]
pub enum MetadataInstruction {
  
    CreateMetadataAccount(CreateMetadataAccountArgs),

    UpdateMetadataAccount(UpdateMetadataAccountArgs),

   
    DeprecatedCreateMasterEdition(CreateMasterEditionArgs),

    
    DeprecatedMintNewEditionFromMasterEditionViaPrintingToken,

 
    UpdatePrimarySaleHappenedViaToken,

    
    DeprecatedSetReservationList(SetReservationListArgs),

   
    DeprecatedCreateReservationList,

 
    SignMetadata,

   
    DeprecatedMintPrintingTokensViaToken(MintPrintingTokensViaTokenArgs),

 
    DeprecatedMintPrintingTokens(MintPrintingTokensViaTokenArgs),

   
    CreateMasterEdition(CreateMasterEditionArgs),

   
    MintNewEditionFromMasterEditionViaToken(MintNewEditionFromMasterEditionViaTokenArgs),

    
    ConvertMasterEditionV1ToV2,

   
    MintNewEditionFromMasterEditionViaVaultProxy(MintNewEditionFromMasterEditionViaTokenArgs),

  
    PuffMetadata,
}

#[allow(clippy::too_many_arguments)]
pub fn create_metadata_accounts(
    program_id: Pubkey,
    metadata_account: Pubkey,
    mint: Pubkey,
    mint_authority: Pubkey,
    payer: Pubkey,
    update_authority: Pubkey,
    name: String,
    symbol: String,
    uri: String,
    creators: Option<Vec<Creator>>,
    seller_fee_basis_points: u16,
    update_authority_is_signer: bool,
    is_mutable: bool,
) -> Instruction {
    Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new(metadata_account, false),
            AccountMeta::new_readonly(mint, false),
            AccountMeta::new_readonly(mint_authority, true),
            AccountMeta::new(payer, true),
            AccountMeta::new_readonly(update_authority, update_authority_is_signer),
            AccountMeta::new_readonly(solana_program::system_program::id(), false),
            AccountMeta::new_readonly(sysvar::rent::id(), false),
        ],
        data: MetadataInstruction::CreateMetadataAccount(CreateMetadataAccountArgs {
            data: Data {
                name,
                symbol,
                uri,
                seller_fee_basis_points,
                creators,
            },
            is_mutable,
        })
        .try_to_vec()
        .unwrap(),
    }
}

pub fn update_metadata_accounts(
    program_id: Pubkey,
    metadata_account: Pubkey,
    update_authority: Pubkey,
    new_update_authority: Option<Pubkey>,
    data: Option<Data>,
    primary_sale_happened: Option<bool>,
) -> Instruction {
    Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new(metadata_account, false),
            AccountMeta::new_readonly(update_authority, true),
        ],
        data: MetadataInstruction::UpdateMetadataAccount(UpdateMetadataAccountArgs {
            data,
            update_authority: new_update_authority,
            primary_sale_happened,
        })
        .try_to_vec()
        .unwrap(),
    }
}

pub fn puff_metadata_account(program_id: Pubkey, metadata_account: Pubkey) -> Instruction {
    Instruction {
        program_id,
        accounts: vec![AccountMeta::new(metadata_account, false)],
        data: MetadataInstruction::PuffMetadata.try_to_vec().unwrap(),
    }
}

#[allow(clippy::too_many_arguments)]
pub fn update_primary_sale_happened_via_token(
    program_id: Pubkey,
    metadata: Pubkey,
    owner: Pubkey,
    token: Pubkey,
) -> Instruction {
    Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new(metadata, false),
            AccountMeta::new_readonly(owner, true),
            AccountMeta::new_readonly(token, false),
        ],
        data: MetadataInstruction::UpdatePrimarySaleHappenedViaToken
            .try_to_vec()
            .unwrap(),
    }
}

#[allow(clippy::too_many_arguments)]
pub fn create_master_edition(
    program_id: Pubkey,
    edition: Pubkey,
    mint: Pubkey,
    update_authority: Pubkey,
    mint_authority: Pubkey,
    metadata: Pubkey,
    payer: Pubkey,
    max_supply: Option<u64>,
) -> Instruction {
    let accounts = vec![
        AccountMeta::new(edition, false),
        AccountMeta::new(mint, false),
        AccountMeta::new_readonly(update_authority, true),
        AccountMeta::new_readonly(mint_authority, true),
        AccountMeta::new(payer, true),
        AccountMeta::new_readonly(metadata, false),
        AccountMeta::new_readonly(spl_token::id(), false),
        AccountMeta::new_readonly(solana_program::system_program::id(), false),
        AccountMeta::new_readonly(sysvar::rent::id(), false),
    ];

    Instruction {
        program_id,
        accounts,
        data: MetadataInstruction::CreateMasterEdition(CreateMasterEditionArgs { max_supply })
            .try_to_vec()
            .unwrap(),
    }
}

#[allow(clippy::too_many_arguments)]
pub fn mint_new_edition_from_master_edition_via_token(
    program_id: Pubkey,
    new_metadata: Pubkey,
    new_edition: Pubkey,
    master_edition: Pubkey,
    new_mint: Pubkey,
    new_mint_authority: Pubkey,
    payer: Pubkey,
    token_account_owner: Pubkey,
    token_account: Pubkey,
    new_metadata_update_authority: Pubkey,
    metadata: Pubkey,
    metadata_mint: Pubkey,
    edition: u64,
) -> Instruction {
    let edition_number = edition.checked_div(EDITION_MARKER_BIT_SIZE).unwrap();
    let as_string = edition_number.to_string();
    let (edition_mark_pda, _) = Pubkey::find_program_address(
        &[
            PREFIX.as_bytes(),
            program_id.as_ref(),
            metadata_mint.as_ref(),
            EDITION.as_bytes(),
            as_string.as_bytes(),
        ],
        &program_id,
    );

    let accounts = vec![
        AccountMeta::new(new_metadata, false),
        AccountMeta::new(new_edition, false),
        AccountMeta::new(master_edition, false),
        AccountMeta::new(new_mint, false),
        AccountMeta::new(edition_mark_pda, false),
        AccountMeta::new_readonly(new_mint_authority, true),
        AccountMeta::new(payer, true),
        AccountMeta::new_readonly(token_account_owner, true),
        AccountMeta::new_readonly(token_account, false),
        AccountMeta::new_readonly(new_metadata_update_authority, false),
        AccountMeta::new_readonly(metadata, false),
        AccountMeta::new_readonly(spl_token::id(), false),
        AccountMeta::new_readonly(solana_program::system_program::id(), false),
        AccountMeta::new_readonly(sysvar::rent::id(), false),
    ];

    Instruction {
        program_id,
        accounts,
        data: MetadataInstruction::MintNewEditionFromMasterEditionViaToken(
            MintNewEditionFromMasterEditionViaTokenArgs { edition },
        )
        .try_to_vec()
        .unwrap(),
    }
}

#[allow(clippy::too_many_arguments)]
pub fn sign_metadata(
    program_id: Pubkey,
    metadata: Pubkey,
    creator: Pubkey,
) -> Instruction {
    Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new(metadata, false),
            AccountMeta::new_readonly(creator, true),
        ],
        data: MetadataInstruction::SignMetadata
            .try_to_vec()
            .unwrap(),
    }
}

#[allow(clippy::too_many_arguments)]
pub fn convert_master_edition_v1_to_v2(
    program_id: Pubkey,
    master_edition: Pubkey,
    one_time_auth: Pubkey,
    printing_mint: Pubkey,
) -> Instruction {
    Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new(master_edition, false),
            AccountMeta::new(one_time_auth, false),
            AccountMeta::new(printing_mint, false),
        ],
        data: MetadataInstruction::ConvertMasterEditionV1ToV2
            .try_to_vec()
            .unwrap(),
    }
}

#[allow(clippy::too_many_arguments)]
pub fn mint_edition_from_master_edition_via_vault_proxy(
    program_id: Pubkey,
    new_metadata: Pubkey,
    new_edition: Pubkey,
    master_edition: Pubkey,
    new_mint: Pubkey,
    edition_mark_pda: Pubkey,
    new_mint_authority: Pubkey,
    payer: Pubkey,
    vault_authority: Pubkey,
    safety_deposit_store: Pubkey,
    safety_deposit_box: Pubkey,
    vault: Pubkey,
    new_metadata_update_authority: Pubkey,
    metadata: Pubkey,
    token_program: Pubkey,
    token_vault_program_info: Pubkey,
    edition: u64,
) -> Instruction {
    let accounts = vec![
        AccountMeta::new(new_metadata, false),
        AccountMeta::new(new_edition, false),
        AccountMeta::new(master_edition, false),
        AccountMeta::new(new_mint, false),
        AccountMeta::new(edition_mark_pda, false),
        AccountMeta::new_readonly(new_mint_authority, true),
        AccountMeta::new(payer, true),
        AccountMeta::new_readonly(vault_authority, true),
        AccountMeta::new_readonly(safety_deposit_store, false),
        AccountMeta::new_readonly(safety_deposit_box, false),
        AccountMeta::new_readonly(vault, false),
        AccountMeta::new_readonly(new_metadata_update_authority, false),
        AccountMeta::new_readonly(metadata, false),
        AccountMeta::new_readonly(token_program, false),
        AccountMeta::new_readonly(token_vault_program_info, false),
        AccountMeta::new_readonly(solana_program::system_program::id(), false),
        AccountMeta::new_readonly(sysvar::rent::id(), false),
    ];

    Instruction {
        program_id,
        accounts,
        data: MetadataInstruction::MintNewEditionFromMasterEditionViaVaultProxy(
            MintNewEditionFromMasterEditionViaTokenArgs { edition },
        )
        .try_to_vec()
        .unwrap(),
    }
}
