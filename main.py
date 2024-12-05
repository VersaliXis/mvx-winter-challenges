from pathlib import Path
import sys
import datetime

from multiversx_sdk import Transaction, TransactionComputer, AccountNonceHolder, ApiNetworkProvider, Mnemonic, Address, UserPEM, UserSigner
from multiversx_sdk.core.address import get_shard_of_pubkey

from use_faucet import request_testnet_EGLD
from transaction_helper import *
from const import *


class ExceededMaxLoopIterationException(Exception):
    "Raised when the loop exceeds maximum iteration"
    pass



#Generate 3 wallets in each of the shards
def generate_and_fill_wallets():
    for shard_id in AVAILABLE_SHARDS:
        for i in range(3):
            address, mnemonic = new_mnemonic_in_shard(i, shard_id)
            print(f"\n\nGenerated {address} on shard {shard_id}")
            print(f"Check on explorer {EXPLORER_ADDRESS}accounts/{address}")
            request_testnet_EGLD(address)

#Create a new mnemonic in a given shard
def new_mnemonic_in_shard(wallet_id: int, shard_index: int) -> (str, Mnemonic):
    for _ in range(MAX_ITERATIONS):
        mnemonic = Mnemonic.generate()
        secret_key = mnemonic.derive_key(0)
        pubkey = mnemonic.derive_key().generate_public_key()
        generated_address_shard = get_shard_of_pubkey(pubkey.buffer, NUMBER_OF_SHARDS)

        if shard_index == generated_address_shard:
            address = Address(pubkey.buffer, "erd").to_bech32()
            pem = UserPEM(label = address, secret_key=secret_key)
            pem.save(WALLETS_PATH / f"wallet_{wallet_id}_shard_{shard_index}.pem")
            return address, mnemonic

    raise ExceededMaxLoopIterationException

if __name__ == "__main__":
    argv = sys.argv
    if len(argv) != 2:
        print(f"{datetime.datetime.now()} Usage: python main.py <date (eg 3d for 3 december)>")
        sys.exit(1)
    match argv[1]:
        case "3d":
            print(f"{datetime.datetime.now()} Generating and requesting faucet for three new wallets on shard 0, 1 and 2 on Testnet")
            generate_and_fill_wallets()
        case "4d":
            print(f"{datetime.datetime.now()} Issuing 100mil WINTER-xx tokens on each address")
            issue_token()
        case _:
            print(f"{datetime.datetime.now()} Unrecognized date code: usage date+first letter of month\n eg '3d' for 3 december")
