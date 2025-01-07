# Character issuer contract
**Testnet only**  

This contract allows the owner to issue a character NFT.  
It allows anyone to mint a character NFT.

## Setup
1. The owner must provide a name (e.g. Citizen) and a ticker (e.g. CITIZEN) for the new token.
```Rust
#[init]
fn init(&self, name: ManagedBuffer, ticker: ManagedBuffer) {}
```
2. The owner has to call the `createCharacter` endpoint to issue the NFT.
3. The owner has to call the `setLocalRoles` endpoint to allow the contract to mint new NFTs

## Endpoints

- ### issueCharacterNFT  
Allows to issue the NFT.  
Callable only by the contract owner. 
```Rust
#[endpoint(issueCharacterNFT)]
#[payable("EGLD")]
#[only_owner]
fn issue_character_nft(&self) {}
```

- ### setLocalRoles  
Used to allow the contract to mint new NFTs.  
Callable only by the contract owner. 
```Rust
#[endpoint(setLocalRoles)]
fn set_local_roles(&self){}
```

- ### createCharacter  
Allows a user to ask for a NFT creation.  
This user must not have another NFT creation pending.  
He has to wait 1 hour before claiming.
```Rust
#[endpoint(createCharacter)]
fn create_character(&self){}
```

- ### claimCharacter  
Allows a user to get his character after 1 hour.
```Rust
#[endpoint(claimCharacter)]
fn claim_character(&self){}
```

## Storage

- ### NFTData
Stores the NFT name and ticker.
```Rust
#[view(getNFTData)]
#[storage_mapper("NFTData")]
fn nft_data(&self) -> SingleValueMapper<NFTDataObj<Self::Api>>;
```

Where `NFTDataObj` is:
```Rust
#[type_abi]
#[derive(TopEncode, TopDecode, PartialEq, Debug, NestedEncode, NestedDecode)]
pub struct  NFTDataObj<M: ManagedTypeApi> {
	pub name: ManagedBuffer<M>,
    pub ticker: ManagedBuffer<M>,
}
```

- ### NFTIdentifier
Stores the NFT token identifier. Set by the callback after issuing.
```Rust
#[view(getNFTIdentifier)]
#[storage_mapper("NFTIdentifier")]
fn nft_token_id(&self) -> SingleValueMapper<TokenIdentifier>;
```

- ### mintDateForAddress
Stores the Mint data for a user.
```Rust
#[view(getMintDateForAddress)]
#[storage_mapper("mintDateForAddress")]
fn mint_data_for_address(&self, address: &ManagedAddress) -> SingleValueMapper<MintDataObj>;
```

Where `MintDataObj` is:
```Rust
#[type_abi]
#[derive(TopEncode, TopDecode, PartialEq, Debug, NestedEncode, NestedDecode)]
pub struct  MintDataObj{
	pub block_nonce: u64,
    pub nft_nonce: Option<u64>
}
```
