#![no_std]

use soroban_sdk::{contract, contractimpl, Env, Symbol, String, contracttype, log};
use soroban_sdk::{contract, contractimpl, Env, Symbol, String, contracttype, log};

// The contract for managing asset leasing on the Stellar blockchain

#[contracttype]
#[derive(Clone)]
pub struct LeaseAgreement {
    pub asset_id: u64,
    pub lessee: String,
    pub lessor: String,
    pub lease_start: u64,
    pub lease_end: u64,
    pub lease_payment: u64,
    pub is_active: bool,
}

#[contracttype]
#[derive(Clone)]
pub struct Asset {
    pub asset_id: u64,
    pub asset_owner: String,
    pub asset_type: String,
    pub asset_description: String,
    pub is_available: bool,
}

const ASSET_BOOK: Symbol = symbol!("asset_book");
const LEASE_BOOK: Symbol = symbol!("lease_book");

#[contract]
pub struct LandRegistryContract;

#[contractimpl]
impl LandRegistryContract {

    // Create a new asset listing
    pub fn create_asset(env: Env, asset_id: u64, owner: String, asset_type: String, asset_description: String) {
        let asset = Asset {
            asset_id,
            asset_owner: owner,
            asset_type,
            asset_description,
            is_available: true,
        };

        env.storage().instance().set(&ASSET_BOOK, &asset);
        log!(&env, "Asset Created with Asset ID: {}", asset_id);
    }

    // Create a lease agreement for an asset
    pub fn create_lease(env: Env, asset_id: u64, lessee: String, lease_start: u64, lease_end: u64, lease_payment: u64) -> bool {
        let asset_key = ASSET_BOOK;
        let mut asset = env.storage().instance().get(&asset_key).unwrap_or(Asset {
            asset_id: 0,
            asset_owner: String::from_str(&env, "Not Found"),
            asset_type: String::from_str(&env, "Not Found"),
            asset_description: String::from_str(&env, "Not Found"),
            is_available: false,
        });

        if asset.is_available {
            // Mark the asset as unavailable for lease
            asset.is_available = false;
            env.storage().instance().set(&asset_key, &asset);

            // Create the lease agreement
            let lease = LeaseAgreement {
                asset_id,
                lessee,
                lessor: asset.asset_owner.clone(),
                lease_start,
                lease_end,
                lease_payment,
                is_active: true,
            };

            // Store the lease agreement
            env.storage().instance().set(&LEASE_BOOK, &lease);
            log!(&env, "Lease Created for Asset ID: {}", asset_id);
            return true;
        }

        false
    }

    // Complete a lease and return the asset
    pub fn complete_lease(env: Env, asset_id: u64) {
        let lease_key = LEASE_BOOK;
        let mut lease = env.storage().instance().get(&lease_key).unwrap_or(LeaseAgreement {
            asset_id: 0,
            lessee: String::from_str(&env, "Not Found"),
            lessor: String::from_str(&env, "Not Found"),
            lease_start: 0,
            lease_end: 0,
            lease_payment: 0,
            is_active: false,
        });

        if lease.is_active {
            // Mark lease as completed
            lease.is_active = false;
            env.storage().instance().set(&lease_key, &lease);

            // Retrieve and update the asset status
            let asset_key = ASSET_BOOK;
            let mut asset = env.storage().instance().get(&asset_key).unwrap_or(Asset {
                asset_id: 0,
                asset_owner: String::from_str(&env, "Not Found"),
                asset_type: String::from_str(&env, "Not Found"),
                asset_description: String::from_str(&env, "Not Found"),
                is_available: false,
            });

            asset.is_available = true; // Mark the asset as available again
            env.storage().instance().set(&asset_key, &asset);

            log!(&env, "Lease Completed and Asset ID: {} is now available", asset_id);
        }
    }

    // View the current status of a lease
    pub fn view_lease(env: Env, asset_id: u64) -> LeaseAgreement {
        env.storage().instance().get(&LEASE_BOOK).unwrap_or(LeaseAgreement {
            asset_id: 0,
            lessee: String::from_str(&env, "Not Found"),
            lessor: String::from_str(&env, "Not Found"),
            lease_start: 0,
            lease_end: 0,
            lease_payment: 0,
            is_active: false,
        })
    }
}