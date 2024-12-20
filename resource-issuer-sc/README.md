# Resource Issuer contract
**Testnet only**  

This contract allows the owner to issue a resource token.  
It allows also to mint and send to an address.

## Setup
The owner must provide a name (e.g. StoneToken) and a ticker (e.g. STONE) for the new token.
```Rust
#[init]
fn init(&self, name: ManagedBuffer, ticker: ManagedBuffer) {}
```

## Endpoints

- ### issueResourceToken  
Allows to issue the token.  
Callable only by the contract owner. It sets permissions to mint new tokens.
```Rust
#[endpoint(issueResourceToken)]
#[payable("EGLD")]
#[only_owner]
fn issue_resource_token(&self) {}
```

- ### mintAndSendResourceToken  
Allows to mint new tokens and send them to the specified address.
```Rust
#[endpoint(mintAndSendResourceToken)]
fn mint_and_send_resource_token(&self, to: ManagedAddress, amount: BigUint){}
```

## Storage

- ### resourceToken
Stores the token. Gives methodes like `issue`, `mint_and_send`....
```Rust
#[view(getResourceToken)]
#[storage_mapper("resourceToken")]
fn resource_token(&self) -> FungibleTokenMapper;
```

- ### resourceTokenData
Stores the token. Gives methodes like `issue`, `mint_and_send`....
```Rust
#[view(getResourceTokenData)]
#[storage_mapper("resourceTokenData")]
fn resource_token_data(&self) -> SingleValueMapper<TokenDataObj<Self::Api>>;
```

Where `TokenDataObj` is:
```Rust
#[type_abi]
#[derive(TopEncode, TopDecode, PartialEq, Debug, NestedEncode, NestedDecode)]
pub struct TokenDataObj<M: ManagedTypeApi> {
	pub name: ManagedBuffer<M>,
    pub ticker: ManagedBuffer<M>,
}
```
`