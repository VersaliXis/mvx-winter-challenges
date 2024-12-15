# MVX Winter Challenges
**Testnet only**  

You can check proofs of the challenges in `./output`.  
It generally contains both a text file and a json file that store explorer links or data.  
Proofs are named given the date of the challenge.

## Installation
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
pip install multiversx-sdk
```
Rust and sc-meta should be installed.   
Follow instructions on [MultiversX docs](https://docs.multiversx.com/sdk-and-tools/troubleshooting/rust-setup/#installing-rust-and-sc-meta)


---
# 12 December

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
# 11 December

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
# 10 December

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
# 9 December

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
# 8 December

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
# 7 December

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

### Proof
A proof of token issuance is accessible here:
[`./output/7d.md`](https://github.com/VersaliXis/mvx-winter-challenges/blob/main/output/7d.md)

---
# 6 December

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
# 5 December

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
# 4 December

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
# 3 December

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

