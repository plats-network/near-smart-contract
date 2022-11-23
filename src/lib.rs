use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LazyOption, LookupMap};
use near_sdk::json_types::U128;
use near_sdk::{env, near_bindgen, AccountId, Balance, PanicOnDefault, StorageUsage};
use near_sdk::serde::{Deserialize, Serialize};
pub mod ft_core;
pub mod events;
pub mod metadata;
pub mod storage;
pub mod internal;

use crate::metadata::*;
use crate::events::*;

/// The image URL for the default icon
const DATA_IMAGE_SVG_NEAR_ICON: &str = "data:image/svg+xml;charset=UTF-8,%3c?xml version='1.0' encoding='utf-8'?%3e%3c!-- Generator: Adobe Illustrator 25.3.1, SVG Export Plug-In . SVG Version: 6.00 Build 0) --%3e%3csvg version='1.1' id='Layer_1' xmlns='http://www.w3.org/2000/svg' xmlns:xlink='http://www.w3.org/1999/xlink' x='0px' y='0px' viewBox='0 0 189.6 207.8' style='enable-background:new 0 0 189.6 207.8;' xml:space='preserve'%3e%3cstyle type='text/css'%3e .st0%7bfill:url(%23SVGID_1_);%7d .st1%7bfill:url(%23SVGID_2_);%7d .st2%7bfill:url(%23SVGID_3_);%7d .st3%7bfill:url(%23SVGID_4_);%7d .st4%7bfill:url(%23SVGID_5_);%7d .st5%7bfill:url(%23SVGID_6_);%7d .st6%7bfill:url(%23SVGID_7_);%7d .st7%7bfill:url(%23SVGID_8_);%7d .st8%7bfill:url(%23SVGID_9_);%7d %3c/style%3e%3clinearGradient id='SVGID_1_' gradientUnits='userSpaceOnUse' x1='-3.6176' y1='59.5054' x2='190.7763' y2='55.408' gradientTransform='matrix(1 0 0 -1 0 210)'%3e%3cstop offset='0.5856' style='stop-color:%23BBDEFF'/%3e%3cstop offset='1' style='stop-color:%238EC2FF'/%3e%3c/linearGradient%3e%3cpath class='st0' d='M73.6,174.8c-13.4,0-26.3-2.1-38.2-5.8l54.3,31.3c3.2,1.9,7.3,1.9,10.6,0l75.5-43.4c3.2-1.9,5.3-5.4,5.3-9.1 v-42.7C168,145.4,124.8,174.8,73.6,174.8z'/%3e%3clinearGradient id='SVGID_2_' gradientUnits='userSpaceOnUse' x1='-33.1223' y1='71.8912' x2='148.6496' y2='138.6491' gradientTransform='matrix(1 0 0 -1 0 210)'%3e%3cstop offset='0' style='stop-color:%23FFFFFF'/%3e%3cstop offset='1' style='stop-color:%23A8D6F9'/%3e%3c/linearGradient%3e%3cpath class='st1' d='M175.8,52L100.3,8.7c-3.2-1.9-7.3-1.9-10.6,0L14.2,52c-3.3,1.9-5.3,5.3-5.3,9.1v4.8v82c0,3.7,2,7.2,5.3,9.1 l21.2,12.2c11.9,3.7,24.8,5.8,38.2,5.8c51.2,0,94.3-29.4,107.5-69.6V61.1C181.1,57.3,179.1,53.9,175.8,52z'/%3e%3clinearGradient id='SVGID_3_' gradientUnits='userSpaceOnUse' x1='106.6307' y1='152.2751' x2='-3.8459' y2='153.699' gradientTransform='matrix(1 0 0 -1 0 210)'%3e%3cstop offset='0' style='stop-color:%230D0E19'/%3e%3cstop offset='0.6022' style='stop-color:%2320222A'/%3e%3cstop offset='0.8398' style='stop-color:%233E4151'/%3e%3cstop offset='1' style='stop-color:%234E5266'/%3e%3c/linearGradient%3e%3cpath class='st2' d='M88.8,48.7L59.6,66.5h33.2V48.2h-1.5C90.6,48.1,89.5,48.3,88.8,48.7z'/%3e%3clinearGradient id='SVGID_4_' gradientUnits='userSpaceOnUse' x1='86.0832' y1='156.788' x2='86.4538' y2='156.788' gradientTransform='matrix(1 0 0 -1 0 210)'%3e%3cstop offset='0' style='stop-color:%230D0E19'/%3e%3cstop offset='0.6022' style='stop-color:%2320222A'/%3e%3cstop offset='0.8398' style='stop-color:%233E4151'/%3e%3cstop offset='1' style='stop-color:%234E5266'/%3e%3c/linearGradient%3e%3cpath class='st3' d='M86.5,52.9c-0.1,0.2-0.3,0.4-0.4,0.5C86.2,53.2,86.3,53,86.5,52.9z'/%3e%3clinearGradient id='SVGID_5_' gradientUnits='userSpaceOnUse' x1='146.8914' y1='123.2117' x2='97.9398' y2='117.3296' gradientTransform='matrix(1 0 0 -1 0 210)'%3e%3cstop offset='0' style='stop-color:%230D0E19'/%3e%3cstop offset='0.6022' style='stop-color:%2320222A'/%3e%3cstop offset='0.8398' style='stop-color:%233E4151'/%3e%3cstop offset='1' style='stop-color:%234E5266'/%3e%3c/linearGradient%3e%3cpath class='st4' d='M102.7,91h-8.4v19.3h7.2c0,0,10.8,0.7,21.2-5.5c3.9-2.3,7.2-5.7,10.1-9.9c2.9-4.3,4.3-9.5,4.3-15.5 c0-3.5-0.4-6.7-1.2-9.8C125.8,92.8,105.4,91.1,102.7,91z'/%3e%3clinearGradient id='SVGID_6_' gradientUnits='userSpaceOnUse' x1='56.3861' y1='172.274' x2='135.7448' y2='116.9405' gradientTransform='matrix(1 0 0 -1 0 210)'%3e%3cstop offset='0' style='stop-color:%230D0E19'/%3e%3cstop offset='0.6022' style='stop-color:%2320222A'/%3e%3cstop offset='0.8398' style='stop-color:%233E4151'/%3e%3cstop offset='1' style='stop-color:%234E5266'/%3e%3c/linearGradient%3e%3cpath class='st5' d='M127.9,56.5c-5.3-4.8-13.4-8.3-22.3-8.3h-0.4c-0.1,0-3.2,0-3.2,0l-12.6,0l-1.7,0c-1.7,0-3.6,0.5-4.3,1L53.4,67 h30.2h19.3c3.2,0.4,6,0.6,8.8,3.7c1.6,1.8,2.1,5.4,2.1,9.1s-1,6.5-2.9,8.4c-2,1.9-4.6,2.8-7.9,2.8c2.7,0.2,23.1,1.8,33.1-21.3 C134.5,64.5,131.8,60,127.9,56.5z'/%3e%3clinearGradient id='SVGID_7_' gradientUnits='userSpaceOnUse' x1='65.6885' y1='41.4389' x2='113.432' y2='78.8091' gradientTransform='matrix(1 0 0 -1 0 210)'%3e%3cstop offset='0.6022' style='stop-color:%2320222A'/%3e%3cstop offset='0.8398' style='stop-color:%233E4151'/%3e%3cstop offset='1' style='stop-color:%234E5266'/%3e%3c/linearGradient%3e%3cpath class='st6' d='M83,160.7l31.1-15.2c1.9-1.1,3.1-3.2,3.1-5.4v-15.9L83,140.1V160.7z'/%3e%3clinearGradient id='SVGID_8_' gradientUnits='userSpaceOnUse' x1='67.6145' y1='112.9381' x2='95.3621' y2='107.3494' gradientTransform='matrix(1 0 0 -1 0 210)'%3e%3cstop offset='0.6022' style='stop-color:%2320222A'/%3e%3cstop offset='0.8398' style='stop-color:%233E4151'/%3e%3cstop offset='1' style='stop-color:%234E5266'/%3e%3c/linearGradient%3e%3cpath class='st7' d='M71.4,91.3l-0.6,19.1h27.3l2.3-19.3L71.4,91.3z'/%3e%3clinearGradient id='SVGID_9_' gradientUnits='userSpaceOnUse' x1='79.4873' y1='120.9365' x2='75.1387' y2='45.0547' gradientTransform='matrix(1 0 0 -1 0 210)'%3e%3cstop offset='0' style='stop-color:%230D0E19'/%3e%3cstop offset='0.6022' style='stop-color:%2320222A'/%3e%3cstop offset='0.8398' style='stop-color:%233E4151'/%3e%3cstop offset='1' style='stop-color:%234E5266'/%3e%3c/linearGradient%3e%3cpath class='st8' d='M83.1,110.7c0,0,0-4.8,0-7.3c0-2.6,0.3-11.2,7.5-12c1-0.1,3-0.2,3-0.2H72.5c-4.2,0-7.9,5.4-7.9,9.5v47.3 l18.4,12.8C83.1,160.7,83.1,112.5,83.1,110.7z'/%3e%3c/svg%3e";

/// The specific version of the standard we're using
pub const FT_METADATA_SPEC: &str = "ft-1.0.0";

// Task campaign
pub type TaskId = String;
pub type ClientAccount = AccountId;
pub type UserAccount = AccountId;

#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub struct TaskInfo {
    pub client: ClientAccount,
    pub amount: U128,
}



#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    /// Keep track of each account's balances with total deposit
    pub accounts: LookupMap<AccountId, Balance>,

    /// Total supply of all tokens.
    pub total_supply: Balance,

    /// Task information 
    pub tasks: LookupMap<TaskId, TaskInfo>,

    /// The bytes for the largest possible account ID that can be registered on the contract 
    pub bytes_for_longest_account_id: StorageUsage,

    /// Metadata for the contract itself
    pub metadata: LazyOption<FungibleTokenMetadata>,
}

/// Helper structure for keys of the persistent collections.
#[derive(BorshSerialize)]
pub enum StorageKey {
    Accounts,
    Metadata,
    Tasks,
}

#[near_bindgen]
impl Contract {
    /// Initializes the contract with the given total supply owned by the given `owner_id` with
    /// default metadata (for example purposes only).
    #[init]
    pub fn new_default_meta(total_supply: U128) -> Self {
        // Calls the other function "new: with some default metadata and the owner_id & total supply passed in 
        Self::new(
            env::signer_account_id(),
            total_supply,
            FungibleTokenMetadata {
                spec: FT_METADATA_SPEC.to_string(),
                name: "Plats Network".to_string(),
                symbol: "PLAT".to_string(),
                icon: Some(DATA_IMAGE_SVG_NEAR_ICON.to_string()),
                reference: None,
                reference_hash: None,
                decimals: 18,
            },
        )
    }

    /// Initializes the contract with the given total supply owned by the given `owner_id` with
    /// the given fungible token metadata.
    #[init]
    pub fn new(
        owner_id: AccountId,
        total_supply: U128,
        metadata: FungibleTokenMetadata,
    ) -> Self {
        // Create a variable of type Self with all the fields initialized. 
        let mut this = Self {
            // Set the total supply
            total_supply: total_supply.0,
            // Set the bytes for the longest account ID to 0 temporarily until it's calculated later
            bytes_for_longest_account_id: 0,
            // Storage keys are simply the prefixes used for the collections. This helps avoid data collision
            accounts: LookupMap::new(StorageKey::Accounts.try_to_vec().unwrap()),
            metadata: LazyOption::new(
                StorageKey::Metadata.try_to_vec().unwrap(),
                Some(&metadata),
            ),
            tasks: LookupMap::new(StorageKey::Tasks.try_to_vec().unwrap()),
        };

        // Measure the bytes for the longest account ID and store it in the contract.
        this.measure_bytes_for_longest_account_id();

        // Register the owner's account and set their balance to the total supply.
        this.internal_register_account(&owner_id);
        this.internal_deposit(&owner_id, total_supply.into());
        
        // Emit an event showing that the FTs were minted
        FtMint {
            owner_id: &owner_id,
            amount: &total_supply,
            memo: Some("Initial token supply is minted"),
        }
        .emit();

        // Return the Contract object
        this
    }


    pub fn get_total_deposit(&self, task_id: TaskId) -> u128 {
        let task_info = self
            .tasks
            .get(&task_id)
            .expect("Client should be deposit first");
        task_info.amount.0
    }

}