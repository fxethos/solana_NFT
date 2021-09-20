pub mod utils;

use {
    crate::utils::{assert_initialized, assert_owned_by, spl_token_transfer, TokenTransferParams},
    anchor_lang::{
        prelude::*, solana_program::system_program, AnchorDeserialize, AnchorSerialize,
        Discriminator, Key,
    },
    arrayref::array_ref,
    spl_token::state::{Account, Mint},
    spl_token_metadata::{
        instruction::{create_master_edition, create_metadata_accounts, update_metadata_accounts},
        state::{
            MAX_CREATOR_LEN, MAX_CREATOR_LIMIT, MAX_NAME_LENGTH, MAX_SYMBOL_LENGTH, MAX_URI_LENGTH,
        },
    },
    std::cell::Ref,
};

const PREFIX: &str = "nft_vault";
#[program]
pub mod nft_nft_vault {
    use anchor_lang::solana_program::{
        program::{invoke, invoke_signed},
        system_instruction,
    };

    use super::*;

    pub fn mint_nft<'info>(ctx: Context<'_, '_, '_, 'info, MintNFT<'info>>) -> ProgramResult {
        let nft_vault = &mut ctx.accounts.nft_vault;
        let config = &ctx.accounts.config;
        let clock = &ctx.accounts.clock;

        match nft_vault.data.go_live_date {
            None => {
                if *ctx.accounts.payer.key != nft_vault.authority {
                    return Err(ErrorCode::NftVaultNotLiveYet.into());
                }
            }
            Some(val) => {
                if clock.unix_timestamp < val {
                    if *ctx.accounts.payer.key != nft_vault.authority {
                        return Err(ErrorCode::NftVaultNotLiveYet.into());
                    }
                }
            }
        }

        if nft_vault.items_redeemed >= nft_vault.data.items_available {
            return Err(ErrorCode::NftVaultEmpty.into());
        }

        if let Some(mint) = nft_vault.token_mint {
            let token_account_info = &ctx.remaining_accounts[0];
            let transfer_authority_info = &ctx.remaining_accounts[1];
            let token_account: Account = assert_initialized(&token_account_info)?;

            assert_owned_by(&token_account_info, &spl_token::id())?;

            if token_account.mint != mint {
                return Err(ErrorCode::MintMismatch.into());
            }

            if token_account.amount < nft_vault.data.price {
                return Err(ErrorCode::NotEnoughTokens.into());
            }

            spl_token_transfer(TokenTransferParams {
                source: token_account_info.clone(),
                destination: ctx.accounts.wallet.clone(),
                authority: transfer_authority_info.clone(),
                authority_signer_seeds: &[],
                token_program: ctx.accounts.token_program.clone(),
                amount: nft_vault.data.price,
            })?;
        } else {
            if ctx.accounts.payer.lamports() < nft_vault.data.price {
                return Err(ErrorCode::NotEnoughSOL.into());
            }

            invoke(
                &system_instruction::transfer(
                    &ctx.accounts.payer.key,
                    ctx.accounts.wallet.key,
                    nft_vault.data.price,
                ),
                &[
                    ctx.accounts.payer.clone(),
                    ctx.accounts.wallet.clone(),
                    ctx.accounts.system_program.clone(),
                ],
            )?;
        }

        let config_line = get_config_line(
            &config.to_account_info(),
            nft_vault.items_redeemed as usize,
        )?;

        nft_vault.items_redeemed = nft_vault
            .items_redeemed
            .checked_add(1)
            .ok_or(ErrorCode::NumericalOverflowError)?;

        let config_key = config.key();
        let authority_seeds = [
            PREFIX.as_bytes(),
            config_key.as_ref(),
            nft_vault.data.uuid.as_bytes(),
            &[nft_vault.bump],
        ];

        let mut creators: Vec<spl_token_metadata::state::Creator> =
            vec![spl_token_metadata::state::Creator {
                address: nft_vault.key(),
                verified: true,
                share: 0,
            }];

        for c in &config.data.creators {
            creators.push(spl_token_metadata::state::Creator {
                address: c.address,
                verified: false,
                share: c.share,
            });
        }

        let metadata_infos = vec![
            ctx.accounts.metadata.clone(),
            ctx.accounts.mint.clone(),
            ctx.accounts.mint_authority.clone(),
            ctx.accounts.payer.clone(),
            ctx.accounts.token_metadata_program.clone(),
            ctx.accounts.token_program.clone(),
            ctx.accounts.system_program.clone(),
            ctx.accounts.rent.to_account_info().clone(),
            nft_vault.to_account_info().clone(),
        ];

        let master_edition_infos = vec![
            ctx.accounts.master_edition.clone(),
            ctx.accounts.mint.clone(),
            ctx.accounts.mint_authority.clone(),
            ctx.accounts.payer.clone(),
            ctx.accounts.metadata.clone(),
            ctx.accounts.token_metadata_program.clone(),
            ctx.accounts.token_program.clone(),
            ctx.accounts.system_program.clone(),
            ctx.accounts.rent.to_account_info().clone(),
            nft_vault.to_account_info().clone(),
        ];

        invoke_signed(
            &create_metadata_accounts(
                *ctx.accounts.token_metadata_program.key,
                *ctx.accounts.metadata.key,
                *ctx.accounts.mint.key,
                *ctx.accounts.mint_authority.key,
                *ctx.accounts.payer.key,
                nft_vault.key(),
                config_line.name,
                config.data.symbol.clone(),
                config_line.uri,
                Some(creators),
                config.data.seller_fee_basis_points,
                false,
                config.data.is_mutable,
            ),
            metadata_infos.as_slice(),
            &[&authority_seeds],
        )?;

        invoke_signed(
            &create_master_edition(
                *ctx.accounts.token_metadata_program.key,
                *ctx.accounts.master_edition.key,
                *ctx.accounts.mint.key,
                nft_vault.key(),
                *ctx.accounts.mint_authority.key,
                *ctx.accounts.metadata.key,
                *ctx.accounts.payer.key,
                Some(config.data.max_supply),
            ),
            master_edition_infos.as_slice(),
            &[&authority_seeds],
        )?;

        let mut new_update_authority = Some(nft_vault.authority);

        if !ctx.accounts.config.data.retain_authority {
            new_update_authority = Some(ctx.accounts.update_authority.key());
        }

        invoke_signed(
            &update_metadata_accounts(
                *ctx.accounts.token_metadata_program.key,
                *ctx.accounts.metadata.key,
                nft_vault.key(),
                new_update_authority,
                None,
                Some(true),
            ),
            &[
                ctx.accounts.token_metadata_program.clone(),
                ctx.accounts.metadata.clone(),
                nft_vault.to_account_info().clone(),
            ],
            &[&authority_seeds],
        )?;

        Ok(())
    }

    pub fn update_nft_vault(
        ctx: Context<UpdateNftVault>,
        price: Option<u64>,
        go_live_date: Option<i64>,
    ) -> ProgramResult {
        let nft_vault = &mut ctx.accounts.nft_vault;

        if let Some(p) = price {
            nft_vault.data.price = p;
        }

        if let Some(go_l) = go_live_date {
            msg!("Go live date changed to {}", go_l);
            nft_vault.data.go_live_date = Some(go_l)
        }
        Ok(())
    }

    pub fn initialize_config(ctx: Context<InitializeConfig>, data: ConfigData) -> ProgramResult {
        let config_info = &mut ctx.accounts.config;
        if data.uuid.len() != 6 {
            return Err(ErrorCode::UuidMustBeExactly6Length.into());
        }

        let mut config = Config {
            data,
            authority: *ctx.accounts.authority.key,
        };

        let mut array_of_zeroes = vec![];
        while array_of_zeroes.len() < MAX_SYMBOL_LENGTH - config.data.symbol.len() {
            array_of_zeroes.push(0u8);
        }
        let new_symbol =
            config.data.symbol.clone() + std::str::from_utf8(&array_of_zeroes).unwrap();
        config.data.symbol = new_symbol;

        // - 1 because we are going to be a creator
        if config.data.creators.len() > MAX_CREATOR_LIMIT - 1 {
            return Err(ErrorCode::TooManyCreators.into());
        }

        let mut new_data = Config::discriminator().try_to_vec().unwrap();
        new_data.append(&mut config.try_to_vec().unwrap());
        let mut data = config_info.data.borrow_mut();
        // god forgive me couldnt think of better way to deal with this
        for i in 0..new_data.len() {
            data[i] = new_data[i];
        }

        let vec_start =
            CONFIG_ARRAY_START + 4 + (config.data.max_number_of_lines as usize) * CONFIG_LINE_SIZE;
        let as_bytes = (config
            .data
            .max_number_of_lines
            .checked_div(8)
            .ok_or(ErrorCode::NumericalOverflowError)? as u32)
            .to_le_bytes();
        for i in 0..4 {
            data[vec_start + i] = as_bytes[i]
        }

        Ok(())
    }

    pub fn add_config_lines(
        ctx: Context<AddConfigLines>,
        index: u32,
        config_lines: Vec<ConfigLine>,
    ) -> ProgramResult {
        let config = &mut ctx.accounts.config;
        let account = config.to_account_info();
        let current_count = get_config_count(&account.data.borrow())?;
        let mut data = account.data.borrow_mut();

        let mut fixed_config_lines = vec![];

        if index > config.data.max_number_of_lines - 1 {
            return Err(ErrorCode::IndexGreaterThanLength.into());
        }

        for line in &config_lines {
            let mut array_of_zeroes = vec![];
            while array_of_zeroes.len() < MAX_NAME_LENGTH - line.name.len() {
                array_of_zeroes.push(0u8);
            }
            let name = line.name.clone() + std::str::from_utf8(&array_of_zeroes).unwrap();

            let mut array_of_zeroes = vec![];
            while array_of_zeroes.len() < MAX_URI_LENGTH - line.uri.len() {
                array_of_zeroes.push(0u8);
            }
            let uri = line.uri.clone() + std::str::from_utf8(&array_of_zeroes).unwrap();
            fixed_config_lines.push(ConfigLine { name, uri })
        }

        let as_vec = fixed_config_lines.try_to_vec()?;
        let serialized: &[u8] = &as_vec.as_slice()[4..];

        let position = CONFIG_ARRAY_START + 4 + (index as usize) * CONFIG_LINE_SIZE;

        let array_slice: &mut [u8] =
            &mut data[position..position + fixed_config_lines.len() * CONFIG_LINE_SIZE];
        array_slice.copy_from_slice(serialized);

        let bit_mask_vec_start = CONFIG_ARRAY_START
            + 4
            + (config.data.max_number_of_lines as usize) * CONFIG_LINE_SIZE
            + 4;

        let mut new_count = current_count;
        for i in 0..fixed_config_lines.len() {
            let position = (index as usize)
                .checked_add(i)
                .ok_or(ErrorCode::NumericalOverflowError)?;
            let my_position_in_vec = bit_mask_vec_start
                + position
                    .checked_div(8)
                    .ok_or(ErrorCode::NumericalOverflowError)?;
            let position_from_right = 7 - position
                .checked_rem(8)
                .ok_or(ErrorCode::NumericalOverflowError)?;
            let mask = u8::pow(2, position_from_right as u32);

            let old_value_in_vec = data[my_position_in_vec];
            data[my_position_in_vec] = data[my_position_in_vec] | mask;
            msg!(
                "My position in vec is {} my mask is going to be {}, the old value is {}",
                position,
                mask,
                old_value_in_vec
            );
            msg!(
                "My new value is {} and my position from right is {}",
                data[my_position_in_vec],
                position_from_right
            );
            if old_value_in_vec != data[my_position_in_vec] {
                msg!("Increasing count");
                new_count = new_count
                    .checked_add(1)
                    .ok_or(ErrorCode::NumericalOverflowError)?;
            }
        }

        data[CONFIG_ARRAY_START..CONFIG_ARRAY_START + 4]
            .copy_from_slice(&(new_count as u32).to_le_bytes());

        Ok(())
    }

    pub fn initialize_nft_vault(
        ctx: Context<InitializeNftVault>,
        bump: u8,
        data: NftVaultData,
    ) -> ProgramResult {
        let nft_vault = &mut ctx.accounts.nft_vault;

        if data.uuid.len() != 6 {
            return Err(ErrorCode::UuidMustBeExactly6Length.into());
        }
        nft_vault.data = data;
        nft_vault.wallet = *ctx.accounts.wallet.key;
        nft_vault.authority = *ctx.accounts.authority.key;
        nft_vault.config = ctx.accounts.config.key();
        nft_vault.bump = bump;
        if ctx.remaining_accounts.len() > 0 {
            let token_mint_info = &ctx.remaining_accounts[0];
            let _token_mint: Mint = assert_initialized(&token_mint_info)?;
            let token_account: Account = assert_initialized(&ctx.accounts.wallet)?;

            assert_owned_by(&token_mint_info, &spl_token::id())?;
            assert_owned_by(&ctx.accounts.wallet, &spl_token::id())?;

            if token_account.mint != *token_mint_info.key {
                return Err(ErrorCode::MintMismatch.into());
            }

            nft_vault.token_mint = Some(*token_mint_info.key);
        }

        if get_config_count(&ctx.accounts.config.to_account_info().data.borrow())?
            < nft_vault.data.items_available as usize
        {
            return Err(ErrorCode::ConfigLineMismatch.into());
        }

        let _config_line = match get_config_line(&ctx.accounts.config.to_account_info(), 0) {
            Ok(val) => val,
            Err(_) => return Err(ErrorCode::ConfigMustHaveAtleastOneEntry.into()),
        };

        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(bump: u8, data: NftVaultData)]
pub struct InitializeNftVault<'info> {
    #[account(init, seeds=[PREFIX.as_bytes(), config.key().as_ref(), data.uuid.as_bytes()], payer=payer, bump=bump, space=8+32+32+33+32+64+64+64+200)]
    nft_vault: ProgramAccount<'info, NftVault>,
    #[account(constraint= wallet.owner == &spl_token::id() || (wallet.data_is_empty() && wallet.lamports() > 0) )]
    wallet: AccountInfo<'info>,
    #[account(has_one=authority)]
    config: ProgramAccount<'info, Config>,
    #[account(signer, constraint= authority.data_is_empty() && authority.lamports() > 0)]
    authority: AccountInfo<'info>,
    #[account(mut, signer)]
    payer: AccountInfo<'info>,
    #[account(address = system_program::ID)]
    system_program: AccountInfo<'info>,
    rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
#[instruction(data: ConfigData)]
pub struct InitializeConfig<'info> {
    #[account(mut, constraint= config.to_account_info().owner == program_id && config.to_account_info().data_len() >= CONFIG_ARRAY_START+4+(data.max_number_of_lines as usize)*CONFIG_LINE_SIZE + 4 + (data.max_number_of_lines.checked_div(8).ok_or(ErrorCode::NumericalOverflowError)? as usize))]
    config: AccountInfo<'info>,
    #[account(constraint= authority.data_is_empty() && authority.lamports() > 0 )]
    authority: AccountInfo<'info>,
    #[account(mut, signer)]
    payer: AccountInfo<'info>,
    rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct AddConfigLines<'info> {
    #[account(mut, has_one = authority)]
    config: ProgramAccount<'info, Config>,
    #[account(signer)]
    authority: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct MintNFT<'info> {
    config: ProgramAccount<'info, Config>,
    #[account(
        mut,
        has_one = config,
        has_one = wallet,
        seeds = [PREFIX.as_bytes(), config.key().as_ref(), nft_vault.data.uuid.as_bytes()],
        bump = nft_vault.bump,
    )]
    nft_vault: ProgramAccount<'info, NftVault>,
    #[account(mut, signer)]
    payer: AccountInfo<'info>,
    #[account(mut)]
    wallet: AccountInfo<'info>,
 
    #[account(mut)]
    metadata: AccountInfo<'info>,
    #[account(mut)]
    mint: AccountInfo<'info>,
    #[account(signer)]
    mint_authority: AccountInfo<'info>,
    #[account(signer)]
    update_authority: AccountInfo<'info>,
    #[account(mut)]
    master_edition: AccountInfo<'info>,
    #[account(address = spl_token_metadata::id())]
    token_metadata_program: AccountInfo<'info>,
    #[account(address = spl_token::id())]
    token_program: AccountInfo<'info>,
    #[account(address = system_program::ID)]
    system_program: AccountInfo<'info>,
    rent: Sysvar<'info, Rent>,
    clock: Sysvar<'info, Clock>,
}

#[derive(Accounts)]
pub struct UpdateNftVault<'info> {
    #[account(
        mut,
        has_one = authority,
        seeds = [PREFIX.as_bytes(), nft_vault.config.key().as_ref(), nft_vault.data.uuid.as_bytes()],
        bump = nft_vault.bump
    )]
    nft_vault: ProgramAccount<'info, NftVault>,
    #[account(signer)]
    authority: AccountInfo<'info>,
}

#[account]
#[derive(Default)]
pub struct NftVault {
    pub authority: Pubkey,
    pub wallet: Pubkey,
    pub token_mint: Option<Pubkey>,
    pub config: Pubkey,
    pub data: NftVaultData,
    pub items_redeemed: u64,
    pub bump: u8,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
pub struct NftVaultData {
    pub uuid: String,
    pub price: u64,
    pub items_available: u64,
    pub go_live_date: Option<i64>,
}

pub const CONFIG_ARRAY_START: usize = 32 + 
4 + 6 + 
4 + MAX_SYMBOL_LENGTH + 
2 + 
1 + 4 + MAX_CREATOR_LIMIT*MAX_CREATOR_LEN + 
8 + 
1 + 
1 + 
4;

#[account]
#[derive(Default)]
pub struct Config {
    pub authority: Pubkey,
    pub data: ConfigData,
   
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Default)]
pub struct ConfigData {
    pub uuid: String,
    pub symbol: String,
    pub seller_fee_basis_points: u16,
    pub creators: Vec<Creator>,
    pub max_supply: u64,
    pub is_mutable: bool,
    pub retain_authority: bool,
    pub max_number_of_lines: u32,
}

pub fn get_config_count(data: &Ref<&mut [u8]>) -> core::result::Result<usize, ProgramError> {
    return Ok(u32::from_le_bytes(*array_ref![data, CONFIG_ARRAY_START, 4]) as usize);
}

pub fn get_config_line(
    a: &AccountInfo,
    index: usize,
) -> core::result::Result<ConfigLine, ProgramError> {
    let arr = a.data.borrow();

    let total = get_config_count(&arr)?;
    if index > total {
        return Err(ErrorCode::IndexGreaterThanLength.into());
    }
    let data_array = &arr[CONFIG_ARRAY_START + 4 + index * (CONFIG_LINE_SIZE)
        ..CONFIG_ARRAY_START + 4 + (index + 1) * (CONFIG_LINE_SIZE)];

    let config_line: ConfigLine = ConfigLine::try_from_slice(data_array)?;

    Ok(config_line)
}

pub const CONFIG_LINE_SIZE: usize = 4 + MAX_NAME_LENGTH + 4 + MAX_URI_LENGTH;
#[derive(AnchorSerialize, AnchorDeserialize, Debug)]
pub struct ConfigLine {
    pub name: String,
    pub uri: String,
}


#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct Creator {
    pub address: Pubkey,
    pub verified: bool,
    pub share: u8,
}

#[error]
pub enum ErrorCode {
    #[msg("Account does not have correct owner!")]
    IncorrectOwner,
    #[msg("Account is not initialized!")]
    Uninitialized,
    #[msg("Mint Mismatch!")]
    MintMismatch,
    #[msg("Index greater than length!")]
    IndexGreaterThanLength,
    #[msg("Config must have atleast one entry!")]
    ConfigMustHaveAtleastOneEntry,
    #[msg("Numerical overflow error!")]
    NumericalOverflowError,
    #[msg("Can only provide up to 4 creators to nft vault (because nft vault is one)!")]
    TooManyCreators,
    #[msg("Uuid must be exactly of 6 length")]
    UuidMustBeExactly6Length,
    #[msg("Not enough tokens to pay for this minting")]
    NotEnoughTokens,
    #[msg("Not enough SOL to pay for this minting")]
    NotEnoughSOL,
    #[msg("Token transfer failed")]
    TokenTransferFailed,
    #[msg("nft vault is empty!")]
    NftVaultEmpty,
    #[msg("nft vault is not live yet!")]
    NftVaultNotLiveYet,
    #[msg("Number of config lines must be at least number of items available")]
    ConfigLineMismatch,
}
