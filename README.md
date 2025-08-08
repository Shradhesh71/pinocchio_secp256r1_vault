# Pinocchio SECP256R1 Vault

A Solana program implementing a secure vault system with SECP256R1 cryptographic signatures for withdrawals.

## Overview

This program provides a vault mechanism where users can deposit SOL tokens and withdraw them using SECP256R1 signature verification. The vault uses Program Derived Addresses (PDAs) for security and deterministic account generation.

## Features

- **Deposit**: Transfer SOL to a user-specific vault
- **Withdraw**: Retrieve SOL from vault with SECP256R1 signature verification
- **PDA-based Security**: Deterministic vault addresses using SECP256R1 public keys
- **Time-based Expiry**: Withdrawal signatures include expiration timestamps

## Program Structure

```
src/
├── lib.rs                 # Main program entrypoint
└── instructions/
    ├── mod.rs            # Module exports
    ├── deposit.rs        # Deposit instruction implementation
    └── withdraw.rs       # Withdraw instruction implementation
```

## Instructions

### Deposit (Discriminator: 0)

Deposits SOL into a vault associated with a SECP256R1 public key.

**Accounts:**
- `[writable, signer]` Payer account
- `[writable]` Vault PDA account
- `[]` System program

**Data:**
- `pubkey: [u8; 33]` - SECP256R1 public key (compressed format)
- `amount: u64` - Amount to deposit in lamports

### Withdraw (Discriminator: 1)

Withdraws all SOL from a vault using SECP256R1 signature verification.

**Accounts:**
- `[writable, signer]` Recipient account
- `[writable]` Vault PDA account
- `[]` Instructions sysvar
- `[]` System program

**Data:**
- `bump: [u8; 1]` - PDA bump seed

## PDA Derivation

Vault addresses are derived using:
```
seeds = ["vault", &pubkey[0..1], &pubkey[1..33]]
program_id = "GWtacCxsGzuySqvAALMbgfTnpVgeFSF6BgubfQiMVmgx"
```

## Dependencies

- **pinocchio**: Core Solana program framework
- **pinocchio-secp256r1-instruction**: SECP256R1 signature verification
- **pinocchio-system**: System program interactions

## Testing

Run the test suite:
```bash
cargo test --test vault
```

Available tests:
- `test_initialize_vault` - Tests deposit functionality
- `test_withdraw_vault` - Tests withdraw setup
- `test_pda_derivation` - Validates PDA generation

## Building

Build the program:
```bash
cargo build-sbf
```

Deploy to Solana:
```bash
solana program deploy target/deploy/pinocchio_secp256r1_vault.so
```

## Security Considerations

- Withdrawals require valid SECP256R1 signatures
- Signature messages include expiration timestamps
- Only the vault owner can initiate withdrawals
- All lamports are transferred on withdrawal

## Program ID

```
GWtacCxsGzuySqvAALMbgfTnpVgeFSF6BgubfQiMVmgx
```
