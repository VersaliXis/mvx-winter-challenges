#![no_std]
use multiversx_sc::imports::*;
multiversx_sc::derive_imports!();

#[type_abi]
#[derive(TopEncode, TopDecode, PartialEq, Debug, NestedEncode, NestedDecode)]
pub struct StakeDataObj<M: ManagedTypeApi> {
	pub token: TokenIdentifier<M>,
	pub staked_amount: BigUint<M>
}

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
    #[storage_mapper("issuedTokens")]
    // Stores each issued token associeted with the address of the endpoint issueTokenSnow caller
    fn staking_data(&self, address: ManagedAddress) -> SetMapper<StakeDataObj<Self::Api>>;

    ///////// Endpoints ///////// 
    /// Allows a user to stake any amount of WINTER-xx token
    /// Can be called once per token, unless the user has fully unstaked the previous amount
    #[endpoint(stakeTokenWinter)]
    #[payable("*")]
    fn stake_token_winter(&self) {
        //raises error if multiple esdt payments
        let payment = self.call_value().single_esdt();
        let token_id = payment.token_identifier;
        let amount = payment.amount;
        let caller = self.blockchain().get_caller();

        for staked_token in self.staking_data(caller.clone()).iter() {
            let s_token_id = staked_token.token;
            if s_token_id == token_id {
                panic!("You have already staked that token, you must unstake before.");
            }
        }
        let staked_data = StakeDataObj{token: token_id, staked_amount: amount};
        self.staking_data(caller).insert(staked_data);
    }

}
