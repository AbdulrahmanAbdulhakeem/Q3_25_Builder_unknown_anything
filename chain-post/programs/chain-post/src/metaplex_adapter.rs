use anchor_lang::prelude::*;

/// Matches `mpl_bubblegum::types::MetadataArgsV2`
#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct MetadataArgsV2Local {
    pub name: String,
    pub symbol: String,
    pub uri: String,
    pub seller_fee_basis_points: u16,
    pub primary_sale_happened: bool,
    pub is_mutable: bool,
    pub token_standard: Option<u8>, // You can also use TokenStandardLocal if needed
    pub creators: Vec<CreatorLocal>,
    pub collection: Option<CollectionLocal>,
    pub uses: Option<UsesLocal>,
    pub edition_nonce: Option<u8>,
    pub token_program_version: u8,
    pub token_metadata_program: Option<Pubkey>,
}

/// Matches `mpl_bubblegum::types::Creator`
#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct CreatorLocal {
    pub address: Pubkey,
    pub verified: bool,
    pub share: u8,
}

/// Matches `mpl_bubblegum::types::Collection`
#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct CollectionLocal {
    pub key: Pubkey,
    pub verified: bool,
}

/// Matches `mpl_bubblegum::types::Uses`
#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct UsesLocal {
    pub use_method: u8, // Convert to/from UseMethod enum manually if needed
    pub remaining: u64,
    pub total: u64,
}

/// Matches `mpl_bubblegum::types::TokenStandard` if needed
#[repr(u8)]
#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub enum TokenStandardLocal {
    NonFungible = 0,
    FungibleAsset = 1,
    Fungible = 2,
    NonFungibleEdition = 3,
    ProgrammableNonFungible = 4,
    ProgrammableNonFungibleEdition = 5,
}
