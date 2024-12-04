from pathlib import Path
from multiversx_sdk import ApiNetworkProvider

WALLETS_PATH = Path("./wallets")
NUMBER_OF_SHARDS = 3
AVAILABLE_SHARDS = [0,1,2]
MAX_ITERATIONS = 100
PROVIDER = ApiNetworkProvider("https://testnet-api.multiversx.com/")
TOKEN_ISSUER_ADDRESS = "erd1qqqqqqqqqqqqqqqpqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzllls8a5w6u"