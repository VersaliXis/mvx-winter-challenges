#![no_std]
use multiversx_sc::imports::*;
multiversx_sc::derive_imports!();

#[type_abi]
#[derive(TopEncode, TopDecode, PartialEq, Debug, NestedEncode, NestedDecode)]
pub struct TokenDataObj<M: ManagedTypeApi> {
	pub name: ManagedBuffer<M>,
    pub ticker: ManagedBuffer<M>,
}

#[multiversx_sc::contract]
pub trait ResourceIssuer: multiversx_sc_modules::default_issue_callbacks::DefaultIssueCallbacksModule
{   
    ///////// Setup ///////// 
    #[init]
    fn init(&self, name: ManagedBuffer, ticker: ManagedBuffer) {
        self.resource_token_data().set(TokenDataObj{
            name: name,
            ticker: ticker,
        });
    }

    #[upgrade]
    #[only_owner]
    fn upgrade(&self) {}

    ///////// Storage ///////// 
    #[view(getResourceToken)]
    #[storage_mapper("resourceToken")]
    fn resource_token(&self) -> FungibleTokenMapper;

    #[view(getResourceTokenData)]
    #[storage_mapper("resourceTokenData")]
    fn resource_token_data(&self) -> SingleValueMapper<TokenDataObj<Self::Api>>;


    ///////// Endpoints ///////// 

    /// The caller must send 0.5 EGLD
    #[endpoint(issueResourceToken)]
    #[payable("EGLD")]
    #[only_owner]
    fn issue_resource_token(&self) {
        require!(self.call_value().egld_value().clone_value() == BigUint::from(50_000_000_000_000_000u64), "Must pay 0.05 EGLD");
        let issue_cost = BigUint::from(50_000_000_000_000_000u64);
        let token_data = self.resource_token_data().get();
        let token_display_name = token_data.name;
        let token_ticker = token_data.ticker;
        let num_decimals: usize = 8;
        self.resource_token().issue_and_set_all_roles(
            issue_cost,
            token_display_name,
            token_ticker,
            num_decimals,
            None
        )
    }


    /// Allows to mint and send minted tokens to an address
    #[endpoint(mintAndSendResourceToken)]
    fn mint_and_send_resource_token(&self, to: ManagedAddress, amount: BigUint){
        self.resource_token().mint_and_send(&to, amount);
    }

}
