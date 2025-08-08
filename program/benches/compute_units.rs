use mollusk_svm::{program, Mollusk};
use mollusk_svm_bencher::MolluskComputeUnitBencher;
use solana_pinocchio_starter::{
    instruction::{InitializeMyStateV1IxData, UpdateMyStateV1IxData},
    instruction::{InitializeMyStateV2IxData, UpdateMyStateV2IxData},
    state::{to_bytes, DataLen, MyStateV1,MyStateV2},
    ID,
};
use solana_sdk::{
    account::Account,
    instruction::{AccountMeta, Instruction},
    native_token::LAMPORTS_PER_SOL,
    pubkey::Pubkey,
    sysvar::Sysvar,
};
use solana_sdk::{pubkey, rent::Rent};

pub const PROGRAM: Pubkey = Pubkey::new_from_array(ID);

pub const RENT: Pubkey = pubkey!("SysvarRent111111111111111111111111111111111");

pub const PAYER: Pubkey = pubkey!("Bv1vrbzogVpKNW2iRYJXLRUEVv6gD8xd9gid1Yh6hoiQ");

pub fn get_rent_data() -> Vec<u8> {
    let rent = Rent::default();
    unsafe {
        core::slice::from_raw_parts(&rent as *const Rent as *const u8, Rent::size_of()).to_vec()
    }
}


fn main() {
    let mollusk = Mollusk::new(&PROGRAM, "target/deploy/pinocchio_secp256r1_vault");

    let (system_program, system_account) = program::keyed_account_for_system_program();
    let payer_account = Account::new(1 * LAMPORTS_PER_SOL, 0, &system_program);

}