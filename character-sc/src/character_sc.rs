#![no_std]

use multiversx_sc::imports::*;
multiversx_sc::derive_imports!();

#[type_abi]
#[derive(TopEncode, TopDecode, PartialEq, Debug, NestedEncode, NestedDecode)]
pub struct  NFTDataObj<M: ManagedTypeApi> {
	pub name: ManagedBuffer<M>,
    pub ticker: ManagedBuffer<M>,
}

#[type_abi]
#[derive(TopEncode, TopDecode, PartialEq, Debug, NestedEncode, NestedDecode)]
pub struct  MintDataObj{
	pub block_nonce: u64,
    pub nft_nonce: Option<u64>
}


#[type_abi]
#[derive(TopEncode, TopDecode)]
pub struct Attributes {
    pub creation_timestamp: u64,
}

const NFT_AMOUNT: u32 = 1;
const FOOD_PAYMENT: u32 = 1_500_000_000;
const WOOD_PAYMENT: u32 = 1_000_000_000;
//number of blocks to wait before minter can claim. 1 hour = 600 blocks
const MINT_COOLDOWN: u64 = 600;

#[multiversx_sc::contract]
pub trait Character {   
    ///////// Setup ///////// 
    #[init]
    fn init(&self, name: ManagedBuffer, ticker: ManagedBuffer) {
        self.nft_data().set(NFTDataObj{name: name, ticker: ticker})
    }

    #[upgrade]
    #[only_owner]
    fn upgrade(&self) {}

    ///////// Storage ///////// 
    #[view(getNFTData)]
    #[storage_mapper("NFTData")]
    fn nft_data(&self) -> SingleValueMapper<NFTDataObj<Self::Api>>;

    #[view(getNFTIdentifier)]
    #[storage_mapper("NFTIdentifier")]
    fn nft_token_id(&self) -> SingleValueMapper<TokenIdentifier>;

    #[view(getMintDateForAddress)]
    #[storage_mapper("mintDateForAddress")]
    fn mint_data_for_address(&self, address: &ManagedAddress) -> SingleValueMapper<MintDataObj>;


    

    ///////// Endpoints ///////// 

    /// The caller must send 0.5 EGLD
    #[endpoint(issueCharacterNFT)]
    #[payable("EGLD")]
    #[only_owner]
    fn issue_character_nft(&self) {
        require!(self.nft_token_id().is_empty(), "Token already issued");
        let nft_data = self.nft_data().get();
        let payment_amount = self.call_value().egld_value();
        self.send()
            .esdt_system_sc_proxy()
            .issue_non_fungible(
                payment_amount.clone_value(),
                &nft_data.name,
                &nft_data.ticker,
                NonFungibleTokenProperties {
                    can_freeze: true,
                    can_wipe: true,
                    can_pause: true,
                    can_transfer_create_role: true,
                    can_change_owner: false,
                    can_upgrade: false,
                    can_add_special_roles: true,
                },
            )
            .with_callback(self.callbacks().issue_callback())
            .async_call_and_exit()
    }
    
    #[endpoint(setLocalRoles)]
    #[only_owner]
    fn set_local_roles(&self) {
        self.require_token_issued();

        self.send()
            .esdt_system_sc_proxy()
            .set_special_roles(
                self.blockchain().get_sc_address(),
                self.nft_token_id().get(),
                [EsdtLocalRole::NftCreate][..].iter().cloned(),
            )
            .async_call_and_exit()
    }

    //Allows a user to ask for a character creation
    //The user must not have another NFT creation pending.
    //The user must send 10 WOOD and 15 FOOD
    #[endpoint(createCharacter)]
    #[payable("*")]
    fn create_character(&self) {
        self.require_token_issued();
        
        let [payment_a, payment_b] = self.call_value().multi_esdt();
        self.require_is_required_payment(payment_a.clone(), payment_b.clone());

        let caller = self.blockchain().get_caller();
        require!(self.mint_data_for_address(&caller).is_empty(), "You already have a NFT creation pending. Please claim it before.");
        self.mint_data_for_address(&caller).set(MintDataObj{
            block_nonce: self.blockchain().get_block_nonce(),
            nft_nonce: Option::None
        });
        //It appers we cannot resend tokens on an edst call, so burning should not occure now.
        self.mint_nft_for_address(caller);
    }

    //Allows a user to get his character after 1 hour
    #[endpoint(claimCharacter)]
    fn claim_character(&self) {
        self.require_token_issued();
        let caller = self.blockchain().get_caller();
        require!(!self.mint_data_for_address(&caller).is_empty(), "You must first ask for NFT creation");
        let mint_data = self.mint_data_for_address(&caller).get();
        require!(self.blockchain().get_block_nonce() - mint_data.block_nonce >= MINT_COOLDOWN, "You must wait 1 hour before claiming");
        self.send().direct_esdt(&caller, &self.nft_token_id().get(), Option::expect(mint_data.nft_nonce, "NFT not minted"), &BigUint::from(NFT_AMOUNT));
        //Should burn tokens here
        self.mint_data_for_address(&caller).clear();
    }

    /// Tools ///
    
    fn mint_nft_for_address(&self, address: ManagedAddress) {
        let nft_token_id = self.nft_token_id().get();
        let name = ManagedBuffer::from("Character");
        let royalties = BigUint::zero();
        let amount = BigUint::from(NFT_AMOUNT);
        let attributes = Attributes {
            creation_timestamp: self.blockchain().get_block_timestamp(),
        };
        let mut serialized_attributes = ManagedBuffer::new();
        if let core::result::Result::Err(err) = attributes.top_encode(&mut serialized_attributes) {
            sc_panic!("Attributes encode error: {}", err.message_bytes());
        }
        let uris = ManagedVec::new();
        let attributes_sha256 = self.crypto().sha256(&serialized_attributes);
        let attributes_hash = attributes_sha256.as_managed_buffer();
        let nft_nonce = self.send().esdt_nft_create(
            &nft_token_id,
            &amount,
            &name,
            &royalties,
            attributes_hash,
            &attributes,
            &uris
        );
        let mint_data = self.mint_data_for_address(&address).get();
        let new_mint_data = MintDataObj{
            block_nonce: mint_data.block_nonce,
            nft_nonce: Option::from(nft_nonce)
        };
        self.mint_data_for_address(&address).set(new_mint_data);
    }

    fn require_token_issued(&self) {
        require!(!self.nft_token_id().is_empty(), "Token not issued");
    } 

    fn require_is_required_payment(&self, payment_a:EsdtTokenPayment , payment_b: EsdtTokenPayment){
        let token_a = payment_a.token_identifier;
        let amount_a = payment_a.amount;
        let token_b = payment_b.token_identifier;
        let amount_b = payment_b.amount;
        require!(
            (
                (
                    (
                        self.is_token(&token_a, ManagedBuffer::from("WOOD")) && amount_a == BigUint::from(WOOD_PAYMENT)
                    )
                    || (
                        self.is_token(&token_a, ManagedBuffer::from("FOOD")) && amount_a == BigUint::from(FOOD_PAYMENT)
                    ) 
                )
                &&
                (
                    (
                        self.is_token(&token_b, ManagedBuffer::from("WOOD")) && amount_b == BigUint::from(WOOD_PAYMENT)
                    )
                    || (
                        self.is_token(&token_b, ManagedBuffer::from("FOOD")) && amount_b == BigUint::from(FOOD_PAYMENT)
                    )
                )
            )
            ,"Unvalid payment"
        )

    }

    fn is_token(&self, token: &TokenIdentifier, substring_mb: ManagedBuffer) -> bool{
        let ticker = token.ticker();
        return substring_mb.eq(&ticker);
    }

    /// Callbacks ///
    #[callback]
    fn issue_callback(
        &self,
        #[call_result] result: ManagedAsyncCallResult<EgldOrEsdtTokenIdentifier>,
    ) {
        match result {
            ManagedAsyncCallResult::Ok(token_id) => {
                self.nft_token_id().set(token_id.unwrap_esdt());
            },
            ManagedAsyncCallResult::Err(_) => {
                let returned = self.call_value().egld_or_single_esdt();
                if returned.token_identifier.is_egld() && returned.amount > 0 {
                    self.tx().to(ToCaller).egld(returned.amount).transfer();
                }
            },
        }
    }
}
