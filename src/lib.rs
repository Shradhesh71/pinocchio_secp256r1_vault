#![no_std]
 
use pinocchio::{account_info::AccountInfo, entrypoint, nostd_panic_handler, program_error::ProgramError, pubkey::Pubkey, ProgramResult};
 
entrypoint!(process_instruction);
nostd_panic_handler!();
 
pub mod instructions;
pub use instructions::*;
 

pinocchio_pubkey::declare_id!("GWtacCxsGzuySqvAALMbgfTnpVgeFSF6BgubfQiMVmgx");

fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    match instruction_data.split_first() {
        Some((Deposit::DISCRIMINATOR, data)) => Deposit::try_from((data, accounts))?.process(),
        Some((Withdraw::DISCRIMINATOR, data)) => Withdraw::try_from((data, accounts))?.process(),
        _ => Err(ProgramError::InvalidInstructionData),
    }
}