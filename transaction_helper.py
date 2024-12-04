from pathlib import Path
from multiversx_sdk import Token, TokenTransfer, Transaction, TransactionComputer, AccountNonceHolder, ApiNetworkProvider, Address, UserPEM, UserSigner
from const import *

#Issue 100mil WINTER-xx tokens on each wallets
def issue_token():
    for shard_id in AVAILABLE_SHARDS:
        for wallet_id in range(3):
            path = Path(WALLETS_PATH / f"wallet_{wallet_id}_shard_{shard_id}.pem")
            signer = UserSigner.from_pem_file(path)
            pem = UserPEM.from_file(path)
            address = pem.public_key.to_address("erd")
            account_on_network = PROVIDER.get_account(address)
            nonce_holder = AccountNonceHolder(account_on_network.nonce)
            #Issue 3 tokens for each wallet
            for i in range(3):
                transaction = Transaction(
                    sender = address.to_bech32(),
                    receiver = TOKEN_ISSUER_ADDRESS,
                    value = 50000000000000000,
                    gas_limit = 60000000,
                    chain_id = "T",
                    data=b"issue" + 
                            b"@57696E746572546F6B656E" +#WinterToken
                            b"@57494E544552" +#WINTER
                            b"@8ac7230489e80000" #10000000000000000000 (100 mil *10**8)
                            b"@08" #8 decimals
                ) 

                transaction.nonce = nonce_holder.get_nonce_then_increment()
                transaction_computer = TransactionComputer()  
                transaction.signature = signer.sign(transaction_computer.compute_bytes_for_signing(transaction))
                hash = PROVIDER.send_transaction(transaction)
                print(f"#{i} Transaction hash for {wallet_id} on shard {shard_id}:", hash)
