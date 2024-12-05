# MVX Winter Challenges
**Testnet only**  



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

