#![no_std]
#[allow(unused_imports)]
use multiversx_sc::imports::*;


#[multiversx_sc::contract]
pub trait TokenIssuerSc:
    multiversx_sc_modules::default_issue_callbacks::DefaultIssueCallbacksModule 
{
    #[init]
    fn init(&self) {}

    #[upgrade]
    fn upgrade(&self) {}

    #[view(getSnowToken)]
    #[storage_mapper("snowToken")]
    fn snow_token(&self) -> FungibleTokenMapper;
    

    #[endpoint(issueTokenSnow)]
    #[payable("EGLD")]
    fn issue_token_snow(&self, amount: BigUint) {
        require!(self.call_value().egld_value().clone_value() == BigUint::from(50_000_000_000_000_000u64), "Must pay 0.05 EGLD");
        let issue_cost = BigUint::from(50_000_000_000_000_000u64);
        let token_display_name = ManagedBuffer::from("SnowToken");
        let token_ticker = ManagedBuffer::from("SNOW");
        let num_decimals = 8;
        let initial_supply = amount;
        self.snow_token().issue(issue_cost,
            token_display_name, 
            token_ticker,
            initial_supply, 
            num_decimals, None);
    }

    #[endpoint(burnTokenSnow)]
    fn burn_token_snow(&self, amount: BigUint) {
        require!(self.snow_token().get_balance() >= amount, "Exceeds held amount");
        self.snow_token().burn(&amount);
    }
}
