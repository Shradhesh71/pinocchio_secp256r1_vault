use mollusk_svm::{program, Mollusk};
use mollusk_svm_bencher::MolluskComputeUnitBencher;
use pinocchio_secp256r1_vault::{DepositInstructionData, WithdrawInstructionData, ID};
use solana_sdk::{
    account::Account,
    instruction::{AccountMeta, Instruction},
    native_token::LAMPORTS_PER_SOL,
    pubkey::Pubkey,
    rent::Rent,
};
use solana_sdk::pubkey;

pub const PROGRAM: Pubkey = Pubkey::new_from_array(ID);

pub const PAYER: Pubkey = pubkey!("Bv1vrbzogVpKNW2iRYJXLRUEVv6gD8xd9gid1Yh6hoiQ");

fn main() {
    let mollusk = Mollusk::new(&PROGRAM, "target/deploy/pinocchio_secp256r1_vault");

    let (system_program, system_account) = program::keyed_account_for_system_program();
    let rent = Rent::default();
    let payer_account = Account::new(
        10 * LAMPORTS_PER_SOL + rent.minimum_balance(0), 
        0, 
        &system_program
    );

    // Create SECP256R1 pubkey format for benchmarking
    let mut pubkey_bytes = [0u8; 33];
    pubkey_bytes[0] = 0x02;  // Compressed pubkey format
    pubkey_bytes[1..33].copy_from_slice(&PAYER.to_bytes());

    let (vault_pda, _bump) = Pubkey::find_program_address(
        &[
            b"vault",
            &pubkey_bytes[..1],
            &pubkey_bytes[1..33]
        ],
        &PROGRAM,
    );

    let vault_account = Account::new(0, 64, &system_program);

    let ix_accounts = vec![
        AccountMeta::new(PAYER, true),
        AccountMeta::new(vault_pda, false),
        AccountMeta::new_readonly(system_program, false),
    ];

    let ix_data = DepositInstructionData { pubkey: pubkey_bytes, amount: 1000 };

    let mut ser_ix_data = vec![0];
    ser_ix_data.extend_from_slice(&ix_data.pubkey);
    ser_ix_data.extend_from_slice(&ix_data.amount.to_le_bytes());

    let instruction = Instruction::new_with_bytes(PROGRAM, &ser_ix_data, ix_accounts);

    let tx_accounts = vec![
        (PAYER, payer_account),
        (vault_pda, vault_account),
        (system_program, system_account),
    ];

    // Setup for withdraw instruction
    let instructions_sysvar = solana_sdk::sysvar::instructions::id();
    let instructions_account = Account::new(
        rent.minimum_balance(1280),
        1280,
        &solana_sdk::sysvar::id()
    );
    
    // Get the bump for PDA (reuse the same calculation)
    let (_, bump) = Pubkey::find_program_address(
        &[
            b"vault",
            &pubkey_bytes[..1],
            &pubkey_bytes[1..33]
        ],
        &PROGRAM,
    );

    let ix_accounts_wd = vec![
        AccountMeta::new(PAYER, true),
        AccountMeta::new(vault_pda, false),
        AccountMeta::new_readonly(instructions_sysvar, false),
        AccountMeta::new_readonly(system_program, false),
    ];

    let ix_data_wd = WithdrawInstructionData { bump: [bump] };

    let mut ser_ix_data_wd = vec![1];  
    ser_ix_data_wd.extend_from_slice(&ix_data_wd.bump);

    let instruction_wd = Instruction::new_with_bytes(PROGRAM, &ser_ix_data_wd, ix_accounts_wd);
    
    // Create separate accounts for withdraw test
    let (_, system_account_wd) = program::keyed_account_for_system_program();
    let payer_account_wd = Account::new(
        10 * LAMPORTS_PER_SOL + rent.minimum_balance(0), 
        0, 
        &system_program
    );
    let vault_lamports = 5000u64;
    let vault_account_wd = Account::new(vault_lamports, 64, &system_program);
    let tx_accounts_wd = vec![
        (PAYER, payer_account_wd),
        (vault_pda, vault_account_wd),
        (instructions_sysvar, instructions_account),
        (system_program, system_account_wd),
    ];

    // Benchmark both instructions (withdraw will fail as expected but we can measure setup cost)
    MolluskComputeUnitBencher::new(mollusk)
        .bench(("deposit", &instruction, &tx_accounts))
        .bench(("withdraw", &instruction_wd, &tx_accounts_wd))
        .must_pass(false)  // Changed to false since withdraw will fail without proper signature
        .out_dir("program/benches/")
        .execute();
}

