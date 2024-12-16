#![no_std]
use multiversx_sc::imports::*;
multiversx_sc::derive_imports!();

#[type_abi]
#[derive(TopEncode, TopDecode, PartialEq, Debug, NestedEncode, NestedDecode)]
pub struct StakingPositionObj<M: ManagedTypeApi> {
	pub staked_amount: BigUint<M>,
    pub stake_block: u64,
}

pub const BLOCKS_IN_YEAR: u64 = 60 * 60 * 24 * 365 / 6;
pub const MAX_PERCENTAGE: u64 = 10_000;

#[multiversx_sc::contract]
pub trait TokenIssuerSc:
{   
    ///////// Setup ///////// 
    #[init]
    fn init(&self) {}

    #[upgrade]
    fn upgrade(&self) {}

    ///////// Storage ///////// 
    /// Stores a set of (token_id, staked_amount) associated to an address
    #[view(getStakingPosition)]
    #[storage_mapper("stakingPosition")]
    // Stores each issued token associeted with the address of the endpoint issueTokenSnow caller
    fn staking_position(&self, address: ManagedAddress, token_id: TokenIdentifier) -> SingleValueMapper<StakingPositionObj<Self::Api>>;

    ///////// Endpoints ///////// 
    /// Allows a user to stake any amount of WINTER-xx token
    /// Can be called once per token, unless the user has fully unstaked the previous amount
    #[endpoint(stakeTokenWinter)]
    #[payable("*")]
    fn stake_token_winter(&self) {
        //raises error if multiple esdt payments
        let payment = self.call_value().single_esdt();
        let token_id = payment.token_identifier;
        let caller = self.blockchain().get_caller();
        let staking_pos = self.staking_position(caller.clone(), token_id.clone());
        
        require!(staking_pos.is_empty(), "You have already staked that token, you must unstake before.");
        let current_block = self.blockchain().get_block_nonce();
        let amount = payment.amount;
        
        let new_staking_pos = StakingPositionObj{stake_block: current_block, staked_amount: amount};
        staking_pos.set(new_staking_pos);
    }

}
