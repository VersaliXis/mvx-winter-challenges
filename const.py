from pathlib import Path
from multiversx_sdk import ApiNetworkProvider

WALLETS_PATH = Path("./wallets")
OUTPUT_PATH = Path("./output")
PROVIDER_ADDRESS = "https://testnet-api.multiversx.com/"
EXPLORER_ADDRESS = "https://testnet-explorer.multiversx.com/"

NUMBER_OF_SHARDS = 3
AVAILABLE_SHARDS = [0,1,2]
MAX_ITERATIONS = 100
PROVIDER = ApiNetworkProvider(PROVIDER_ADDRESS)
TOKEN_ISSUER_ADDRESS = "erd1qqqqqqqqqqqqqqqpqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzllls8a5w6u"