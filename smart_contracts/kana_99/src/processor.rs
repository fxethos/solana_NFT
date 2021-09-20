use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    log::sol_log_compute_units,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};
use percentage::Percentage;

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct InstructionData {
    pub receiveddata: String,
    pub datatype: u64,
}

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();

    let message = InstructionData::try_from_slice(instruction_data).map_err(|err| {
        msg!("Receiving message as string utf8 failed, {:?}", err);
        ProgramError::InvalidInstructionData
    })?;

    //msg!("message {:?}", message);

    if message.datatype == 0 {
        let account = next_account_info(accounts_iter)?;
        msg!("contestid {:?}",message.receiveddata);
        let data = &mut &mut account.data.borrow_mut();
        data[..instruction_data.len()].copy_from_slice(&instruction_data);
        //msg!("after length {:?}", instruction_data.len());
        sol_log_compute_units();
    } else {
        
        //msg!("Recieved request for payouts ");
        let source_info = next_account_info(accounts_iter)?;
        let amount = message.receiveddata;
        //msg!("array lengtht {:?}",message.datatype);

        if source_info.owner != program_id {
            msg!(" account does not have the correct program id");
            return Err(ProgramError::IncorrectProgramId);
        };
        for i in 0..message.datatype{
                let destination_info = next_account_info(accounts_iter)?;
                //msg!("destination account {:?}" ,destination_info);
                let total_amount = amount.parse::<f64>().unwrap();
                //msg!("{:?}", total_amount);
                let rank_1 = Percentage::from_decimal(0.5);
                let rank_2 = Percentage::from_decimal(0.3);
                let rank_3 = Percentage::from_decimal(0.2);
                // msg!("50%   is: {}", rank_1.apply_to(total_amount));
                // msg!("30%   is: {}", rank_2.apply_to(total_amount));
                // msg!("20%  is: {}", rank_3.apply_to(total_amount));
                let payable_amount;

                if i==0 {
                    payable_amount =  1000000000 as f64 * rank_1.apply_to(total_amount)   ;
                    msg!("paid : {}", payable_amount as u64);
                     **source_info.try_borrow_mut_lamports()? -= payable_amount as u64;
                     **destination_info.try_borrow_mut_lamports()? += payable_amount as u64;
                }else  if i==1{
                    payable_amount = 1000000000 as f64 * rank_2.apply_to(total_amount);
                    msg!("paid {}", payable_amount as u64);
                    **source_info.try_borrow_mut_lamports()? -= payable_amount as u64;
                    **destination_info.try_borrow_mut_lamports()? += payable_amount as u64;
                }else if i==2{
                    payable_amount = 1000000000 as f64 * rank_3.apply_to(total_amount);
                    msg!("paid {}", payable_amount as u64);
                    **source_info.try_borrow_mut_lamports()? -= payable_amount as u64;
                    **destination_info.try_borrow_mut_lamports()? += payable_amount as u64;
                }
       
    };
    };

   
    Ok(())

    
}

