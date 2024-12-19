# Token Issuer contract
**Testnet only**  

This contract allows anyone to issue a SNOW token.  
It keeps track of all SNOW tokens issued and their respective issuers.  
It allows to claim issued tokens, mint (and send to a specific address) and burn issued tokens.  


## Setup

```Rust
#[init]
fn init(&self) {}
```

## Endpoints

- ### issueTokenSnow  
Allows to issue a SNOW-xx token. Uses a callback to save the token id.   
The caller must send 0.5 EGLD (issue cost).
Usses a callback.
```Rust
#[endpoint(issueTokenSnow)]
#[payable("EGLD")]
fn issue_token_snow(&self, amount: BigUint) {}
```

- ### setLocalRoles  
Set local permissions to mint and burn specified token.
```Rust
#[endpoint(setLocalRoles)]
fn set_local_roles(&self, token: TokenIdentifier) {}
```

- ### burnTokenSnow  
Allows to burn a specified amount of SNOW tokens.
```Rust
#[endpoint(burnTokenSnow)]
fn burn_token_snow(&self, token: TokenIdentifier, amount: BigUint) {}
```

- ### mintTokenSnow  
Allows to mint a specified amount of a SNOW token
Permission for local mint should be enabled before by calling `setLocalRoles`.
```Rust
#[endpoint(mintTokenSnow)]
fn mint_token_snow(&self, token: &TokenIdentifier ,amount: &BigUint) {}
```

- ### mintAndSendTokenSnow  
Allows to mint a specified amount of a SNOW token and send them directly to specified address.
Permission for local mint should be enabled before by calling `setLocalRoles`.  
Uses a callback.
```Rust
#[endpoint(mintAndSendTokenSnow)]
fn mint_and_send_token_snow(&self, token: &TokenIdentifier, amount: &BigUint, to: &ManagedAddress){}
```


## Storage

- ### issuedTokens
Stores every SNOW issued tokens.
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
    

    
