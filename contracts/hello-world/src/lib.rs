#![allow(non_snake_case)]
#![no_std]

use soroban_sdk::{contract, contractimpl, Env, Symbol, contracttype, log, symbol_short, String}; // Import String here

// Structure to store information about the lease
#[contracttype]
#[derive(Clone)]
pub struct Lease {
    pub lease_id: u64,        // Unique ID for each lease
    pub asset_id: u64,        // Asset ID being leased
    pub owner: String,        // Asset owner's Stellar account ID
    pub lessee: String,       // Lessee's Stellar account ID
    pub start_time: u64,      // Lease start time
    pub end_time: u64,        // Lease end time
    pub payment_amount: u64,  // Payment amount in Lumens (XLM)
    pub is_active: bool,      // Whether the lease is active or expired
}

// Structure to track all leases status (approved, pending, and expired)
#[contracttype]
#[derive(Clone)]
pub struct LeaseStatus {
    pub active: u64,          // Count of active leases
    pub completed: u64,       // Count of completed leases
    pub expired: u64,         // Count of expired leases
    pub total: u64,           // Total number of leases created
}

// For referencing LeaseStatus
const ALL_LEASES: Symbol = symbol_short!("ALL_LES");  // Shorten symbol name to 9 characters

// For creating unique lease IDs
const LEASE_COUNT: Symbol = symbol_short!("LEA_CNT");  // Shorten symbol name to 9 characters

// Enum for storing lease details in a map
#[contracttype]
pub enum LeaseBook {
    Lease(u64),
}

#[contract]
pub struct LandRegistryContract;

#[contractimpl]
impl LandRegistryContract {

    // Function to create a new lease
    pub fn create_lease(env: Env, asset_id: u64, owner: String, lessee: String, start_time: u64, end_time: u64, payment_amount: u64) -> u64 {
        let mut lease_count: u64 = env.storage().instance().get(&LEASE_COUNT).unwrap_or(0);
        lease_count += 1;

        // Create a new lease instance
        let new_lease = Lease {
            lease_id: lease_count,
            asset_id,
            owner,
            lessee,
            start_time,
            end_time,
            payment_amount,
            is_active: true,
        };

        // Update LeaseStatus data
        let mut all_leases = Self::view_all_lease_status(env.clone());
        all_leases.total += 1;
        all_leases.active += 1;

        // Store new lease data
        env.storage().instance().set(&LeaseBook::Lease(lease_count), &new_lease);
        env.storage().instance().set(&ALL_LEASES, &all_leases);
        env.storage().instance().set(&LEASE_COUNT, &lease_count);

        log!(&env, "Lease Created with Lease-ID: {}", new_lease.lease_id);

        lease_count  // Return the unique lease ID
    }

    // Function to complete a lease (mark as completed)
    pub fn complete_lease(env: Env, lease_id: u64) {
        // Retrieve lease details
        let mut lease = Self::view_lease(env.clone(), lease_id);
        
        // If the lease is still active, complete it
        if lease.is_active {
            lease.is_active = false;
            
            // Update LeaseStatus data
            let mut all_leases = Self::view_all_lease_status(env.clone());
            all_leases.active -= 1;
            all_leases.completed += 1;

            // Store updated lease data
            env.storage().instance().set(&LeaseBook::Lease(lease_id), &lease);
            env.storage().instance().set(&ALL_LEASES, &all_leases);

            log!(&env, "Lease-ID: {}, has been completed", lease_id);
        } else {
            log!(&env, "Lease-ID: {}, is already completed or expired", lease_id);
            // Instead of panicking, it might be better to log and exit gracefully
            return;
        }
    }

    // Function to expire a lease (mark as expired)
    pub fn expire_lease(env: Env, lease_id: u64) {
        // Retrieve lease details
        let mut lease = Self::view_lease(env.clone(), lease_id);
        
        // If the lease is still active, expire it
        if lease.is_active {
            lease.is_active = false;
            
            // Update LeaseStatus data
            let mut all_leases = Self::view_all_lease_status(env.clone());
            all_leases.active -= 1;
            all_leases.expired += 1;

            // Store updated lease data
            env.storage().instance().set(&LeaseBook::Lease(lease_id), &lease);
            env.storage().instance().set(&ALL_LEASES, &all_leases);

            log!(&env, "Lease-ID: {}, has been expired", lease_id);
        } else {
            log!(&env, "Lease-ID: {}, is already expired or completed", lease_id);
            // Instead of panicking, it might be better to log and exit gracefully
            return;
        }
    }

    // Function to view the status of all leases
    pub fn view_all_lease_status(env: Env) -> LeaseStatus {
        env.storage().instance().get(&ALL_LEASES).unwrap_or_else(|| LeaseStatus {
            active: 0,
            completed: 0,
            expired: 0,
            total: 0,
        })
    }

    // Function to view details of a specific lease by lease ID
    pub fn view_lease(env: Env, lease_id: u64) -> Lease {
        let key = LeaseBook::Lease(lease_id);
        env.storage().instance().get(&key).unwrap_or_else(|| Lease {
            lease_id: 0,
            asset_id: 0,
            owner: String::from_str(&env, "Not_Found"),
            lessee: String::from_str(&env, "Not_Found"),
            start_time: 0,
            end_time: 0,
            payment_amount: 0,
            is_active: false,
        })
    }
}
