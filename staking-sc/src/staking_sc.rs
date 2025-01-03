#![no_std]

use multiversx_sc::imports::*;
multiversx_sc::derive_imports!();
mod token_issuer_sc_proxy;

#[type_abi]
#[derive(TopEncode, TopDecode, PartialEq, Debug, NestedEncode, NestedDecode)]
pub struct StakingPositionObj<M: ManagedTypeApi> {
	pub staked_amount: BigUint<M>,
    pub last_interaction_block: u64,
    pub rewards_recipient: ManagedAddress<M>,
    pub last_food_date: u64,
    pub last_gold_date: u64,
    pub last_stone_date: u64,
    pub last_wood_date: u64,
}

pub const BLOCKS_IN_DAY: u64 = 60 * 60 * 24 / 6;
pub const MIN_BLOCK_BEFORE_CLAIM: u64 = BLOCKS_IN_DAY; //1 epoch
pub const DAILY_RATE_PERCENTAGE: u64 = 1;
pub const MAX_PERCENTAGE: u64 = 100;


pub const FOOD_COOLDOWN_ROUNDS: u64 = 1200; //0
pub const GOLD_COOLDOWN_ROUNDS: u64 = 2400; //1
pub const STONE_COOLDOWN_ROUNDS: u64 = 1800; //2
pub const WOOD_COOLDOWN_ROUNDS: u64 = 600; //3




#[multiversx_sc::contract]
pub trait TokenIssuerSc:
{   
    ///////// Proxy /////////
    #[proxy]
    fn token_issuer_proxy(&self, sc_address: ManagedAddress) -> token_issuer_sc_proxy::Proxy<Self::Api>;

    ///////// Setup ///////// 
    #[init]
    fn init(&self, 
        snow_issuer_address: ManagedAddress,
        food_issuer_address: ManagedAddress,
        gold_issuer_address: ManagedAddress,
        stone_issuer_address: ManagedAddress,
        wood_issuer_address: ManagedAddress
    ) {
        self.snow_issuer_address().set(snow_issuer_address);
        self.resources_issuers_addresses(&0u8).set(food_issuer_address);
        self.resources_issuers_addresses(&1u8).set(gold_issuer_address);
        self.resources_issuers_addresses(&2u8).set(stone_issuer_address);
        self.resources_issuers_addresses(&3u8).set(wood_issuer_address);
    }

    
    #[upgrade]
    #[only_owner]
    fn upgrade(&self) {}

    ///////// Storage ///////// 
    #[view(getSnowIssuerAddress)]
    #[storage_mapper("snowIssuerAddress")]
    fn snow_issuer_address(&self) -> SingleValueMapper<ManagedAddress>;

    /// 0:food
    /// 1: gold
    /// 2: stone
    /// 3: wood
    #[view(getResourcesIssuersAddresses)]
    #[storage_mapper("resourcesIssuersAddresses")]
    fn resources_issuers_addresses(&self, issuer_id: &u8) -> SingleValueMapper<ManagedAddress>;

    #[view(getRewardToken)]
    #[storage_mapper("rewardToken")]
    fn reward_token(&self) -> SingleValueMapper<TokenIdentifier>;

    #[view(getStakingPosition)]
    #[storage_mapper("stakingPosition")]
    fn staking_position(&self, address: &ManagedAddress, winter_token_id: &TokenIdentifier) 
        -> SingleValueMapper<StakingPositionObj<Self::Api>>;

    #[view(getResourceStatus)]
    #[storage_mapper("resourceStatus")]
    /// Stores last interaction block
    fn ressource_status(&self, address: &ManagedAddress, winter_token_id: &TokenIdentifier, ressource_id: &u8) 
        -> SingleValueMapper<u64>;

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
            rewards_recipient: caller,
            last_food_date: current_block,
            last_gold_date: current_block,
            last_stone_date: current_block,
            last_wood_date: current_block
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
    fn claim_rewards(&self, token_id: TokenIdentifier) {
        require!(!self.reward_token().is_empty(), "No reward token set. Use setRewardToken");
        let caller = self.blockchain().get_caller();
        let staking_pos_mapper = self.staking_position(&caller, &token_id);
        require!(!staking_pos_mapper.is_empty(), "You have not staked that token");
        
        let dest_address = staking_pos_mapper.get().rewards_recipient;
        let staking_pos = staking_pos_mapper.get();
        let last_interaction_block = staking_pos.last_interaction_block;
        let current_block = self.blockchain().get_block_nonce();
        let rewards = self.calculate_snow_rewards(staking_pos);

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
            rewards_recipient: new_recipient,
            last_food_date: staking_data.last_food_date,
            last_gold_date: staking_data.last_gold_date,
            last_stone_date: staking_data.last_stone_date,
            last_wood_date: staking_data.last_wood_date
        };
        self.staking_position(&caller, &staked_token).set(new_staking_position);
    }

    fn calculate_snow_rewards(&self, staking_position: StakingPositionObj<Self::Api>) -> BigUint {
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

    /// Resources managment ///
    /// Given the resource id and the staked winter id, allows to claim reward.
    /// food: 0
    /// gold: 1
    /// stone: 2
    /// wood: 3
    #[endpoint(claimResourceReward)]
    fn claim_resource_reward(&self, winter_token_id: TokenIdentifier, ressource_id: &u8) {
        let caller = self.blockchain().get_caller();
        let rewards = self.calculate_resource_rewards(caller.clone(), winter_token_id, ressource_id);
        require!(&rewards > &0, "Rewards null. Must wait.");
        
        self.mint_and_distribute_resource_rewards_async(&rewards, &caller, &ressource_id);
    }

    fn calculate_resource_rewards(&self, 
        staker: ManagedAddress, 
        winter_token_id: TokenIdentifier, ressource_id: &u8) -> BigUint
    {
        let current_round = self.blockchain().get_block_nonce();
        
        let staking_position = self.staking_position(&staker, &winter_token_id).get();
        let winter_staked_amount = staking_position.staked_amount;
        require!(&winter_staked_amount > &BigUint::from(100_000_000_000u64), "Must have staked 1000 WINTER at least");
        //let a = 100000000u64;
        //let winter_amount_treshold = BigUint::from(a);
        let mut last_date = 0u64;
        let mut cooldown = 1u64;
        let mut staking_position = self.staking_position(&staker, &winter_token_id).get();

        match ressource_id {
            //Food
            &0u8 => {
                last_date = staking_position.last_food_date;
                staking_position.last_food_date = current_round;
                cooldown = FOOD_COOLDOWN_ROUNDS;
            },
            //Gold
            &1u8 => {
                last_date = staking_position.last_gold_date;
                staking_position.last_gold_date = current_round;
                
                cooldown = GOLD_COOLDOWN_ROUNDS;
            },
            //Stone
            &2u8 => {
                last_date = staking_position.last_stone_date;
                staking_position.last_stone_date = current_round;
                
                cooldown = STONE_COOLDOWN_ROUNDS;
            },
            //Wood
            &3u8 => {
                last_date = staking_position.last_wood_date;
                staking_position.last_wood_date = current_round;
                
                cooldown = WOOD_COOLDOWN_ROUNDS;
            },
            _ => {
                BigUint::zero();
            }
        }
        let date_diff = (current_round - last_date);
        require!(date_diff > cooldown, "You have to wait before claiming that resource");
        winter_staked_amount / BigUint::from(100_000_000_000u64) * date_diff / cooldown
    }
    
    /// Calls by the proxy the SNOW-xx issuer contract
    /// Change ressource issuer _> only 1 token
    fn mint_and_distribute_rewards_async(&self, rewards: &BigUint, address: &ManagedAddress) {
        let proxy_address = self.snow_issuer_address().get();
        let mut proxy_instance = self.token_issuer_proxy(proxy_address);
        let rewards_token = self.reward_token().get();
        proxy_instance
                .mint_and_send_token_snow(rewards_token, rewards, address)
                .async_call_and_exit();
    }

    fn mint_and_distribute_resource_rewards_async(&self, rewards: &BigUint, address: &ManagedAddress, resource_id: &u8) {
        let proxy_address = self.resources_issuers_addresses(resource_id).get();
        let mut proxy_instance = self.token_issuer_proxy(proxy_address);
        proxy_instance
                .mint_and_send_resource_token(address, rewards)
                .async_call_and_exit();
    }

    ///////// Tools ///////// 
    fn is_token(&self, token: TokenIdentifier, substring_mb: ManagedBuffer) -> bool{
        let ticker = token.ticker();
        return substring_mb.eq(&ticker);
    }
}
