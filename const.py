from pathlib import Path
from multiversx_sdk import ApiNetworkProvider

PROVIDER_ADDRESS = "https://testnet-api.multiversx.com/"
EXPLORER_ADDRESS = "https://testnet-explorer.multiversx.com/"

NUMBER_OF_SHARDS = 3
AVAILABLE_SHARDS = [0,1,2]
MAX_ITERATIONS = 100
PROVIDER = ApiNetworkProvider(PROVIDER_ADDRESS)
CONFIG = PROVIDER.get_network_config()
TOKEN_ISSUER_ADDRESS = "erd1qqqqqqqqqqqqqqqpqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzllls8a5w6u"
CONTRACT_ADDRESS = "erd1qqqqqqqqqqqqqpgqq0fwefctjgjq8nq7j67kkl65c845cpk3dpxqlzwk4l"
ABI_PATH = "./token-issuer-sc/output/token-issuer-sc.abi.json"

MAX_SIMPLE_QUERY_NUMBER = 10000