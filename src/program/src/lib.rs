use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    pubkey::Pubkey,
    msg,
};

#[cfg(not(feature = "no-entrypoint"))]
mod entrypoint {
    use super::process_instruction;
    use solana_program::entrypoint;
    entrypoint!(process_instruction);
}

mod processor;

fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    msg!("process instruction");

    let account_info_iter = &mut accounts.iter();

    let program_id = next_account_info(account_info_iter)?;
    let system_program = next_account_info(account_info_iter)?;

    msg!("program_id: {:?}", program_id.key);
    msg!("system_program: {:?}", system_program.key);
    
    processor::exec(program_id.key, accounts, instruction_data)?;
    
    Ok(())
}
