# Staking contract
**Testnet only**  

This contract allow anyone to stake their WINTER token.  
It provides rewards in SNOW tokens. 

## Userflow
1. The owner creates the `token_issuer_sc.rs` contract
2. The owner issues a new SNOW token (e.g. `SNOW-c9af3e`) by calling `issueTokenSnow`
3. The owner creates the `staking_sc.rs` contract with in argument the address of the issuer contract
4. The owner calls `setRewardToken(SNOW-c9af3e)` to set reward token as `SNOW-c9af3e`
5. The user stakes his WINTER by sending them to the endpoint `stakeTokenWinter(WINTER-xx)` of `staking_sc.rs` contract
6. The user claim his rewards in `SNOW-c9af3e` tokens by calling `claimRewards(WINTER-xx)` of `staking_sc.rs` contract

## Setup

```Rust
#[init]
fn init(&self, issuer_address: ManagedAddress) {}
```

## Endpoints
- ### set_reward_token  
Called to set a SNOW-xx as reward token
```Rust
#[endpoint(setRewardToken)]
fn set_reward_token(&self, token_id: TokenIdentifier) {}
```
- ### stakeTokenWinter
Allows a user to stake any amount of WINTER-xx token  
Can be called once per token, unless the user has fully unstaked the previous amount
```Rust
#[endpoint(stakeTokenWinter)]
#[payable("*")]
fn stake_token_winter(&self) {}
```
- ### claimRewards
Called by a WINTER-xx staker.   
Must specify on which WINTER token to claim.  
Rewards are calculated, and if not null, it calls the endpoint mintAndSend of the SNOW-xx issuer contract 
```Rust
#[endpoint(claimRewards)]
fn claim_rewards(&self, token_id: TokenIdentifier) {}
```
