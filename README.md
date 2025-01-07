# ❄ MultiversX Winter Challenges ❄
**Testnet only**  

You can check proofs of the challenges in `./output`.  
It generally contains both a text file and a json file that store explorer links or data.  
Proofs are named given the date of the challenge.

## 🔧 Installation
Clone
```Bash
git clone https://github.com/VersaliXis/mvx-winter-challenges.git  
```
Create a venv in the repo
```Bash
python -m venv [path to venv] 
```
Activate the venv and install packages
```Bash
source [path to venv]/bin/activate
pip install -r requirements.txt
```
Rust and sc-meta should be installed.   
Follow instructions on [MultiversX docs](https://docs.multiversx.com/sdk-and-tools/troubleshooting/rust-setup/#installing-rust-and-sc-meta)

---
# Contract informations

- ## Resource Issuer contract
    - [docs](https://github.com/VersaliXis/mvx-winter-challenges/blob/main/resource-issuer-sc/README.md)
    - [code](https://github.com/VersaliXis/mvx-winter-challenges/tree/main/resource-issuer-sc)
    - [abi](https://github.com/VersaliXis/mvx-winter-challenges/blob/main/resource-issuer-sc/output/resource-issuer.abi.json)

- ## Token Issuer contract
    - [docs](https://github.com/VersaliXis/mvx-winter-challenges/blob/main/token-issuer-sc/README.md)
    - [code](https://github.com/VersaliXis/mvx-winter-challenges/tree/main/token-issuer-sc)
    - [abi](https://github.com/VersaliXis/mvx-winter-challenges/blob/main/token-issuer-sc/output/token-issuer-sc.abi.json)

- ## Staking contract
    - [docs](https://github.com/VersaliXis/mvx-winter-challenges/blob/main/staking-sc/README.md)
    - [code](https://github.com/VersaliXis/mvx-winter-challenges/tree/main/staking-sc)
    - [abi](https://github.com/VersaliXis/mvx-winter-challenges/blob/main/staking-sc/output/staking-sc.abi.json)

- ## Character(Citizens...) issuer contract
    - [docs](https://github.com/VersaliXis/mvx-winter-challenges/blob/main/character-sc/README.md)
    - [code](https://github.com/VersaliXis/mvx-winter-challenges/tree/main/character-sc)
    - [abi](https://github.com/VersaliXis/mvx-winter-challenges/blob/main/character-sc/output/character-sc.abi.json)

---
# 📅 18 December
- ## Character(Citizens...) Issuer Contracts
### Use 
```Rust
//character-sc.rs
#[init]
fn init(&self, name: ManagedBuffer, ticker: ManagedBuffer) {}
```

### Explanation
This contract allows the owner to issue a character NFT (e.g. CITIZEN).  
It allows anyone to mint a character NFT.

### Userflow
1. The owner creates one `character_sc.rs` contract. 
2. The user ask for character creation by sending 10 `WOOD` and 15 `FOOD`.
3. The user can claim his requested character after 1 hour.

### Proof
- [code](https://github.com/VersaliXis/mvx-winter-challenges/tree/main/character-sc)
- [abi](https://github.com/VersaliXis/mvx-winter-challenges/blob/main/character-sc/output/character-sc.abi.json) 
- you can find sample interactions [here](https://github.com/VersaliXis/mvx-winter-challenges/blob/main/output/18d.md)
---


---
# 📅 16, 17 December
- ## Resource Issuer Contracts
### Use 
```Rust
//resource_issuer.rs
#[init]
fn init(&self, name: ManagedBuffer, ticker: ManagedBuffer) {}
```

### Explanation
This contract allows the owner to issue a resource token.  
It allows also to mint and send to an address.

### Userflow
1. The owner creates one `token_issuer_sc.rs` contract for each resource. 
2. The user stakes at least 1000 `SNOW` tokens.
3. The user can claim each resource if specific cooldown is verified by calling the `claimResourceReward` endpoint.
```Rust
/// Resources id:
/// 0: food
/// 1: gold
/// 2: stone
/// 3: wood
#[endpoint(claimResourceReward)]
    fn claim_resource_reward(&self, winter_token_id: TokenIdentifier, ressource_id: &u8) {}
```
### Proof
- [code](https://github.com/VersaliXis/mvx-winter-challenges/tree/main/staking-sc)
- [abi](https://github.com/VersaliXis/mvx-winter-challenges/blob/main/staking-sc/output/staking-sc.abi.json)   
- you can find sample interactions [here](https://github.com/VersaliXis/mvx-winter-challenges/blob/main/output/16d.md)
---

# 📅 15 December
- ## Transfer Staking Rewards
### Use 
```Rust
//staking-sc.rs
#[endpoint(changeRewardsRecipient)]
fn change_rewards_recipient(&self, staked_token: TokenIdentifier, new_recipient: ManagedAddress) {}
```

### Explanation
The endpoint allows any staker to set another address as reward recipient.   
The staker is still the owner of the staking position, so he is still the only allowed to call `claimRewards` but rewards will be sent the new address.

### Proof
A proof is accessible here: [`./staking-sc/staking-sc.abi.json`](https://github.com/VersaliXis/mvx-winter-challenges/blob/main/staking-sc/output/staking-sc.abi.json)    
You can check sample transactions here: [`./output/15d.md`](https://github.com/VersaliXis/mvx-winter-challenges/blob/main/output/15d.md)

---

# 📅 14 December
- ## Claim SNOW rewards
### Use 
```Rust
// staking_sc.rs
#[endpoint(claimRewards)]
fn claim_rewards(&self, token_id: TokenIdentifier) {}
```

### Explanation
The endpoint allows anyone who have staked a WINTER-xx token to claim daily rewrds in SNOW-xx tokens.  
The `staking_sc.rs` contract uses a proxy to interact with the `token_issuer_sc.rs` contract.

### Userflow
1. The owner creates the `token_issuer_sc.rs` contract
2. The owner issues a new SNOW token (e.g. `SNOW-c9af3e`) by calling `issueTokenSnow`
3. The owner creates the `staking_sc.rs` contract with in argument the address of the issuer contract:
```Rust
// staking_sc.rs
#[init]
fn init(&self, issuer_address: ManagedAddress) {}
```
4. The owner calls `setRewardToken(SNOW-c9af3e)` to set reward token as `SNOW-c9af3e`
```Rust
// staking_sc.rs
#[endpoint(setRewardToken)]
fn set_reward_token(&self, token_id: TokenIdentifier) {}
```
5. The user stakes his WINTER by sending them to the endpoint `stakeTokenWinter(WINTER-xx)` of `staking_sc.rs` contract
6. The user claim his rewards in `SNOW-c9af3e` tokens by calling `claimRewards(WINTER-xx)` of `staking_sc.rs` contract

### Proof
A proof of is accessible here: [`./staking-sc/staking-sc.abi.json`](https://github.com/VersaliXis/mvx-winter-challenges/blob/main/staking-sc/output/staking-sc.abi.json).
You can check sample transactions here: [`./output/14d.md`](https://github.com/VersaliXis/mvx-winter-challenges/blob/main/output/14d.md)

---
# 📅 13 December
- ## Stake WINTER tokens
### Use 
```Rust
#[endpoint(stakeTokenWinter)]
#[payable("*")]
fn stake_token_winter(&self) {}
```

### Explanation
The endpoint allows anyone to stake their WINTER-xx token by sending any amount to this endpoint.
Raises error:
- if no unique esdt payments
- if the user has already staked this token

### Proof
A proof of is accessible here: [`./staking-sc/staking-sc.abi.json`](https://github.com/VersaliXis/mvx-winter-challenges/blob/main/staking-sc/output/staking-sc.abi.json)    
You can check sample transactions here: [`./output/13d.md`](https://github.com/VersaliXis/mvx-winter-challenges/blob/main/output/13d.md)


---
# 📅 12 December
- ## Leaderboard of WINTER holders
### Use 
Activate the venv
```Bash
source [path to venv]/bin/activate
```
Launch the script
```Bash
python3 main.py 12d
```

### Explanation
The script generates a leaderboard for each token ranking top holders.  
It then save the leaderboards as a csv file.

### Proof  
A proof of is accessible here: [`./output/12d.csv`](https://github.com/VersaliXis/mvx-winter-challenges/blob/main/output/12d.csv)  
Leaderboards are stored one on the top of each other.

---
# 📅 11 December
- ## Claim WINTER tokens
### Use 
Activate the venv
```Bash
source [path to venv]/bin/activate
```
Launch the script
```Bash
python3 main.py 11d
```

### Explanation
The scripts queries all issued tokens by the contract.  
Then it call the endpoint  `claimTokens` for each of the 9 most recent tokens with each of the 9 wallets created in challenge #1

### Proof
A proof of is accessible here: [`./output/11d.json`](https://github.com/VersaliXis/mvx-winter-challenges/blob/main/output/11d.json) and [`./output/11d.md`](https://github.com/VersaliXis/mvx-winter-challenges/blob/main/output/11d.md)


---
# 📅 10 December
- ## Claim WINTER tokens
### Use 
```Rust
#[endpoint(claimTokens)]
fn claim_tokens(&self, token: TokenIdentifier) {}
```

### Explanation
The endpoint allows anyone to claim a token issued previously.  
The contracts checks if the token was issued and if its balance is not empty.

### Proof
A proof of is accessible here: [`./token-issuer-sc/output/token-issuer-sc.abi.json`](https://github.com/VersaliXis/mvx-winter-challenges/blob/main/token-issuer-sc/output/token-issuer-sc.abi.json) and [`./output/10d.md`](https://github.com/VersaliXis/mvx-winter-challenges/blob/main/output/10d.md)


---
# 📅 9 December
- ## Store SNOW issued amount
### Use 
```Rust
#[view(getAccountState)]
#[storage_mapper("accountState")]
fn account_state(&self, address: &ManagedAddress) -> SetMapper<IssueDataObj<Self::Api>>;

```

### Explanation
The endpoint allows anyone to query all issued tokens and their initial supply by an address.

### Proof
A proof of is accessible here: [`token_issuer_sc.rs`](https://github.com/VersaliXis/mvx-winter-challenges/blob/ecaf75240c50ef54ef3fafe2584562101ec22628/token-issuer-sc/src/token_issuer_sc.rs#L27)

---
# 📅 8 December
- ## Burn SNOW tokens
### Use 
```Rust
#[endpoint(burnTokenSnow)]
fn burn_token_snow(&self, token: TokenIdentifier, amount: BigUint) {}
```

### Explanation
The smart contract allows any user to burn any SNOW tokens by calling the `burnTokenSnow` endpoint and specifying the amount to burn.     
The contract checks if the token was issued and if the burn amount is less or equal balance.

### Proof
A proof of token burning is accessible here: [`./output/8d.md`](https://github.com/VersaliXis/mvx-winter-challenges/blob/main/output/8d.md)

---
# 📅 7 December
- ## Issue SNOW tokens
### Use 
```Rust
#[endpoint(issueTokenSnow)]
#[payable("EGLD")]
fn issue_token_snow(&self, amount: BigUint) {}
```

### Explanation
The smart contract allows any user to issue a new SNOW token by calling the `issueTokenSnow` endpoint and sending 0.05 EGLD (current cost to issue a token).  
The user chooses initial supply.  
It uses a callback in order to save the issued token identifier in a `SetMapper`

#### Update
Based on Decmber 14 challenge where the contract should be able to mint tokens, I added the following endpoint that should be called manually after issuance.  
Check [this commit](https://github.com/VersaliXis/mvx-winter-challenges/commit/cffc933ac69ddf242ed85eb6207a1e063ab3b5ac) for details.
```Rust
#[endpoint(setLocalRoles)]
fn set_local_roles(&self, token: TokenIdentifier) {}
```
### Proof
A proof of token issuance is accessible here:
[`./output/7d.md`](https://github.com/VersaliXis/mvx-winter-challenges/blob/main/output/7d.md)

---
# 📅 6 December
- ## Query list of txs
### Use 
Activate the venv
```Bash
source [path to venv]/bin/activate
```
Launch the script
```Bash
python3 main.py 6d
```

### Explanation
The script queries the blockchain to retrieve and display the list of transactions for each of the generated account.


### Proof
A proof of token distribution is accessible here:  
[`./output/6d.json`](https://raw.githubusercontent.com/VersaliXis/mvx-winter-challenges/refs/heads/main/output/6d.json) or 
[`./output/6d.txt`](https://github.com/VersaliXis/mvx-winter-challenges/blob/main/output/6d.txt)

---
# 📅 5 December
- ## Transfer WINTER tokens
### Use 
Activate the venv
```Bash
source [path to venv]/bin/activate
```Bash
Launch the script
```Bash
python3 main.py 5d
```

### Explanation
The script uses the pem previously created and filled.   
For each of them, it send 10,000 WINTER-xx tokens to 1,000 random accounts.

### Proof
A proof of token distribution is accessible here:  
[`./output/5d.txt`](https://github.com/VersaliXis/mvx-winter-challenges/blob/main/output/5d.txt) or
[`./output/5d_token_owners.json`](https://github.com/VersaliXis/mvx-winter-challenges/blob/main/output/5d.json)

---
# 📅 4 December
- ## Issue WINTER tokens
### Use 
Activate the venv
```Bash
source [path to venv]/bin/activate
```
Launch the script
```Bash
python3 main.py 4d
```

### Explanation
The script uses the pem previously created and filled.   
For each of them, it issues a token with following specs:
- 100mil supply
- ticker WINTER-xx 
- 8 decimals  

### Proof
A proof of token creation is accessible here:  
[`./output/4d.txt`](https://github.com/VersaliXis/mvx-winter-challenges/blob/main/output/4d.txt) or
[`./output/4d.json`](https://github.com/VersaliXis/mvx-winter-challenges/blob/main/output/4d.json)

---
# 📅 3 December
- ## Create wallets
### Use 
Activate the venv
```Bash
source [path to venv]/bin/activate
```
Launch the script
```Bash
python3 main.py 3d
```

### Explanation
The script creates 3 wallets on each of the three shards.  
It saves their pem in `./wallets/`  
It then calls the API of the faucet `r3d4.fr` to get 1 EGLD on Testnet  

### Proof
A proof of wallet creation and *automatic* fauceting is accessible here:  
[`./output/3d.txt`](https://github.com/VersaliXis/mvx-winter-challenges/blob/main/output/3d.txt) or
[`./output/3d.json`](https://github.com/VersaliXis/mvx-winter-challenges/blob/main/output/3d.json)

