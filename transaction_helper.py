from pathlib import Path
from multiversx_sdk import (TransactionsFactoryConfig, 
    TransferTransactionsFactory, Token, Mnemonic, TokenTransfer, Transaction, 
    TransactionComputer, AccountNonceHolder, ApiNetworkProvider, Address, UserPEM, UserSigner)
from multiversx_sdk.network_providers.config import DefaultPagination


from const import *
import time

def get_address_of_wallet(wallet_path, wallet_id, shard_id):
    path = Path(wallet_path / f"wallet_{wallet_id}_shard_{shard_id}.pem")
    signer = UserSigner.from_pem_file(path)
    pem = UserPEM.from_file(path)
    address = pem.public_key.to_address("erd")
    return address

#Issue 100mil WINTER-xx tokens on each wallets
def issue_token_transaction(wallet_path, wallet_id, shard_id) -> str:

    address = get_address_of_wallet(wallet_path, wallet_id, shard_id)

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
                b"@2386f26fc10000" #10000000000000000 (100 *10**6 *10**8)
                b"@08" #8 decimals
    ) 
    transaction.nonce = nonce_holder.get_nonce_then_increment()
    transaction_computer = TransactionComputer()  
    transaction.signature = signer.sign(transaction_computer.compute_bytes_for_signing(transaction))
    hash = PROVIDER.send_transaction(transaction)
    print(f"Check on explorer {EXPLORER_ADDRESS}transactions/{hash}")
    return f"{EXPLORER_ADDRESS}transactions/{hash}"

#returns: proof (str) is a link on explorer to check token owners
def send_token(wallet_path, wallet_id: int, shard_id: int) -> str:

    address = get_address_of_wallet(wallet_path, wallet_id, shard_id)
    
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
        print(f"{(i+1)*100}/1000")
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
    print(f"Proof: {EXPLORER_ADDRESS}tokens/{token.identifier}/accounts?size=100")
    return f"{EXPLORER_ADDRESS}tokens/{token.identifier}/accounts?size=100"

#returns a list of tx (dicts)
def get_transactions_of_wallet(wallet_path, wallet_id, shard_id):
    pagination = DefaultPagination()
    pagination.size = 1100
    address = get_address_of_wallet(wallet_path, wallet_id, shard_id)
    account_on_network = PROVIDER.get_account(address)
    transactions = PROVIDER.get_account_transactions(address, pagination)
    transaction_list = []

    for tx in transactions:
        transaction_list.append(tx.to_dictionary())
        print(f"nonce: {tx.nonce}: hash {tx.hash}")
    print(f"Total transactions fetched {len(transactions)}")
    return address, transaction_list
