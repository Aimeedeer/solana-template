use anyhow::Result;
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    system_program,
};

/// # Accounts
///
/// - 0: payer - writable, signer
/// - 1: recipient - writable
/// - 2: system_program - executable
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct TransferInstruction {
    pub amount: u64,
}

impl TransferInstruction {
    pub fn build_instruction(
        payer: &Pubkey,
        recipient: &Pubkey,
        amount: u64,
        program_id: &Pubkey,
    ) -> Result<Instruction> {
        let instr = TransferInstruction { amount };

        let accounts = vec![
            AccountMeta::new(*payer, true),
            AccountMeta::new(*recipient, false),
            AccountMeta::new_readonly(system_program::ID, false),
        ];

        Ok(Instruction::new_with_borsh(*program_id, &instr, accounts))
    }
}
