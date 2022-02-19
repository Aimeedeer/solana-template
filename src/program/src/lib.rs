use borsh::de::BorshDeserialize;
use common::TransferInstruction;
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program::invoke,
    pubkey::Pubkey,
    system_instruction, system_program,
};

#[cfg(not(feature = "no-entrypoint"))]
mod entrypoint {
    use super::exec_transfer_instruction;
    use solana_program::entrypoint;
    entrypoint!(exec_transfer_instruction);
}

fn exec_transfer_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    msg!("process instruction");

    let account_info_iter = &mut accounts.iter();

    let payer = next_account_info(account_info_iter)?;
    let recipient = next_account_info(account_info_iter)?;
    let system_account = next_account_info(account_info_iter)?;

    {
        msg!("payer: {:?}", payer.key);
        msg!("recipient: {:?}", recipient.key);

        assert!(payer.is_writable);
        assert!(payer.is_signer);
        assert!(recipient.is_writable);

        assert_eq!(&system_program::ID, system_account.key);
        assert!(system_account.executable);
    }

    let mut instruction_data = instruction_data;
    let transfer_instr = TransferInstruction::deserialize(&mut instruction_data)?;

    invoke(
        &system_instruction::transfer(payer.key, recipient.key, transfer_instr.amount),
        &[payer.clone(), recipient.clone(), system_account.clone()],
    )
}
