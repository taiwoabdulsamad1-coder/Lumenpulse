# Soroban (Stellar) On-Chain Setup for Lumenpulse

This directory contains a Soroban Rust workspace for Stellar smart contracts.

## Prerequisites

- Rust toolchain (stable) and `wasm32-unknown-unknown` target
  - `rustup target add wasm32-unknown-unknown`
- Soroban CLI
  - `cargo install soroban-cli`
- Stellar testnet account and network config (Soroban CLI manages this)

## Workspace

- `Cargo.toml` (workspace)
- `contracts/lumenpulse` (starter Soroban contract crate)

## Build

From the repository root or `apps/on-chain`:

```
cargo build -p lumenpulse-contract --target wasm32-unknown-unknown --release
```

You can also use Soroban CLI:

```
soroban contract build --manifest-path contracts/lumenpulse/Cargo.toml
```

The compiled WASM will be under `target/wasm32-unknown-unknown/release/`.

## Deploy (Testnet)

1. Generate or import a key for the admin caller:
```
soroban keys generate lumenpulse-admin
```

2. Set network (built-in testnet):
```
soroban network add --rpc-url https://rpc-futurenet.stellar.org --network-passphrase "Test SDF Network ; September 2015"
```

3. Deploy the contract:
```
soroban contract deploy \
  --wasm target/wasm32-unknown-unknown/release/lumenpulse_contract.wasm \
  --source lumenpulse-admin \
  --network futurenet
```
Note: WASM filename matches the crate name (`lumenpulse-contract` -> `lumenpulse_contract.wasm`).

4. Initialize with admin:
```
soroban contract invoke \
  --id <DEPLOYED_CONTRACT_ID> \
  --source lumenpulse-admin \
  --network futurenet \
  -- \
  init --admin <ADMIN_ADDRESS>
```

5. Set and get greeting:
```
# Set (requires admin auth)
soroban contract invoke \
  --id <DEPLOYED_CONTRACT_ID> \
  --source lumenpulse-admin \
  --network futurenet \
  -- \
  set_greeting --caller <ADMIN_ADDRESS> --greeting "Hello, Stellar!"

# Get
soroban contract invoke \
  --id <DEPLOYED_CONTRACT_ID> \
  --network futurenet \
  -- \
  get_greeting
```

## PowerShell Helper

See `scripts/deploy.ps1` for a simple Windows helper to build and deploy on futurenet.

## Next Steps

- Add more contracts under `contracts/`
- Integrate backend with Soroban via Horizon/Soroban RPC as needed
- Replace StarkNet analytics with Stellar/Horizon data flows (separate task)