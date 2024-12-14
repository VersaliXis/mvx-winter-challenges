from multiversx_sdk import ( Address,ProxyNetworkProvider, QueryRunnerAdapter,
                            SmartContractQueriesController)
from multiversx_sdk.abi import Abi
from pathlib import Path
from const import *


def query_issued_tokens():
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
    query_issued_tokens()

