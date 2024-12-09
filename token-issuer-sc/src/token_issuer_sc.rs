#![no_std]

#[allow(unused_imports)]
use multiversx_sc::imports::*;

/// An empty contract. To be used as a template when starting a new contract from scratch.
#[multiversx_sc::contract]
pub trait TokenIssuerSc {
    #[init]
    fn init(&self) {}

    #[upgrade]
    fn upgrade(&self) {}

    #[storage_mapper("snow_token")]
    fn snow_token(&self) -> FungibleTokenMapper;
    
    #[endpoint(issueTokenSnow)]
    #[payable("EGLD")]
    fn issue_token_snow(&self) {
        require!(self.call_value().egld_value().clone_value() == BigUint::from(50_000_000_000_000_000u64), "Must pay 0.05 EGLD");
        let issue_cost = BigUint::from(50_000_000_000_000_000u64);
        let token_display_name = ManagedBuffer::from("SnowToken");
        let token_ticker = ManagedBuffer::from("SNOW");
        let initial_supply = BigUint::from(10_000_000_000_000_000u64);
        let num_decimals: usize = 8;
        self.snow_token().issue(issue_cost, token_display_name, token_ticker,initial_supply, num_decimals, None);
    }
}
