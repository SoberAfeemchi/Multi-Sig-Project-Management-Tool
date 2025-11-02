#![allow(non_snake_case)]
#![no_std]
use soroban_sdk::{contract, contracttype, contractimpl, log, Env, Symbol, String, Address, symbol_short};

// Milestone structure to store milestone details
#[contracttype]
#[derive(Clone)]
pub struct Milestone {
    pub milestone_id: u64,
    pub project_name: String,
    pub description: String,
    pub payment_amount: u64,
    pub approvals_received: u64,
    pub required_approvals: u64,
    pub is_released: bool,
    pub created_at: u64,
}

// Enum for mapping milestone_id to Milestone
#[contracttype]
pub enum MilestoneBook {
    Milestone(u64)
}

// Symbol for tracking total milestone count
const MILESTONE_COUNT: Symbol = symbol_short!("M_COUNT");

#[contract]
pub struct MultiSigProjectContract;

#[contractimpl]
impl MultiSigProjectContract {

    // Function to create a new milestone with payment
    pub fn create_milestone(
        env: Env, 
        project_name: String, 
        description: String, 
        payment_amount: u64,
        required_approvals: u64
    ) -> u64 {
        // Get current milestone count and increment
        let mut milestone_count: u64 = env.storage().instance().get(&MILESTONE_COUNT).unwrap_or(0);
        milestone_count += 1;

        // Get current timestamp
        let timestamp = env.ledger().timestamp();

        // Create new milestone
        let new_milestone = Milestone {
            milestone_id: milestone_count,
            project_name,
            description,
            payment_amount,
            approvals_received: 0,
            required_approvals,
            is_released: false,
            created_at: timestamp,
        };

        // Store the milestone
        env.storage().instance().set(&MilestoneBook::Milestone(milestone_count), &new_milestone);
        
        // Update milestone count
        env.storage().instance().set(&MILESTONE_COUNT, &milestone_count);
        
        // Extend TTL
        env.storage().instance().extend_ttl(5000, 5000);

        log!(&env, "Milestone Created with ID: {}", milestone_count);

        milestone_count
    }

    // Function for stakeholders to approve a milestone
    pub fn approve_milestone(env: Env, milestone_id: u64) {
        let mut milestone = Self::view_milestone(env.clone(), milestone_id);

        // Check if milestone exists and is not already released
        if milestone.milestone_id != 0 && !milestone.is_released {
            milestone.approvals_received += 1;

            // Check if required approvals are met
            if milestone.approvals_received >= milestone.required_approvals {
                milestone.is_released = true;
                log!(&env, "Milestone ID: {} - Payment Released!", milestone_id);
            } else {
                log!(&env, "Milestone ID: {} - Approval received ({}/{})", 
                    milestone_id, milestone.approvals_received, milestone.required_approvals);
            }

            // Update the milestone
            env.storage().instance().set(&MilestoneBook::Milestone(milestone_id), &milestone);
            env.storage().instance().extend_ttl(5000, 5000);

        } else {
            log!(&env, "Milestone not found or payment already released");
            panic!("Milestone not found or payment already released");
        }
    }

    // Function to view a specific milestone
    pub fn view_milestone(env: Env, milestone_id: u64) -> Milestone {
        let key = MilestoneBook::Milestone(milestone_id);
        
        env.storage().instance().get(&key).unwrap_or(Milestone {
            milestone_id: 0,
            project_name: String::from_str(&env, "Not_Found"),
            description: String::from_str(&env, "Not_Found"),
            payment_amount: 0,
            approvals_received: 0,
            required_approvals: 0,
            is_released: false,
            created_at: 0,
        })
    }

    // Function to get total number of milestones
    pub fn get_milestone_count(env: Env) -> u64 {
        env.storage().instance().get(&MILESTONE_COUNT).unwrap_or(0)
    }
}