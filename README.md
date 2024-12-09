# MVX Winter Challenges
**Testnet only**  

You can check proofs of the challenges in `./output`.  
It generally contains both a text file and a json file that store explorer links or data.  
Proofs are named given the date of the challenge.

## Installation
Clone
```
git clone https://github.com/VersaliXis/mvx-winter-challenges.git  
```
Create a venv in the repo
```
python -m venv [path to venv] 
```
Activate the venv and install packages
```
source [path to venv]/bin/activate
pip install multiversx-sdk
```
Rust and sc-meta should be installed.   
Follow instructions on [MultiversX docs](https://docs.multiversx.com/sdk-and-tools/troubleshooting/rust-setup/#installing-rust-and-sc-meta)

---
# 7 December

### Use 
```
cd ./token-issuer-sc
sc-meta all build
```

### Explanation
The smart contract allows users to issue SNOW tokens by calling the `issueTokenSnow` endpoint.


---
# 6 December

### Use 
Activate the venv
```
source [path to venv]/bin/activate
```
Launch the script
```
python3 main.py 6d
```

### Explanation
The script queries the blockchain to retrieve and display the list of transactions for each of the generated account.


### Proof
A proof of token distribution is accessible here:  
`./output/6d_transactions.txt`
`./output/6d_transactions.json`

---
# 5 December

### Use 
Activate the venv
```
source [path to venv]/bin/activate
```
Launch the script
```
python3 main.py 5d
```

### Explanation
The script uses the pem previously created and filled.   
For each of them, it send 10,000 WINTER-xx tokens to 1,000 random accounts.

### Proof
A proof of token distribution is accessible here:  
`./output/5d_token_owners.txt`
`./output/5d_token_owners.json`

---
# 4 December

### Use 
Activate the venv
```
source [path to venv]/bin/activate
```
Launch the script
```
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
`./output/4d_transaction_issue_token.txt`
`./output/4d_transaction_issue_token.json`

---
# 3 December

### Use 
Activate the venv
```
source [path to venv]/bin/activate
```
Launch the script
```
python3 main.py 3d
```

### Explanation
The script creates 3 wallets on each of the three shards.  
It saves their pem in `./wallets/`  
It then calls the API of the faucet `r3d4.fr` to get 1 EGLD on Testnet  

### Proof
A proof of wallet creation and *automatic* fauceting is accessible here:  
`./output/3d_wallet_creation.txt`
`./output/3d_wallet_creation.json`

