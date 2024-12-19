#![no_std]
use core::future::ready;

use __wasm__endpoints__::staking_position;
use multiversx_sc::imports::*;
multiversx_sc::derive_imports!();
mod token_issuer_sc_proxy;

#[type_abi]
#[derive(TopEncode, TopDecode, PartialEq, Debug, NestedEncode, NestedDecode)]
pub struct StakingPositionObj<M: ManagedTypeApi> {
	pub staked_amount: BigUint<M>,
    pub last_interaction_block: u64,
    pub rewards_recipient: ManagedAddress<M>,
}

pub const BLOCKS_IN_DAY: u64 = 60 * 60 * 24 / 6;
pub const MIN_BLOCK_BEFORE_CLAIM: u64 = 1;
pub const DAILY_RATE_PERCENTAGE: u64 = 100;
pub const MAX_PERCENTAGE: u64 = 100;

#[multiversx_sc::contract]
pub trait TokenIssuerSc:
{   
    ///////// Proxy /////////
    #[proxy]
    fn token_issuer_sc_proxy(&self, sc_address: ManagedAddress) -> token_issuer_sc_proxy::Proxy<Self::Api>;

    ///////// Setup ///////// 
    #[init]
    fn init(&self, issuer_address: ManagedAddress) {
        self.issuer_address().set(issuer_address);
    }

    
    #[upgrade]
    #[only_owner]
    fn upgrade(&self) {}

    ///////// Storage ///////// 

    #[view(getIssuerAddress)]
    #[storage_mapper("issuerAddress")]
    fn issuer_address(&self) -> SingleValueMapper<ManagedAddress>;

    #[view(getRewardToken)]
    #[storage_mapper("rewardToken")]
    fn reward_token(&self) -> SingleValueMapper<TokenIdentifier>;

    #[view(getStakingPosition)]
    #[storage_mapper("stakingPosition")]
    fn staking_position(&self, address: &ManagedAddress, token_id: &TokenIdentifier) -> SingleValueMapper<StakingPositionObj<Self::Api>>;

    ///////// Endpoints ///////// 
    /// Allows a user to stake any amount of WINTER-xx token
    /// Can be called once per token, unless the user has fully unstaked the previous amount
    #[endpoint(stakeTokenWinter)]
    #[payable("*")]
    fn stake_token_winter(&self) {
        let winter = ManagedBuffer::from("WINTER");
        //raises error if multiple esdt payments
        let payment = self.call_value().single_esdt();
        let token_id = payment.token_identifier;
        require!(self.is_token(token_id.clone(), winter), "Not a WINTER token");
        let caller = self.blockchain().get_caller();
        let staking_pos = self.staking_position(&caller, &token_id);
        let amount = payment.amount;
        let current_block = self.blockchain().get_block_nonce();
        let mut new_stake = amount;

        // If user has already an opened position
        if !staking_pos.is_empty() {
            new_stake += staking_pos.get().staked_amount;
        }
        let new_staking_pos = StakingPositionObj{
            last_interaction_block: current_block, 
            staked_amount: new_stake,
            rewards_recipient: caller
        };
        staking_pos.set(new_staking_pos);
        if !staking_pos.is_empty() {
            // here we should claim for the user his reawrds or at least store pending rewards
            //self.claim_rewards(token_id, OptionalValue::Some(caller));
        }
    }
    
    /// Called to set a SNOW-xx as reward token
    #[endpoint(setRewardToken)]
    #[only_owner]
    fn set_reward_token(&self, token_id: TokenIdentifier) {
        let snow = ManagedBuffer::from("SNOW");
        require!(self.is_token(token_id.clone(), snow), "Not a SNOW token");
        self.reward_token().set(token_id);
    }

    /// Called by a WINTER-xx staker. Must specify on which WINTER token to claim.
    /// Rewards are calculated, and if not null, it calls the endpoint mintAndSend of the SNOW-xx issuer contract 
    #[endpoint(claimRewards)]
    fn claim_rewards(&self, token_id: TokenIdentifier, opt_dest_address: OptionalValue<ManagedAddress>) {
        require!(!self.reward_token().is_empty(), "No reward token set. Use setRewardToken");
        let caller = self.blockchain().get_caller();
        let staking_pos_mapper = self.staking_position(&caller, &token_id);
        require!(!staking_pos_mapper.is_empty(), "You have not staked that token");
        
        let dest_address = match opt_dest_address {
            OptionalValue::Some(address) => address,
            OptionalValue::None => staking_pos_mapper.get().rewards_recipient,
        };

        let staking_pos = staking_pos_mapper.get();
        let last_interaction_block = staking_pos.last_interaction_block;
        let current_block = self.blockchain().get_block_nonce();
        let rewards = self.calculate_rewards(staking_pos);

        require!(current_block - last_interaction_block >= MIN_BLOCK_BEFORE_CLAIM, "You have to wait 1 day before claiming again.");
        self.mint_and_distribute_rewards_async(&rewards, &dest_address);
    }

    /// Only the staker can change rewards recipient
    #[endpoint(changeRewardsRecipient)]
    fn change_rewards_recipient(&self, staked_token: TokenIdentifier, new_recipient: ManagedAddress) {
        let caller = self.blockchain().get_caller();
        let staking_position = self.staking_position(&caller, &staked_token);
        require!(!staking_position.is_empty(), "You have not staked that token");
        let staking_data = staking_position.get();
        let new_staking_position = StakingPositionObj {
            last_interaction_block: staking_data.last_interaction_block, 
            staked_amount: staking_data.staked_amount,
            rewards_recipient: new_recipient
        };
        self.staking_position(&caller, &staked_token).set(new_staking_position);
    }

    fn calculate_rewards(&self, staking_position: StakingPositionObj<Self::Api>) -> BigUint {
        let current_block = self.blockchain().get_block_nonce();
        let block_diff = current_block - staking_position.last_interaction_block;
        if &block_diff <= &0 {
            return BigUint::zero();
        }

        let rewards = staking_position.staked_amount 
        * DAILY_RATE_PERCENTAGE/ MAX_PERCENTAGE * block_diff / BLOCKS_IN_DAY;
        require!(&rewards > &0, "Reward is null. Wait");
        return rewards
    }

    /// Calls by the proxy the SNOW-xx issuer contract
    fn mint_and_distribute_rewards_async(&self, rewards: &BigUint, address: &ManagedAddress) {
        let proxy_address = self.issuer_address().get();
        let mut proxy_instance = self.token_issuer_sc_proxy(proxy_address);
        let rewards_token = self.reward_token().get();
        proxy_instance
                .mint_and_send_token_snow(rewards_token, rewards, address)
                .async_call_and_exit();
    }

    ///////// Tools ///////// 
    fn is_token(&self, token: TokenIdentifier, substring_mb: ManagedBuffer) -> bool{
        let ticker = token.ticker();
        return substring_mb.eq(&ticker);
    }
}
