// validator_set.rs
// Manages the set of validators, including selection and rotation.

use std::collections::HashMap;
use crate::stake_manager::StakeManager;
use crate::utilities::crypto_utils;

// Struct representing a validator.
#[derive(Debug, Clone)]
pub struct Validator {
    pub id: String,
    pub stake: u64,
    pub is_active: bool,
    pub last_active_epoch: u64,
}

// ValidatorSet manages the current set of validators.
pub struct ValidatorSet {
    validators: HashMap<String, Validator>,
    stake_manager: StakeManager,
}

impl ValidatorSet {
    /// Initializes a new ValidatorSet.
    pub fn new(stake_manager: StakeManager) -> Self {
        ValidatorSet {
            validators: HashMap::new(),
            stake_manager,
        }
    }

    /// Adds a new validator to the set.
    /// 
    /// # Arguments
    /// * `validator_id` - A unique identifier for the validator.
    /// * `stake` - The amount of stake the validator is putting up.
    pub fn add_validator(&mut self, validator_id: String, stake: u64) {
        let validator = Validator {
            id: validator_id.clone(),
            stake,
            is_active: false,
            last_active_epoch: 0,
        };
        self.stake_manager.add_stake(&validator_id, stake);
        self.validators.insert(validator_id, validator);
    }

    /// Updates the status of a validator.
    /// 
    /// # Arguments
    /// * `validator_id` - The ID of the validator.
    /// * `is_active` - The new active status.
    pub fn update_validator_status(&mut self, validator_id: &str, is_active: bool) {
        if let Some(validator) = self.validators.get_mut(validator_id) {
            validator.is_active = is_active;
        }
    }

    /// Selects validators for the next epoch based on their stake and other criteria.
    pub fn select_validators_for_next_epoch(&self) -> Vec<Validator> {
        // Implement selection logic, possibly involving randomness and stake amount.
        // For example, validators with higher stakes and consistent participation
        // might have a higher chance of being selected.
    }

    /// Rotates validators based on the selection for the new epoch.
    pub fn rotate_validators(&mut self) {
        let selected_validators = self.select_validators_for_next_epoch();
        // Update the validators set based on the selected validators for the new epoch.
    }

    /// Returns the current set of active validators.
    pub fn get_active_validators(&self) -> Vec<&Validator> {
        self.validators.values().filter(|v| v.is_active).collect()
    }

    /// Calculates and distributes rewards to validators based on their participation.
    pub fn distribute_rewards(&self) {
        // Implement reward calculation and distribution logic.
        // Rewards might depend on factors like the amount of stake,
        // the number of blocks proposed/validated, etc.
    }

    /// Removes a validator from the set.
    /// 
    /// # Arguments
    /// * `validator_id` - The ID of the validator to be removed.
    pub fn remove_validator(&mut self, validator_id: &str) {
        self.validators.remove(validator_id);
        self.stake_manager.remove_stake(validator_id);
    }
}
