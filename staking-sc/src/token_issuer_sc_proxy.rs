#![no_std]

use multiversx_sc::imports::*;


#[multiversx_sc::proxy]
pub trait TokenIssuerSc {
    #[endpoint(mintAndSendTokenSnow)]
    fn mint_and_send_token_snow(&self, token: &TokenIdentifier, amount: &BigUint, to: &ManagedAddress){}

    #[endpoint(mintAndSendResourceToken)]
    fn mint_and_send_resource_token(&self, to: ManagedAddress, amount: BigUint){}
}