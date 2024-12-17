#![no_std]
use multiversx_sc::imports::*;
multiversx_sc::derive_imports!();

#[type_abi]
#[derive(TopEncode, TopDecode, PartialEq, Debug, NestedEncode, NestedDecode)]
pub struct IssueDataObj<M: ManagedTypeApi> {
	pub token: TokenIdentifier<M>,
	pub issued_amount: BigUint<M>
}

#[multiversx_sc::contract]
pub trait TokenIssuerSc:
{   
    ///////// Setup ///////// 
    #[init]
    fn init(&self) {}

    #[upgrade]
    #[only_owner]
    fn upgrade(&self) {}

    ///////// Storage ///////// 
    #[view(getIssuedTokens)]
    #[storage_mapper("issuedTokens")]
    // Stores each issued token associeted with the address of the endpoint issueTokenSnow caller
    fn issued_tokens(&self) -> SetMapper<TokenIdentifier>;

    #[view(getAccountState)]
    #[storage_mapper("accountState")]
    // Stores each issued token associeted with the address of the endpoint issueTokenSnow caller
    fn account_state(&self, address: &ManagedAddress) -> SetMapper<IssueDataObj<Self::Api>>;


    ///////// Endpoints ///////// 
    /// Allow a user to claim the total amount of isued tokens
    #[endpoint(claimTokens)]
    fn claim_tokens(&self, token: TokenIdentifier) {
        require!(self.issued_tokens().contains(&token), "Invalid token ID");
        let esdt_id = EgldOrEsdtTokenIdentifier::esdt(token.clone());
        require!(self.blockchain().get_sc_balance(&esdt_id, 0)>0, "Null balance");
        
        let balance = self.blockchain().get_sc_balance(&esdt_id, 0);
        let caller = self.blockchain().get_caller();
        let _ = self.send().direct(&caller, &esdt_id, 0, &balance);
    }

    /// Allows to issue a SNOW-xx token. Uses a callback to save the token id
    #[endpoint(issueTokenSnow)]
    #[payable("EGLD")]
    fn issue_token_snow(&self, amount: BigUint) {
        require!(self.call_value().egld_value().clone_value() == BigUint::from(50_000_000_000_000_000u64), "Must pay 0.05 EGLD");
        let issue_cost = BigUint::from(50_000_000_000_000_000u64);
        let token_display_name = ManagedBuffer::from("SnowToken");
        let token_ticker = ManagedBuffer::from("SNOW");
        let num_decimals: usize = 8;
        let initial_supply = amount;
        let caller = self.blockchain().get_caller();
        let _ = self.send().esdt_system_sc_proxy().issue_fungible(issue_cost,
            token_display_name, 
            token_ticker,
            initial_supply, 
            FungibleTokenProperties {
                    num_decimals: num_decimals,
                    can_freeze: true,
                    can_wipe: true,
                    can_pause: true,
                    can_mint: true,
                    can_burn: true,
                    can_change_owner: true,
                    can_upgrade: true,
                    can_add_special_roles: true,
                },
            ).with_callback(self.callbacks().issue_token_callback(&caller))
            .async_call_and_exit();
    }

    /// Must be called before minting
    #[endpoint(setLocalRoles)]
    #[only_owner]
    fn set_local_roles(&self, token: TokenIdentifier) {
        let roles_iter = [EsdtLocalRole::Mint, EsdtLocalRole::Burn,].into_iter();
        self.send().esdt_system_sc_proxy().set_special_roles(
            self.blockchain().get_sc_address(), 
            token, 
            roles_iter).async_call_and_exit();
    }

    /// Allows to burn any amount of token specified
    #[endpoint(burnTokenSnow)]
    fn burn_token_snow(&self, token: TokenIdentifier, amount: BigUint) {
        require!(self.issued_tokens().contains(&token), "Invalid token ID");
        require!(self.blockchain().get_sc_balance(&EgldOrEsdtTokenIdentifier::esdt(token.clone()), 0) >= amount, "Burn amount exceeds balance");
        self.send().esdt_local_burn(&token, 0u64, &amount);
    }

    /// Allows to mint and send minted tokens to an address
    #[endpoint(mintAndSendTokenSnow)]
    fn mint_and_send_token_snow(&self, token: &TokenIdentifier, amount: &BigUint, to: &ManagedAddress){
        self.mint_token_snow(token, amount);
        self.send().direct_esdt(to, token, 0u64, amount);
    }

    /// Allows to mint a SNOW-xx token
    /// Must have called setLocalRoles before
    #[endpoint(mintTokenSnow)]
    fn mint_token_snow(&self, token: &TokenIdentifier ,amount: &BigUint) {
        require!(self.issued_tokens().contains(token), "You must issue before minting");
        let _ = self.send().esdt_local_mint(token, 0u64, amount);
    }


    ///////// Callbacks ///////// 
    #[callback]
    fn issue_token_callback(
        &self, 
        caller: &ManagedAddress,
        #[call_result] result: ManagedAsyncCallResult<()>
    ) {
        let (token_id, returned_amount) = self.call_value().egld_or_single_fungible_esdt();
        match result {
            ManagedAsyncCallResult::Ok(()) => {
                let token = token_id.unwrap_esdt();
                let issue_data = IssueDataObj{token: token.clone(), issued_amount: returned_amount};
                self.issued_tokens().insert(token.clone());
                self.account_state(&caller).insert(issue_data);
            },
            ManagedAsyncCallResult::Err(_) => {
                // Token returned id EGLD -> issue Failed
                if token_id.is_egld() && returned_amount > 0u64 {
                    self.send().direct_egld(caller, &returned_amount);
                }
            },
        }
    }

    #[callback]
    fn mint_and_send_callback(&self, 
        to: &ManagedAddress,
        #[call_result] result: ManagedAsyncCallResult<()>
    ) {
        let (token, minted_amount) = self.call_value().egld_or_single_fungible_esdt();
        match result {
            ManagedAsyncCallResult::Ok(()) => {
                self.send().direct(to, &token, 0u64, &minted_amount);
            },
            ManagedAsyncCallResult::Err(_) => {
                !panic!("Error while trying to mint and send");
            },
        }
    }
}
