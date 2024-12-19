# Token Issuer contract
**Testnet only**  
This set of contracts is used to issue FOOD, GOLD, STONE and WOOD tokens. Code is exactly the same as the `token-issuer-sc`. Replace [Resource] by the any of the one listed above.


This contract allows anyone to issue a [RESSOURCE] token.  
It keeps track of all [RESSOURCE] tokens issued and their respective issuers.  
It allows to claim issued tokens, mint (and send to a specific address) and burn issued tokens.  


## Setup

```Rust
#[init]
fn init(&self) {}
```

## Endpoints

- ### issueToken[Resource]  
Allows to issue a [RESSOURCE]-xx token. Uses a callback to save the token id.   
The caller must send 0.5 EGLD (issue cost).
Usses a callback.
```Rust
#[endpoint(issueToken[Resource])]
#[payable("EGLD")]
fn issue_token_[resource](&self, amount: BigUint) {}
```

- ### setLocalRoles  
Set local permissions to mint and burn specified token.
```Rust
#[endpoint(setLocalRoles)]
fn set_local_roles(&self, token: TokenIdentifier) {}
```

- ### burnToken[Resource]  
Allows to burn a specified amount of [RESSOURCE] tokens.
```Rust
#[endpoint(burnToken[Resource])]
fn burn_token_[resource](&self, token: TokenIdentifier, amount: BigUint) {}
```

- ### mintToken[Resource]  
Allows to mint a specified amount of a [RESSOURCE] token
Permission for local mint should be enabled before by calling `setLocalRoles`.
```Rust
#[endpoint(mintToken[Resource])]
fn mint_token_[resource](&self, token: &TokenIdentifier ,amount: &BigUint) {}
```

- ### mintAndSendToken[Resource]  
Allows to mint a specified amount of a [RESSOURCE] token and send them directly to specified address.
Permission for local mint should be enabled before by calling `setLocalRoles`.  
Uses a callback.
```Rust
#[endpoint(mintAndSendToken[Resource])]
fn mint_and_send_token_[resource](&self, token: &TokenIdentifier, amount: &BigUint, to: &ManagedAddress){}
```


## Storage

- ### issuedTokens
Stores every [RESSOURCE] issued tokens.
```Rust
#[view(getIssuedTokens)]
#[storage_mapper("issuedTokens")]
fn issued_tokens(&self) -> SetMapper<TokenIdentifier>;
```

- ### accountState
Stores every issued tokens by an address, with their initial supply.
```Rust
#[view(getAccountState)]
#[storage_mapper("accountState")]
fn account_state(&self, address: &ManagedAddress) -> SetMapper<IssueDataObj<Self::Api>>;
```
Where `IssueDataObj` is defined as:  
```Rust
#[type_abi]
#[derive(TopEncode, TopDecode, PartialEq, Debug, NestedEncode, NestedDecode)]
pub struct IssueDataObj<M: ManagedTypeApi> {
	pub token: TokenIdentifier<M>,
	pub issued_amount: BigUint<M>
}
```
    

    
