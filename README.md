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

## Benchmarking

The program includes comprehensive compute unit benchmarks for performance analysis:

```bash
cargo bench
```

### Benchmark Details

- **Framework**: Mollusk SVM with MolluskComputeUnitBencher
- **Environment**: Solana CLI 2.2.17 (Agave client)
- **Output**: Generated in `program/benches/compute_units.md`

The deposit benchmark measures a complete successful transaction, while the withdraw benchmark measures the initial setup and validation phase before signature verification (which fails without a proper SECP256R1 signature).

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

## 👨‍💻 Author & Support

**Maintained by [@Shradhesh71](https://github.com/Shradhesh71)**

🐛 **Found a bug?** [Open an issue](https://github.com/Shradhesh71/pinocchio_secp256r1_vault/issues)  
💡 **Feature request?** [Start a discussion](https://github.com/Shradhesh71/pinocchio_secp256r1_vault/discussions)  
📧 **Need help?** Check our documentation or open an issue

---

**Built with ❤️ for the Solana ecosystem**