# 🌟 StakebyFi Staking Contract

This is a simple **staking contract** built using MultiversX Smart Contracts. It allows users to stake and unstake EGLD tokens while keeping track of staking positions and staked addresses.

## 🚀 Features
- ✅ **Stake EGLD**: Users can stake EGLD tokens by calling the `stake` function.
- 🔄 **Unstake EGLD**: Users can withdraw their staked EGLD using the `unstake` function.
- 📊 **Track Staking Positions**: Keeps a record of each user's staking amount.
- 📌 **List Staked Addresses**: Stores all addresses that have staked EGLD.

## 📜 Contract Methods

### 🔧 Initialization
```rust
#[init]
fn init(&self) {}
```
Initializes the contract (no specific logic required).

### 💰 Stake EGLD
```rust
#[payable("EGLD")]
#[endpoint]
fn stake(&self) {
    let payment_amount = self.call_value().egld_value().clone_value();
    require!(payment_amount > 0, "Must pay more than 0");
    
    let caller = self.blockchain().get_caller();
    self.staking_position(&caller).update(|current_amount| {
        *current_amount += payment_amount;
    });
    self.staked_addresses().insert(caller);
}
```
- Accepts EGLD payments.
- Stores the staked amount per user.
- Adds the user to the staked addresses list.

### 🔓 Unstake EGLD
```rust
#[endpoint]
fn unstake(&self) {
    let caller = self.blockchain().get_caller();
    let stake_mapper = self.staking_position(&caller);

    let caller_stake = stake_mapper.get();
    if caller_stake == 0 {
        return;
    }

    self.staked_addresses().swap_remove(&caller);
    stake_mapper.clear();

    self.send().direct_egld(&caller, &caller_stake);
}
```
- Checks if the user has a staking balance.
- Removes the user from the staked addresses list.
- Sends the staked EGLD back to the user.

### 🔍 View Functions
```rust
#[view(getStakedAddresses)]
#[storage_mapper("stakedAddresses")]
fn staked_addresses(&self) -> UnorderedSetMapper<ManagedAddress>;
```
- Returns the list of all addresses that have staked EGLD.

```rust
#[view(getStakingPosition)]
#[storage_mapper("stakingPosition")]
fn staking_position(&self, addr: &ManagedAddress) -> SingleValueMapper<BigUint>;
```
- Retrieves the staking amount for a given address.

## 🛠 Project Setup
### 📌 Prerequisites
Ensure you have the MultiversX Rust environment set up:
```sh
rustup target add wasm32-unknown-unknown
cargo install multiversx-sc-meta multiversx-sc
```

### 🔐 Example

To run the contract, rename the following files by removing `.example` from their filenames:

```sh
deploy-devnet.interaction.json.example  →  deploy-devnet.interaction.json
upgrade-devnet.interaction.json.example  →  upgrade-devnet.interaction.json
wallet-owner.pem.example  →  wallet-owner.pem
```

### 💵 Using Your Wallet

To compile the contract, follow these steps:

1. **Create a Wallet**  
   - Go to [MultiversX Devnet Wallet](https://devnet-wallet.multiversx.com/) and create your wallet.

2. **Claim Faucet Funds**  
   - You need testnet funds to execute transactions. Claim them from the [MultiversX Devnet Faucet](https://devnet-wallet.multiversx.com/).

### 🔨 Build
To compile the contract, run:
```sh
sc-meta all build
```

### 🧪 Testing
Run the tests using:
```sh
cargo test
```

### 🛫 Deploy
Run the tests using:
```sh
mxpy --verbose contract deploy --bytecode=./output/staking-contract.wasm \
    --recall-nonce --pem=./wallet/wallet-owner.pem \
    --gas-limit=10000000 \
    --send --outfile="deploy-devnet.interaction.json" --wait-result \
    --proxy=https://devnet-gateway.multiversx.com --chain=D
```

## 📂 Project Structure
```
staking-contract/
│── src/
│   ├── staking_contract.rs  # Main contract logic
│── Cargo.toml               # Project dependencies
```

## 📦 Dependencies
```toml
[dependencies.multiversx-sc]
version = "0.56.1"

[dev-dependencies]
num-bigint = "0.4"

[dev-dependencies.multiversx-sc-scenario]
version = "0.56.1"
```

## ✍️ Authors
- **you**

## 📜 License
This project is licensed under the MIT License.