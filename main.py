from pathlib import Path
import sys
import datetime
import json

from multiversx_sdk import Transaction, TransactionComputer, AccountNonceHolder, ApiNetworkProvider, Mnemonic, Address, UserPEM, UserSigner
from multiversx_sdk.core.address import get_shard_of_pubkey

from use_faucet import request_testnet_EGLD
from transaction_helper import *
from const import *


class ExceededMaxLoopIterationException(Exception):
    "Raised when the loop exceeds maximum iteration"
    pass

class Chanllenges():
    def __init__(self, is_test_run=False):
        self.is_test_run = is_test_run
        suffix = "-test-only" if is_test_run else ""
        self.wallet_path = Path("./wallets" + suffix)
        self.output_path = Path("./output" + suffix)

    
    ### 6 December
    #fetch all signed transactions of each wallets
    def get_all_transactions(self):
        transactions = {}
        for shard_id in AVAILABLE_SHARDS:
            for wallet_id in range(3):
                print(f"\nGet transactions of wallet #{wallet_id} on shard {shard_id}")
                address, transaction_list = get_transactions_of_wallet(self.wallet_path, wallet_id, shard_id)
                transactions[address.to_bech32()] = transaction_list

        with open(Path(self.output_path / "6d_transactions.json"), "w") as f:
            json.dump(transactions, f)
        
        with open(Path(self.output_path / "6d_transactions.txt"), "w") as f:
            f.write(str(datetime.datetime.now())+"\n")
            for address in transactions:
                f.write(f"\n\nAddress {address} total transactions fetched: {len(transactions[address])}\n")
                for tx in transactions[address]:
                    f.write(f"nonce: {tx["nonce"]} tx: {EXPLORER_ADDRESS}transactions/{tx["hash"]}\n")

    ### 5 December     
    def distribute_tokens(self):
        proof_list = []
        for shard_id in AVAILABLE_SHARDS:
            for wallet_id in range(3):
                print(f"\nDistribute tokens from wallet #{wallet_id} on shard {shard_id}")
                proof = send_token(self.wallet_path, wallet_id, shard_id)
                proof_list.append(proof)

        with open(Path(self.output_path / "5d_token_owners.json"), "w") as f:
            json.dump(proof_list, f)
        
        with open(Path(self.output_path / "5d_token_owners.txt"), "w") as f:
            f.write(str(datetime.datetime.now())+"\n")
            for line in proof_list:
                f.write(line+"\n")


    ### 4 December
    def issue_token(self):
        proof_list = []
        for shard_id in AVAILABLE_SHARDS:
            for wallet_id in range(3):
                proof = issue_token_transaction(self.wallet_path, wallet_id, shard_id)
                proof_list.append(proof)

        with open(Path(self.output_path / "4d_transaction_issue_token.json"), "w") as f:
            json.dump(proof_list, f)
        
        with open(Path(self.output_path / "4d_transaction_issue_token.txt"), "w") as f:
            f.write(str(datetime.datetime.now())+"\n")
            for line in proof_list:
                f.write(line+"\n")

    ### 3 December
    #Generate 3 wallets in each of the shards
    def generate_and_fill_wallets(self):
        proof_list = []
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
                    pem.save(self.wallet_path / f"wallet_{wallet_id}_shard_{shard_index}.pem")
                    return address, mnemonic

            raise ExceededMaxLoopIterationException

        for shard_id in AVAILABLE_SHARDS:
            for i in range(3):
                address, mnemonic = new_mnemonic_in_shard(i, shard_id)
                print(f"\n\nGenerated {address} on shard {shard_id}")
                print(f"Check on explorer {EXPLORER_ADDRESS}accounts/{address}")
                request_testnet_EGLD(address)
                proof = f"{EXPLORER_ADDRESS}accounts/{address}"
                proof_list.append(proof)

        with open(Path(self.output_path / "3d_wallet_creation.json"), "w") as f:
            json.dump(proof_list, f)

        with open(Path(self.output_path / "3d_wallet_creation.txt"), "w") as f:
            f.write(str(datetime.datetime.now())+"\n")
            for line in proof_list:
                f.write(line+"\n")

    #to be launched manually after fauceting
    def create_proof_wallet_fauceting(self):
        proof_list = []
        for shard_id in AVAILABLE_SHARDS:
            for wallet_id in range(3):
                address, transaction_list = get_transactions_of_wallet(self.wallet_path, wallet_id, shard_id, start=0 ,size=1)
                proof = f"{EXPLORER_ADDRESS}transactions/{transaction_list[0]["hash"]}"
                proof_list.append(proof)

        with open(Path(self.output_path / "3d_wallet_creation.txt"), "r") as f:
            prev_lines = f.readlines()
            print(prev_lines)
        with open(Path(self.output_path / "3d_wallet_creation.txt"), "w") as f:
            
            f.write(str(datetime.datetime.now())+"\n")

            for line in proof_list:
                f.write(line+"\n")
            for line in prev_lines:
                #\n already in line
                f.write(line)


if __name__ == "__main__":
    
    is_test_run = False
    argv = sys.argv

    if len(argv) not in [2,3]:
        print(f"{datetime.datetime.now()} Usage: python main.py <date (eg 3d for 3 december)> <opt 0 or 1 for is_test_run bool>")
        sys.exit(1)
    if len(argv) == 3:
        if not argv[2] in ["0","1"]:
            print(f"{datetime.datetime.now()} Usage: python main.py <date (eg 3d for 3 december)> <opt 0 or 1 for is_test_run bool>")
            sys.exit(1)
        is_test_run = bool(int(argv[2]))
        if is_test_run:
            print("##########TEST RUN##########")

    chall = Chanllenges(is_test_run)

    
    match argv[1]:
        case "3d":
            print(f"{datetime.datetime.now()} Generating and requesting faucet for three new wallets on shard 0, 1 and 2 on Testnet")
            chall.generate_and_fill_wallets()
        case "3d_proof":
            chall.create_proof_wallet_fauceting()
        case "4d":
            print(f"{datetime.datetime.now()} Issuing 100mil WINTER-xx tokens on each address")
            chall.issue_token()
        case "5d":
            chall.distribute_tokens()
        case "6d":
            chall.get_all_transactions()
        case _:
            print(f"{datetime.datetime.now()} Unrecognized date code: usage date+first letter of month\n eg '3d' for 3 december")
