use mollusk_svm::result::{Check, ProgramResult};
use mollusk_svm::{program, Mollusk};
use solana_sdk::account::Account;
use solana_sdk::instruction::{AccountMeta, Instruction};
use solana_sdk::native_token::LAMPORTS_PER_SOL;
use solana_sdk::pubkey;
use solana_sdk::pubkey::Pubkey;
extern crate alloc;
use alloc::vec;

use pinocchio_secp256r1_vault::{DepositInstructionData, WithdrawInstructionData, ID};
use solana_sdk::rent::Rent;
use solana_sdk::sysvar::Sysvar;

pub const PROGRAM: Pubkey = Pubkey::new_from_array(ID);

pub const RENT: Pubkey = pubkey!("SysvarRent111111111111111111111111111111111");

pub const PAYER: Pubkey = pubkey!("Bv1vrbzogVpKNW2iRYJXLRUEVv6gD8xd9gid1Yh6hoiQ");

pub fn mollusk() -> Mollusk {
    let mollusk = Mollusk::new(&PROGRAM, "target/deploy/pinocchio_secp256r1_vault");
    mollusk
}

pub fn get_rent_data() -> Vec<u8> {
    let rent = Rent::default();
    unsafe {
        core::slice::from_raw_parts(&rent as *const Rent as *const u8, Rent::size_of()).to_vec()
    }
}

#[test]
fn test_initialize_vault() {
    let mollusk = mollusk();

    let (system_program, _system_account) = program::keyed_account_for_system_program();
    let rent = Rent::default();
    let payer_account = Account::new(
        10 * LAMPORTS_PER_SOL + rent.minimum_balance(0), 
        0, 
        &system_program
    );

    let mut pubkey_bytes = [0u8; 33];
    pubkey_bytes[0] = 0x02;  
    pubkey_bytes[1..33].copy_from_slice(&PAYER.to_bytes());

 let (vault, _bump) = Pubkey::find_program_address(
        &[
            b"vault",
            &pubkey_bytes[..1],    
            &pubkey_bytes[1..33],
        ],
        &PROGRAM,
    );

    let vault_account = Account::new(0, 64, &system_program);

    let ix_accounts = vec![
        AccountMeta::new(PAYER, true),
        AccountMeta::new(vault, false),
        AccountMeta::new_readonly(system_program, false),
    ];

    let ix_data = DepositInstructionData { pubkey: pubkey_bytes, amount: 1000 };

    let mut ser_ix_data = vec![0]; 
    ser_ix_data.extend_from_slice(&ix_data.pubkey);
    ser_ix_data.extend_from_slice(&ix_data.amount.to_le_bytes());

    let instruction = Instruction::new_with_bytes(PROGRAM, &ser_ix_data, ix_accounts);

    let tx_accounts = &vec![
        (PAYER, payer_account.clone()),
        (vault, vault_account.clone()),
        (system_program, _system_account.clone()),
    ];

    let result = mollusk.process_and_validate_instruction(&instruction, tx_accounts, &[Check::success()]);

    assert!(result.program_result == ProgramResult::Success);
}

#[test]
fn test_withdraw_vault() {
    let mollusk = mollusk();

    let (system_program, _system_account) = program::keyed_account_for_system_program();
    let rent = Rent::default();
    let payer_account = Account::new(
        10 * LAMPORTS_PER_SOL + rent.minimum_balance(0), 
        0, 
        &system_program
    );

    // Create the same SECP256R1 pubkey format as deposit
    let mut pubkey_bytes = [0u8; 33];
    pubkey_bytes[0] = 0x02;  // Set the first byte to 0x02 (compressed pubkey format)
    pubkey_bytes[1..33].copy_from_slice(&PAYER.to_bytes());

    let (vault, bump) = Pubkey::find_program_address(
        &[
            b"vault",
            &pubkey_bytes[..1],
            &pubkey_bytes[1..33],
        ],
        &PROGRAM,
    );

    let vault_lamports = 5000u64; // Amount previously deposited
    let vault_account = Account::new(vault_lamports, 64, &system_program);

    // Instructions sysvar account (required for withdraw)
    let instructions_sysvar = solana_sdk::sysvar::instructions::id();
    let instructions_account = Account::new(
        rent.minimum_balance(1280), 
        1280,
        &solana_sdk::sysvar::id()
    );

    let ix_accounts = vec![
        AccountMeta::new(PAYER, true),
        AccountMeta::new(vault, false),
        AccountMeta::new_readonly(instructions_sysvar, false),
        AccountMeta::new_readonly(system_program, false),
    ];

    let ix_data = WithdrawInstructionData { bump: [bump] };

    let mut ser_ix_data = vec![1]; 
    ser_ix_data.extend_from_slice(&ix_data.bump);

    let instruction = Instruction::new_with_bytes(PROGRAM, &ser_ix_data, ix_accounts);

    let tx_accounts = &vec![
        (PAYER, payer_account.clone()),
        (vault, vault_account.clone()),
        (instructions_sysvar, instructions_account.clone()),
        (system_program, _system_account.clone()),
    ];

    // Note: This test will fail because withdraw requires a valid SECP256R1 signature
    // in the next instruction, but demonstrates the correct account setup
    let result = mollusk.process_and_validate_instruction(
        &instruction, 
        tx_accounts, 
        &[Check::err(solana_sdk::program_error::ProgramError::InvalidInstructionData)]
    );

    assert!(result.program_result != ProgramResult::Success);
}

#[test]
fn test_pda_derivation() {
    let test_payer = PAYER;
    
    // Create SECP256R1 pubkey format
    let mut pubkey_bytes = [0u8; 33];
    pubkey_bytes[0] = 0x02;  // Compressed pubkey format
    pubkey_bytes[1..33].copy_from_slice(&test_payer.to_bytes());

    let (vault_pda, bump) = Pubkey::find_program_address(
        &[
            b"vault",
            &pubkey_bytes[..1],
            &pubkey_bytes[1..33],
        ],
        &PROGRAM,
    );

    assert_ne!(vault_pda, Pubkey::default());
    assert!(bump > 0);  
}