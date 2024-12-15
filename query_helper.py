from multiversx_sdk import ( Address,ProxyNetworkProvider, QueryRunnerAdapter,
                            SmartContractQueriesController)
from multiversx_sdk.abi import Abi
from pathlib import Path
from const import *
import requests
import time
import csv


### 12 December
#Fetch all WINTER-xx tokens
#return: tokens = [{"identifier": "WINTER-xx", ...}, ...]
def query_winter_tokens():
    tokens = []
    url = f"{PROVIDER_ADDRESS}tokens?size={MAX_SIMPLE_QUERY_NUMBER}&search=WINTER"
    try:
        tokens = requests.get(url, headers={"accept": "application/json"}).json()
        if len(tokens) == MAX_SIMPLE_QUERY_NUMBER:
            print(f"Warning: there is probably more tokens with this name but endpoint doesn't allow more data fecth")
    except:
        for _ in range(5):
            print(f"Error with url {url}.")
            print("Retrying in 1 second")
            time.sleep(1)
    print(f"Found {len(tokens)} WINTER-xx tokens")
    return tokens

#For a given token id, fetch all holders and held value
#return: holders = [{"address": "erd...", "amount": "10000"}, ...]
def query_token_holdings(token_id: str):
    #list of dics containing address and balance
    url = f"{PROVIDER_ADDRESS}tokens/{token_id}/accounts?size={MAX_SIMPLE_QUERY_NUMBER}"
    try:
        holders = requests.get(url, headers={"accept": "application/json"}).json()
        if len(holders) == MAX_SIMPLE_QUERY_NUMBER:
            print(f"Warning: there is probably more holders of this token but endpoint doesn't allow more data fecth")
    except:
        for _ in range(5):
            print(f"Error with url {url}.")
            print("Retrying in 1 second")
            time.sleep(1)

    return holders


        


### 11 December
# Query the endpoint to get all issued tokens 
def query_issued_tokens() -> list:
    contract = Address.new_from_bech32(CONTRACT_ADDRESS)
    query_runner = QueryRunnerAdapter(ProxyNetworkProvider(PROVIDER_ADDRESS))

    abi = Abi.load(Path(ABI_PATH))
    query_controller = SmartContractQueriesController(query_runner, abi)

    #caller = Address.new_from_bech32('erd10ta6mf3dw9znheszffr9kyvxykjs2kag0cagp7z2xdcvq94axnaqwf0vn3')

    data = query_controller.query(
        contract=contract.to_bech32(),
        function="getIssuedTokens",
        arguments=[],
    )
    print(f"Tokens issued: {data[0]}")
    return data[0]

if __name__ == "__main__":
    query_winter_owners()
    

