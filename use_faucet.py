import os

def request_testnet_EGLD(address: str) -> str:
    payload = '{"formdata":{"network":"T","token":"1","address":"'+ address + '","amount":"1"}}'
    #dirty way to make a request, but requests.post doesn't work in this case
    os.system(f"curl -X POST -d '{payload}' https://api.r3d4.fr/faucet/list")

