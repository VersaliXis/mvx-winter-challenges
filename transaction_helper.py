from pathlib import Path
from multiversx_sdk import TransactionsFactoryConfig, TransferTransactionsFactory, Token, Mnemonic, TokenTransfer, Transaction, TransactionComputer, AccountNonceHolder, ApiNetworkProvider, Address, UserPEM, UserSigner
from const import *
import time
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
            transaction = Transaction(
                sender = address.to_bech32(),
                receiver = TOKEN_ISSUER_ADDRESS,
                value = 50000000000000000,
                gas_limit = 60000000,
                chain_id = "T",
                data=b"issue" + 
                        b"@57696E746572546F6B656E" +#WinterToken
                        b"@57494E544552" +#WINTER
                        b"@2386f26fc10000" #10000000000000000 (100 mil *10**8)
                        b"@08" #8 decimals
            ) 

            transaction.nonce = nonce_holder.get_nonce_then_increment()
            transaction_computer = TransactionComputer()  
            transaction.signature = signer.sign(transaction_computer.compute_bytes_for_signing(transaction))
            hash = PROVIDER.send_transaction(transaction)
            #print(f"#{i} Transaction hash for {wallet_id} on shard {shard_id}:", hash)
            print(f"Check on explorer {EXPLORER_ADDRESS}transactions/{hash}")

def send_token(wallet_id: int, shard_id: int):
    path = Path(WALLETS_PATH / f"wallet_{wallet_id}_shard_{shard_id}.pem")
    signer = UserSigner.from_pem_file(path)
    pem = UserPEM.from_file(path)
    address = pem.public_key.to_address("erd")
    account_on_network = PROVIDER.get_account(address)
    
    fungible_tokens_on_network = PROVIDER.get_fungible_tokens_of_account(address)
    for token in fungible_tokens_on_network:
        if "WINTER" in token.identifier:
            token_to_send = Token(token.identifier)
    try:
        if token_to_send:
            pass
    except:
        print("error token not found")

    #generate 1000 random addresses and send 10,000 tokens to each
    account_on_network = PROVIDER.get_account(address)
    nonce_holder = AccountNonceHolder(account_on_network.nonce)
    
    for i in range(10):
        tx_list = []
        print(f"{i*100}/1000")
        #need to split because cannot broadcast more than 100tx at a time
        for j in range(100):
            #generate new address
            mnemonic = Mnemonic.generate()
            secret_key = mnemonic.derive_key(0)
            pubkey = mnemonic.derive_key().generate_public_key()
            receiver_address = Address(pubkey.buffer, "erd")

            transfer = TokenTransfer(token_to_send, 10000 *10**8)
            config = TransactionsFactoryConfig(chain_id="T")
            transfer_factory = TransferTransactionsFactory(config=config)
            transaction = transfer_factory.create_transaction_for_esdt_token_transfer(
                sender=address,
                receiver=receiver_address,
                token_transfers=[transfer]
            )   
            transaction_computer = TransactionComputer()  
            transaction.nonce = nonce_holder.get_nonce_then_increment()
            transaction.signature = signer.sign(transaction_computer.compute_bytes_for_signing(transaction))

            tx_list.append(transaction)
            hashes = PROVIDER.send_transactions(tx_list)
    print(f"Distribution done. \n Check on explorer {EXPLORER_ADDRESS}accounts/{address.to_bech32()}")
