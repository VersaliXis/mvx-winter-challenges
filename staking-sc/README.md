# Staking contract
**Testnet only**  

This contract allow anyone to stake their WINTER token.  
It provides rewards in SNOW tokens. 
It keeps track of the staked amount and the date of staking.

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
- ### setRewardToken  
Called by the owner to set a SNOW-xx as reward token.  
Check if it is a SNOW token.
```Rust
#[endpoint(setRewardToken)]
#[only_owner]
fn set_reward_token(&self, token_id: TokenIdentifier) {}
```
- ### stakeTokenWinter
Allows a user to stake any amount of WINTER-xx token.  
Check if token is a WINTER token.  
Can be called once per token, unless the user has fully unstaked the previous amount.
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
- ### changeRewardsRecipient
Allows any staker to set another address as reward recipient.   
The staker is still the owner of the staking position, so he is still the only allowed to call `claimRewards` but rewards will be sent the new address.
```Rust
#[endpoint(changeRewardsRecipient)]
fn change_rewards_recipient(&self, staked_token: TokenIdentifier, new_recipient: ManagedAddress) {}
```
## Storage

### issuerAddress
Stores the SNOW issuer contract address.  
Used to mint SNOW tokens as reward.
```Rust
#[view(getIssuerAddress)]
#[storage_mapper("issuerAddress")]
fn issuer_address(&self) -> SingleValueMapper<ManagedAddress>;
```

### rewardToken
Stores the SNOW reward token set by contract owner.
```Rust
#[view(getRewardToken)]
#[storage_mapper("rewardToken")]
fn reward_token(&self) -> SingleValueMapper<TokenIdentifier>;
```

### stakingPosition
Stores a staking position, indexed by staker address and token staked.
```Rust
#[view(getStakingPosition)]
#[storage_mapper("stakingPosition")]
fn staking_position(&self, address: &ManagedAddress, token_id: &TokenIdentifier) -> SingleValueMapper<StakingPositionObj<Self::Api>>;
``` 
Where `StakingPositionObj` is defined as:  
`last_interaction_block` is used to calculate rewards.   
It is set to current block nonce on stake position creation or on rewards distribution.
```Rust
#[type_abi]
#[derive(TopEncode, TopDecode, PartialEq, Debug, NestedEncode, NestedDecode)]
pub struct StakingPositionObj<M: ManagedTypeApi> {
	pub staked_amount: BigUint<M>,
    pub last_interaction_block: u64,
}
```
    

    
