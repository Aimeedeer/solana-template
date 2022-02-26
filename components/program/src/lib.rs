use borsh::de::BorshDeserialize;
use common::{ProgramInstruction, TransferInstruction};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program::invoke,
    pubkey::Pubkey,
    system_instruction, system_program,
};

entrypoint!(process_instruction);

fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    msg!("process instruction");
    let instr = ProgramInstruction::deserialize(&mut &instruction_data[..])?;
    let instr: &dyn Exec = match &instr {
        ProgramInstruction::Transfer(instr) => instr,
    };
    instr.exec(program_id, accounts)
}

trait Exec {
    fn exec(&self, program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult;
}

impl Exec for TransferInstruction {
    fn exec(
        &self,
        _program_id: &Pubkey,
        accounts: &[AccountInfo],
    ) -> ProgramResult {
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

        invoke(
            &system_instruction::transfer(payer.key, recipient.key, self.amount),
            &[payer.clone(), recipient.clone(), system_account.clone()],
        )
    }
}
